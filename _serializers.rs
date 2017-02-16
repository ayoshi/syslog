// Key Separator ValueS erializer
pub struct KSVSerializer<W, > {
    io: W,
    separator: String,
}

impl<W: io::Write> KSVSerializer<W> {
    pub fn new(io: W, separator: String) -> Self {
        KSVSerializer { io: io , separator: separator }
    }

    pub fn finish(self) -> W {
        self.io
    }
}

macro_rules! s(
    ($w:expr, $k:expr, $s:expr, $v:expr) => {
        write!($w.io, "{}{}{}", $k, $s, $v)?;
    };
);

impl<W: io::Write> slog::ser::Serializer for KSVSerializer<W> {

    fn emit_none(&mut self, key: &str) -> ser::Result {
        s!(self, key, self.separator, "None");
        Ok(())
    }

    fn emit_unit(&mut self, key: &str) -> ser::Result {
        s!(self, key, self.separator, "()");
        Ok(())
    }

    fn emit_bool(&mut self, key: &str, val: bool) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_char(&mut self, key: &str, val: char) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_usize(&mut self, key: &str, val: usize) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_isize(&mut self, key: &str, val: isize) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_u8(&mut self, key: &str, val: u8) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_i8(&mut self, key: &str, val: i8) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_u16(&mut self, key: &str, val: u16) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_i16(&mut self, key: &str, val: i16) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_u32(&mut self, key: &str, val: u32) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_i32(&mut self, key: &str, val: i32) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_f32(&mut self, key: &str, val: f32) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_u64(&mut self, key: &str, val: u64) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_i64(&mut self, key: &str, val: i64) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_f64(&mut self, key: &str, val: f64) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_str(&mut self, key: &str, val: &str) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }

    fn emit_arguments(&mut self, key: &str, val: &fmt::Arguments) -> ser::Result {
        s!(self, key, self.separator, val);
        Ok(())
    }
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
