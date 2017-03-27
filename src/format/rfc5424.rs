use super::{HeaderFields, FormatHeader, FormatMessage, SyslogFormat, SyslogFormatter, MessageOnly,
            MessageWithKsv};
use serializers::{KsvSerializerQuotedValue, KsvSerializerUnquoted};
use slog::{Record, OwnedKeyValueList, Serialize};
use slog_stream::Format as StreamFormat;
use std::io;
use std::marker::PhantomData;
use syslog::{Facility, Priority};
use time::{FormatTimestamp, OmitTimestamp, Ts3164Local, Ts3164Utc, TsIsoLocal, TsIsoUtc};

// RFC5424 ABNF

// SYSLOG-MSG      = HEADER SP STRUCTURED-DATA [SP MSG]

//       HEADER          = PRI VERSION SP TIMESTAMP SP HOSTNAME
//                         SP APP-NAME SP PROCID SP MSGID
//       PRI             = "<" PRIVAL ">"
//       PRIVAL          = 1*3DIGIT ; range 0 .. 191
//       VERSION         = NONZERO-DIGIT 0*2DIGIT
//       HOSTNAME        = NILVALUE / 1*255PRINTUSASCII

//       APP-NAME        = NILVALUE / 1*48PRINTUSASCII
//       PROCID          = NILVALUE / 1*128PRINTUSASCII
//       MSGID           = NILVALUE / 1*32PRINTUSASCII

//       TIMESTAMP       = NILVALUE / FULL-DATE "T" FULL-TIME
//       FULL-DATE       = DATE-FULLYEAR "-" DATE-MONTH "-" DATE-MDAY
//       DATE-FULLYEAR   = 4DIGIT
//       DATE-MONTH      = 2DIGIT  ; 01-12
//       DATE-MDAY       = 2DIGIT  ; 01-28, 01-29, 01-30, 01-31 based on
//                                 ; month/year
//       FULL-TIME       = PARTIAL-TIME TIME-OFFSET
//       PARTIAL-TIME    = TIME-HOUR ":" TIME-MINUTE ":" TIME-SECOND
//                         [TIME-SECFRAC]
//       TIME-HOUR       = 2DIGIT  ; 00-23
//       TIME-MINUTE     = 2DIGIT  ; 00-59
//       TIME-SECOND     = 2DIGIT  ; 00-59
//       TIME-SECFRAC    = "." 1*6DIGIT
//       TIME-OFFSET     = "Z" / TIME-NUMOFFSET
//       TIME-NUMOFFSET  = ("+" / "-") TIME-HOUR ":" TIME-MINUTE


//       STRUCTURED-DATA = NILVALUE / 1*SD-ELEMENT
//       SD-ELEMENT      = "[" SD-ID *(SP SD-PARAM) "]"
//       SD-PARAM        = PARAM-NAME "=" %d34 PARAM-VALUE %d34
//       SD-ID           = SD-NAME
//       PARAM-NAME      = SD-NAME
//       PARAM-VALUE     = UTF-8-STRING ; characters '"', '\' and
//                                      ; ']' MUST be escaped.
//       SD-NAME         = 1*32PRINTUSASCII
//                         ; except '=', SP, ']', %d34 (")

//       MSG             = MSG-ANY / MSG-UTF8
//       MSG-ANY         = *OCTET ; not starting with BOM
//       MSG-UTF8        = BOM UTF-8-STRING
//       BOM             = %xEF.BB.BF

// Write separator
macro_rules! write_sp { ($io:expr) => ( write!($io, " ") ) }

// Write Rfc5424 NILVALUE
macro_rules! write_nilvalue { ($io:expr) => ( write!($io, "-") ) }

// Write end of message some server implementation need NULL Termination, some need LF
// some need both, so let's send both
macro_rules! write_eom { ($io:expr) => ( write!($io, "\n\0") ) }

/// Rfc5424 Header
#[derive(Debug)]
pub struct Rfc5424<T, F> {
    fields: HeaderFields,
    _timestamp: PhantomData<T>,
    _header_format: PhantomData<F>,
}

impl<T, F> Rfc5424<T, F> {
    fn new(fields: HeaderFields) -> Self {
        Rfc5424::<T, F> {
            fields: fields,
            _timestamp: PhantomData,
            _header_format: PhantomData,
        }
    }
}

/// Rfc5424 header without structured data section
pub struct Rfc5424Short;

/// Rfc5424 header with structured data
pub struct Rfc5424Full;

pub trait Rfc5424Header {}

impl Rfc5424Header for Rfc5424Short {}
impl Rfc5424Header for Rfc5424Full {}

impl<T, F> Rfc5424<T, F>
    where T: FormatTimestamp,
          F: Rfc5424Header
{
    fn format_prioriy(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        let priority = Priority::new(self.fields.facility, record.level().into());
        write!(io, "<{}>1", priority)?;
        Ok(())
    }

    fn format_tag(&self, io: &mut io::Write) -> io::Result<()> {
        match self.fields.process_name {
            Some(ref process_name) => write!(io, "{}[{}]:", process_name, self.fields.pid)?,
            None => write!(io, "[{}]:", self.fields.pid)?,
        }
        Ok(())
    }

    fn format_timestamp(&self, io: &mut io::Write) -> io::Result<()> {
        T::format(io)?;
        Ok(())
    }

    fn format_hostname(&self, io: &mut io::Write) -> io::Result<()> {
        match self.fields.hostname {
            Some(ref hostname) => write!(io, "{}", hostname)?,
            None => write_nilvalue!(io)?,
        }
        Ok(())
    }

    fn format_application(&self, io: &mut io::Write) -> io::Result<()> {
        match self.fields.process_name {
            Some(ref process_name) => write!(io, "{}", process_name)?,
            None => write_nilvalue!(io)?,
        }
        Ok(())
    }

    fn format_pid(&self, io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", self.fields.pid)?;
        Ok(())
    }

    fn format_message_id(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        write!(io, "{}", record.level().to_string())?;
        Ok(())
    }

    fn format_sd_element<'a, I, K, V>(&self,
                                      io: &mut io::Write,
                                      record: &Record,
                                      sd_id: &str,
                                      pairs: I)
                                      -> io::Result<()>
        where I: Iterator<Item = &'a (K, V)>,
              K: 'a,
              V: 'a
    {
        // SD-ELEMENT: []
        write!(io, "{}", "[")?;
        write!(io, "{}", sd_id)?;
        let mut serializer = KsvSerializerQuotedValue::new(io, "=");
        for &(key, value) in pairs {
            serializer.emit_delimiter()?;
            value.serialize(record, key, &mut serializer)?;
        }
        let mut io = serializer.finish();
        write!(io, "{}", "]")?;
        Ok(())
    }

    fn format_header(&self,
                     io: &mut io::Write,
                     record: &Record,
                     logger_values: &OwnedKeyValueList)
                     -> io::Result<()> {

        self.format_prioriy(io, record)?; // Priority: <PRI>VERSION
        write_sp!(io)?; // SP
        self.format_timestamp(io)?; // TIMESTAMP (ISOTIMESTAMP)
        write_sp!(io)?; // SP
        self.format_hostname(io)?; // HOSTNAME
        write_sp!(io)?; // SP
        self.format_application(io)?; // APPLICATION
        write_sp!(io)?; // SP
        self.format_pid(io)?; // PID
        write_sp!(io)?;
        self.format_message_id(io, record); // MESSAGEID
        Ok(())
    }
}

impl<T> FormatHeader for Rfc5424<T, Rfc5424Short>
    where T: FormatTimestamp
{
    type Timestamp = T;

    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        self.format_header(io, record, logger_values)?; // HEADER

        write_nilvalue!(io)?; // NILVALUE

        Ok(())
    }
}

impl<T> FormatHeader for Rfc5424<T, Rfc5424Full>
    where T: FormatTimestamp
{
    type Timestamp = T;

    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        self.format_header(io, record, logger_values)?; // HEADER
        write_sp!(io)?; // SP

        // MESSAGE STRUCTURED_DATA
        self.format_sd_element(io,
                               record,
                               format!("{}@{}", "msg", record.line()),
                               record.values().iter().rev())?;

        self.format_sd_element(io,
                               record,
                               format!("{}@{}", "msg", record.line()),
                               logger_values().iter())?;
        Ok(())
    }
}


/// Rfc5424 message formatter with Ksv serialized data
pub type Rfc5424Ksv<T, F> = SyslogFormatter<Rfc5424<T, F>, MessageWithKsv>;

/// Rfc5424 message formatter with RFC5424 structured data
pub type Rfc5424Native<T, F> = SyslogFormatter<Rfc5424<T, F>, MessageOnly>;

// SyslogFormatter invariants with timestamps

/// Rfc5424, Ksv, Local TZ
pub type Rfc5424KsvTsIsoLocal = Rfc5424Ksv<TsIsoLocal, Rfc5424Short>;

/// Rfc5424, Ksv, UTC
pub type Rfc5424KsvTsIsoUtc = Rfc5424Ksv<TsIsoUtc, Rfc5424Short>;

/// Rfc5424, Local TZ
pub type Rfc5424NativeTsIsoLocal = Rfc5424Native<TsIsoLocal, Rfc5424Full>;

/// Rfc5424, UTC
pub type Rfc5424NativeTsIsoUtc = Rfc5424Native<TsIsoUtc, Rfc5424Full>;
