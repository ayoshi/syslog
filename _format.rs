/// Full formatting with support for RFC3164 and RFC5424
pub struct Format {
    mode: FormatMode,
    fn_timestamp: Box<TimestampFn>,
    hostname: String,
    process_name: String,
    pid: i32,
    facility: Facility
}

impl Format {
    pub fn new<S>(mode: FormatMode,
               fn_timestamp: Box<TimestampFn>,
               hostname: S,
               process_name: S,
               pid: i32,
               facility: Facility)
               -> Self
    where S: Into<String>
    {
        Format {
            mode: mode,
            fn_timestamp: fn_timestamp,
            hostname: hostname.into(),
            process_name: process_name.into(),
            pid: pid,
            facility: facility
        }
    }

    /// Format priority
    fn fmt_priority(&self,
               io: &mut io::Write,
               f: &Fn(&mut io::Write) -> io::Result<()>)
               -> io::Result<()> {
        try!(write!(io, "<"));
        f(io);
        try!(write!(io, ">"));
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
            try!(v.serialize(record, k, &mut serializer));
        }

        for (k, v) in logger_values.iter() {
            try!(v.serialize(record, k, &mut serializer));
        }

        let mut io = serializer.finish();

        try!(write!(io, "\n"));

        Ok(())
    }

    fn format_rfc3164(&self,
                      io: &mut io::Write,
                      record: &Record,
                      logger_values: &OwnedKeyValueList)
                      -> io::Result<()> {

//        match self.hostname {
//            Some(hostname) => {
//                let msg = format!(
//                   "{priority} {timestamp} {hostname} {tag} {msg}",
//                    priority=Priority::new(self.facility, record.level().into()),
//                    timestamp=self.fn_timestamp(),
//                    hostname=hostname,
//                    tag=" ",
//                    msg=&record.msg()
//                );
//            }
//            None => {
//                let format = "{priority} {timestamp} {tag} {msg}>";
//            }
//        };

        //        pub fn
        // format_3164<T: fmt::Display>(&self, severity:Severity, message: T) -> String {
        //        if let Some(ref hostname) = self.hostname {
        //        format!("<{}>{} {} {}[{}]: {}",
        //        self.encode_priority(severity, self.facility),
        //        time::now().strftime("%b %d %T").unwrap(),
        //        hostname, self.process, self.pid, message)
        //        } else {
        //        format!("<{}>{} {}[{}]: {}",
        //        self.encode_priority(severity, self.facility),
        //        time::now().strftime("%b %d %T").unwrap(),
        //        self.process, self.pid, message)
        //        }
        //        }

        //        let msg = concat!(&record.msg, );

        try!(self.fmt_priority(
            io,
            &|io: &mut io::Write| write!(io,"{}", Priority::new(self.facility, record.level().into())))
        );
        try!(self.fmt_separator(io, &|io: &mut io::Write| write!(io, " ")));
        try!(self.fmt_timestamp(io, &*self.fn_timestamp));
        try!(self.fmt_separator(io, &|io: &mut io::Write| write!(io, " ")));
//        try!(self.fmt_tag(io, &|io: &mut io::Write| write!(io, " ")));
        try!(self.fmt_msg(io, &|io: &mut io::Write| write!(io, "{}", record.msg())));
        try!(self.fmt_separator(io, &|io: &mut io::Write| write!(io, " ")));

        let mut serializer = Serializer::new(io);

        for &(k, v) in record.values().iter().rev() {
            try!(v.serialize(record, k, &mut serializer));
        }

        for (k, v) in logger_values.iter() {
            try!(v.serialize(record, k, &mut serializer));
        }

        let mut io = serializer.finish();

        try!(write!(io, "\n"));

        Ok(())

    }
}

struct Serializer<W> {
    io: W,
}

impl<W: io::Write> Serializer<W> {
    fn new(io: W) -> Self {
        Serializer { io: io }
    }

    fn print_comma(&mut self) -> io::Result<()> {
        try!(write!(self.io, ", "));
        Ok(())
    }

    fn finish(self) -> W {
        self.io
    }
}

macro_rules! s(
    ($s:expr, $k:expr, $v:expr) => {
        try!(write!($s.io, "{}", $k));
        try!(write!($s.io, "="));
        try!(write!($s.io, "{}", $v));
    };
);


impl<W: io::Write> slog::ser::Serializer for Serializer<W> {
    fn emit_none(&mut self, key: &str) -> ser::Result {
        s!(self, key, "None");
        Ok(())
    }
    fn emit_unit(&mut self, key: &str) -> ser::Result {
        s!(self, key, "()");
        Ok(())
    }
    fn emit_bool(&mut self, key: &str, val: bool) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_char(&mut self, key: &str, val: char) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_usize(&mut self, key: &str, val: usize) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_isize(&mut self, key: &str, val: isize) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_u8(&mut self, key: &str, val: u8) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_i8(&mut self, key: &str, val: i8) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_u16(&mut self, key: &str, val: u16) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_i16(&mut self, key: &str, val: i16) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_u32(&mut self, key: &str, val: u32) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_i32(&mut self, key: &str, val: i32) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_f32(&mut self, key: &str, val: f32) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_u64(&mut self, key: &str, val: u64) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_i64(&mut self, key: &str, val: i64) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_f64(&mut self, key: &str, val: f64) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_str(&mut self, key: &str, val: &str) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_arguments(&mut self, key: &str, val: &fmt::Arguments) -> ser::Result {
        s!(self, key, val);
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


