// use serde;
// use serde::ser::SerializeMap;

use slog;
// use slog::OwnedKeyValueList;
// use slog::Record;
use std::{io, fmt};
// use std::cell::RefCell;
// use std::fmt::Write;
use std::marker::PhantomData;

/// `Unquoted` key=value
pub struct Unquoted {}

///  `QuotedValue` key="value"
pub struct QuotedValue {}

/// Key Separator Value Serializer
pub struct KsvSerializer<W, Q> {
    io: W,
    separator: String,
    _quoting: PhantomData<Q>,
}

impl<W, Q> KsvSerializer<W, Q>
    where W: io::Write
{
    /// Return new instance of serializers with specified separator
    pub fn new(io: W, separator: &str) -> Self {
        KsvSerializer {
            io: io,
            separator: separator.to_owned(),
            _quoting: PhantomData,
        }
    }

    /// Return back borrowed mutable handle to `io`
    /// at the end of serialization
    pub fn finish(self) -> W {
        self.io
    }

    /// Emit k/v delimiter, in case of syslog it's always space.
    pub fn emit_delimiter(&mut self) -> slog::ser::Result {
        write!(self.io, " ")?;
        Ok(())
    }
}

macro_rules! impl_unquoted_serialize_for (
    (T $value_type:ty, $func_name:ident) => (
        fn $func_name(&mut self, key: &str, val: $value_type) -> slog::ser::Result {
            write!(self.io, "{}{}{}", key, self.separator, val)?;
            Ok(())
        }
    );
    (V $value:expr, $func_name:ident) => (
        fn $func_name(&mut self, key: &str) -> slog::ser::Result {
            write!(self.io, "{}{}{}", key, self.separator, $value)?;
            Ok(())
        }
    );
);

macro_rules! impl_quoted_value_serialize_for (
    (T $value_type:ty, $func_name:ident) => (
        fn $func_name(&mut self, key: &str, val: $value_type) -> slog::ser::Result {
            write!(self.io, "{}{}\"{}\"", key, self.separator, val)?;
            Ok(())
        }
    );
    (V $value:expr, $func_name:ident) => (
        fn $func_name(&mut self, key: &str) -> slog::ser::Result {
            write!(self.io, "{}{}\"{}\"", key, self.separator, $value)?;
            Ok(())
        }
    );
);

impl<W: io::Write> slog::ser::Serializer for KsvSerializer<W, Unquoted> {
    impl_unquoted_serialize_for!(V "None", emit_none);
    impl_unquoted_serialize_for!(V "()", emit_unit);
    impl_unquoted_serialize_for!(T bool, emit_bool);
    impl_unquoted_serialize_for!(T char, emit_char);
    impl_unquoted_serialize_for!(T usize, emit_usize);
    impl_unquoted_serialize_for!(T isize, emit_isize);
    impl_unquoted_serialize_for!(T u8, emit_u8);
    impl_unquoted_serialize_for!(T i8, emit_i8);
    impl_unquoted_serialize_for!(T u16, emit_u16);
    impl_unquoted_serialize_for!(T i16, emit_i16);
    impl_unquoted_serialize_for!(T u32, emit_u32);
    impl_unquoted_serialize_for!(T i32, emit_i32);
    impl_unquoted_serialize_for!(T f32, emit_f32);
    impl_unquoted_serialize_for!(T u64, emit_u64);
    impl_unquoted_serialize_for!(T i64, emit_i64);
    impl_unquoted_serialize_for!(T f64, emit_f64);
    impl_unquoted_serialize_for!(T & str, emit_str);
    impl_unquoted_serialize_for!(T & fmt::Arguments, emit_arguments);
}

impl<W: io::Write> slog::ser::Serializer for KsvSerializer<W, QuotedValue> {
    impl_quoted_value_serialize_for!(V "None", emit_none);
    impl_quoted_value_serialize_for!(V "()", emit_unit);
    impl_quoted_value_serialize_for!(T bool, emit_bool);
    impl_quoted_value_serialize_for!(T char, emit_char);
    impl_quoted_value_serialize_for!(T usize, emit_usize);
    impl_quoted_value_serialize_for!(T isize, emit_isize);
    impl_quoted_value_serialize_for!(T u8, emit_u8);
    impl_quoted_value_serialize_for!(T i8, emit_i8);
    impl_quoted_value_serialize_for!(T u16, emit_u16);
    impl_quoted_value_serialize_for!(T i16, emit_i16);
    impl_quoted_value_serialize_for!(T u32, emit_u32);
    impl_quoted_value_serialize_for!(T i32, emit_i32);
    impl_quoted_value_serialize_for!(T f32, emit_f32);
    impl_quoted_value_serialize_for!(T u64, emit_u64);
    impl_quoted_value_serialize_for!(T i64, emit_i64);
    impl_quoted_value_serialize_for!(T f64, emit_f64);
    impl_quoted_value_serialize_for!(T & str, emit_str);
    impl_quoted_value_serialize_for!(T & fmt::Arguments, emit_arguments);
}

/// Unquoted KSV serializer ex: key=value
pub type KsvSerializerUnquoted<W> = KsvSerializer<W, Unquoted>;

/// Quoted value KSV serializer ex: key="value"
pub type KsvSerializerQuotedValue<W> = KsvSerializer<W, QuotedValue>;
