/// Full formatting with support for RFC3164 and RFC5424
pub struct Format {
    mode: FormatMode,
    fn_timestamp: Box<TimestampFn>,
    hostname: Option<String>,
    process_name: Option<String>,
    pid: i32,
    facility: Facility,
}

impl Format {
    pub fn new(mode: FormatMode,
               fn_timestamp: Box<TimestampFn>,
               hostname: Option<String>,
               process_name: Option<String>,
               pid: i32,
               facility: Facility)
               -> Self {
        Format {
            mode: mode,
            fn_timestamp: fn_timestamp,
            hostname: hostname,
            process_name: process_name,
            pid: pid,
            facility: facility,
        }
    }

    /// Format priority
    fn fmt_priority(&self,
                    io: &mut io::Write,
                    f: &Fn(&mut io::Write) -> io::Result<()>)
                    -> io::Result<()> {
        write!(io, "<")?;
        f(io);
        write!(io, ">")?;
        Ok(())
    }

    /// Format a field
    fn fmt_msg(&self,
               io: &mut io::Write,
               f: &Fn(&mut io::Write) -> io::Result<()>)
               -> io::Result<()> {
        f(io)
    }


    /// Format a key
    fn fmt_key(&self,
               io: &mut io::Write,
               f: &Fn(&mut io::Write) -> io::Result<()>)
               -> io::Result<()> {
        f(io)
    }
    /// Format a separator
    fn fmt_separator(&self,
                     io: &mut io::Write,
                     f: &Fn(&mut io::Write) -> io::Result<()>)
                     -> io::Result<()> {
        f(io)
    }
    /// Format a value
    fn fmt_value(&self,
                 io: &mut io::Write,
                 f: &Fn(&mut io::Write) -> io::Result<()>)
                 -> io::Result<()> {
        f(io)
    }
    /// Format a timestamp
    fn fmt_timestamp(&self,
                     io: &mut io::Write,
                     f: &Fn(&mut io::Write) -> io::Result<()>)
                     -> io::Result<()> {
        f(io)
    }

    fn format_rfc5424(&self,
                      io: &mut io::Write,
                      record: &Record,
                      logger_values: &OwnedKeyValueList)
                      -> io::Result<()> {

        /// format RFC 5424 structured data as `([id (name="value")*])*`
        /// / pub fn format_5424_structured_data(&self, data: StructuredData) -> String {
        /// /if data.is_empty() {
        /// /"-".to_string()
        /// /} else {
        /// /let mut res = String::new();
        /// /for (id, params) in data.iter() {
        /// /res = res + "["+id;
        /// /for (name,value) in params.iter() {
        /// /res = res + " " + name + "=\"" + value + "\"";
        /// /}
        /// /res = res + "]";
        /// /}
        /// /
        /// /res
        /// /}
        /// /}
        ///
        /// // format a message as a RFC 5424 log message
        /// pub fn
        /// format_5424<T: fmt::Display>(
        /// &self, severity:Severity,
        /// message_id: i32, data:
        /// StructuredData, message: T) -> String {
        /// let f =  format!("<{}> {} {} {} {} {} {} {} {}",
        /// self.encode_priority(severity, self.facility),
        /// 1, // version
        /// time::now_utc().rfc3339(),
        /// self.hostname.as_ref().map(|x| &x[..]).unwrap_or("localhost"),
        /// self.process, self.pid, message_id,
        /// self.format_5424_structured_data(data), message);
        /// return f;
        ///
        ///
        ///        let mut comma_needed = try!(self.print_msg_header(io,  record));
        let mut serializer = Serializer::new(io);

        for &(k, v) in record.values().iter().rev() {
            v.serialize(record, k, &mut serializer)?;
        }

        for (k, v) in logger_values.iter() {
            v.serialize(record, k, &mut serializer)?;
        }

        let mut io = serializer.finish();

        write!(io, "\n")?;

        Ok(())
    }

    fn format_rfc3164(&self,
                      io: &mut io::Write,
                      record: &Record,
                      logger_values: &OwnedKeyValueList)
                      -> io::Result<()> {

        self.fmt_priority(io,
                          &|io: &mut io::Write| {
                              write!(io,
                                     "{}",
                                     Priority::new(self.facility, record.level().into()))
                          })?;
        self.fmt_separator(io, &|io: &mut io::Write| write!(io, " "))?;
        self.fmt_timestamp(io, &*self.fn_timestamp)?;
        self.fmt_separator(io, &|io: &mut io::Write| write!(io, " "))?;
        match self.hostname {
            Some(ref hostname) => {
                self.fmt_msg(io, &|io: &mut io::Write| write!(io, "{}", hostname))?;
                self.fmt_separator(io, &|io: &mut io::Write| write!(io, " "))?;
            }
            None => {}
        };
        match self.process_name {
            Some(ref process_name) => {
                self.fmt_msg(io,
                             &|io: &mut io::Write| write!(io, "{}[{}]:", process_name, self.pid))?;
            }
            None => {
                self.fmt_msg(io, &|io: &mut io::Write| write!(io, "[{}]:", self.pid))?;
            }
        };
        self.fmt_separator(io, &|io: &mut io::Write| write!(io, " "))?;
        self.fmt_msg(io, &|io: &mut io::Write| write!(io, "{}", record.msg()))?;
        self.fmt_separator(io, &|io: &mut io::Write| write!(io, " "))?;

        let mut serializer = Serializer::new(io);

        for &(k, v) in record.values().iter().rev() {
            v.serialize(record, k, &mut serializer)?;
        }

        for (k, v) in logger_values.iter() {
            v.serialize(record, k, &mut serializer)?;
        }

        let mut io = serializer.finish();

        write!(io, "\n")?;

        Ok(())

    }
}


impl StreamFormat for Format {
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        match self.mode {
            FormatMode::RFC3164 => self.format_rfc3164(io, record, logger_values),
            FormatMode::RFC5424 => self.format_rfc5424(io, record, logger_values),
        }
    }
}
