use std::io;
use std::fmt;
use slog;

// Key Separator Value Serializer
pub struct KSVSerializer<W> {
    io: W,
    separator: String,
}

impl<W: io::Write> KSVSerializer<W> {
    pub fn new(io: W, separator: &str) -> Self {
        KSVSerializer {
            io: io,
            separator: separator.to_owned(),
        }
    }

    pub fn finish(self) -> W {
        self.io
    }
}

macro_rules! ksv_emit (
    ($func_name:ident,T $value_type:ty) => (
        fn $func_name(&mut self, key: &str, val: $value_type) -> slog::ser::Result {
            write!(self.io, "{}{}{}", key, self.separator, val)?;
            Ok(())
        }
    );
    ($func_name:ident,V $value:expr) => (
        fn $func_name(&mut self, key: &str) -> slog::ser::Result {
            write!(self.io, "{}{}{}", key, self.separator, $value)?;
            Ok(())
        }
    );
);

impl<W: io::Write> slog::ser::Serializer for KSVSerializer<W> {

    ksv_emit!(emit_none, V "None");
    ksv_emit!(emit_unit, V "()");
    ksv_emit!(emit_bool, T bool);
    ksv_emit!(emit_char, T char);
    ksv_emit!(emit_usize, T usize);
    ksv_emit!(emit_isize, T isize);
    ksv_emit!(emit_u8,  T u8);
    ksv_emit!(emit_i8,  T i8);
    ksv_emit!(emit_u16, T u16);
    ksv_emit!(emit_i16, T i16);
    ksv_emit!(emit_u32, T u32);
    ksv_emit!(emit_i32, T i32);
    ksv_emit!(emit_f32, T f32);
    ksv_emit!(emit_u64, T u64);
    ksv_emit!(emit_i64, T i64);
    ksv_emit!(emit_f64, T f64);
    ksv_emit!(emit_str, T &str);
    ksv_emit!(emit_arguments, T &fmt::Arguments);

}

// struct CEESerializer<W> {
//     io: W,
// }

// impl<W: io::Write> CEESerializer<W> {

//     fn serialize(&self,
//                  io: &mut io::Write,
//                  rinfo: &Record,
//                  logger_values: &OwnedKeyValueList)
//                  -> io::Result<()> {

//         let serializer = serde_json::Serializer::new(io);
//         let mut serializer = try!(SerdeSerializer::start(serializer, None));

//         let _ = try!(io.write_all("cee@:".as_bytes()));

//         for (ref k, ref v) in logger_values.iter() {
//             try!(v.serialize(rinfo, k, &mut serializer));
//         }

//         for &(ref k, ref v) in rinfo.values().iter() {
//             try!(v.serialize(rinfo, k, &mut serializer));
//         }

//         let (serializer, res) = serializer.end();

//         let _ = try!(res);
//         Ok(())
//     }
// }
