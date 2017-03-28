use super::{HeaderFields, FormatHeader, FormatMessage, SyslogFormat, SyslogFormatter, MessageWithKsv};
use serializers::{KsvSerializerQuotedValue, KsvSerializerUnquoted};
use slog::{Record, OwnedKeyValueList};
use slog_stream::Format as StreamFormat;
use std::io;
use std::marker::PhantomData;
use syslog::{Facility, Priority};
use time::{FormatTimestamp, OmitTimestamp, Ts3164Local, Ts3164Utc, TsIsoLocal, TsIsoUtc};

// Write separator
macro_rules! write_sp { ($io:expr) => ( write!($io, " ") ) }

// Write Rfc5424 NILVALUE
macro_rules! write_nilvalue { ($io:expr) => ( write!($io, "-") ) }

// Write end of message some server implementation
// need NULL Termination, some need LF
// some need both, so let's send both
macro_rules! write_eom { ($io:expr) => ( write!($io, "\n\0") ) }

/// RFC3164 RFC3164Short header (PRIORITY HOSTNAME TAG)
pub struct Rfc3164Short;

/// RFC3164 RFC3164Full header (PRIORITY TIMESTAMP TAG)
pub struct Rfc3164Full;

/// RFC3164 Header Format
pub trait Rfc3164Header {}

impl Rfc3164Header for Rfc3164Short {}
impl Rfc3164Header for Rfc3164Full {}


/// RFC3164 Header
#[derive(Debug)]
pub struct Rfc3164<T, F>
    where T: FormatTimestamp,
          F: Rfc3164Header
{
    fields: HeaderFields,
    _timestamp: PhantomData<T>,
    _header_format: PhantomData<F>,
}

impl<T, F> Rfc3164<T, F>
    where T: FormatTimestamp,
          F: Rfc3164Header
{

    fn format_prioriy(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        let priority = Priority::new(self.fields.facility, record.level().into());
        write!(io, "<{}>", priority)?;
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
        if let Some(ref hostname) = self.fields.hostname {
            write!(io, "{}", hostname)?
        }
        Ok(())
    }

}

impl FormatHeader for Rfc3164<OmitTimestamp, Rfc3164Short> {
    type Timestamp = OmitTimestamp;

    fn new(fields: HeaderFields) -> Self {
        Rfc3164::<OmitTimestamp, Rfc3164Short> {
            fields: fields,
            _timestamp: PhantomData,
            _header_format: PhantomData,
        }
    }

    #[allow(unused_variables)]
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        // PRIORITY: <PRI>
        self.format_prioriy(io, record)?;

        // Should we add separator? Rfc specifies that separatoer
        // will be added if there is no timestamp/host field
        write_sp!(io)?;

        // TAG: process_name[pid]:
        self.format_tag(io)?;

        Ok(())
    }
}

impl<T> FormatHeader for Rfc3164<T, Rfc3164Full>
    where T: FormatTimestamp
{
    type Timestamp = T;

    fn new(fields: HeaderFields) -> Self {
        Rfc3164::<T, Rfc3164Full> {
            fields: fields,
            _timestamp: PhantomData,
            _header_format: PhantomData,
        }
    }

    #[allow(unused_variables)]
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        // PRIORITY: <PRI>
        self.format_prioriy(io, record)?;

        // TIMESTAMP
        self.format_timestamp(io)?;

        // Should we add separator? Rfc specifies that separatoer
        // will be added if there is no timestamp/host field
        write_sp!(io)?;

        // HOSTNAME
        self.format_hostname(io)?;

        write_sp!(io)?;

        // TAG: process_name[pid]:
        self.format_tag(io)?;

        Ok(())
    }
}
