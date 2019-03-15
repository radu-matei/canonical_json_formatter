use itoa;
use serde_json::ser::Formatter;
use std::io;

pub struct CanonicalFormatter {}

/// This formatter follows the IETF working draft for Canonical JSON
/// https://tools.ietf.org/html/draft-rundgren-json-canonicalization-scheme-05
impl Formatter for CanonicalFormatter {
    /// Writes a `null` value to the specified writer.
    #[inline]
    fn write_null<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(b"null")
    }

    /// Writes a `true` or `false` value to the specified writer.
    #[inline]
    fn write_bool<W: ?Sized>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: io::Write,
    {
        let s = if value {
            b"true" as &[u8]
        } else {
            b"false" as &[u8]
        };
        writer.write_all(s)
    }

    /// Writes an i8 value like `-123` to the specified writer.
    #[inline]
    fn write_i8<W: ?Sized>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
    where
        W: io::Write,
    {
        itoa::write(writer, value).map(drop)
    }
}

/*

    #[inline]
    fn serialize_bool(&mut self, value: bool) -> Result<(), Error> {
        if value {
            self.0.write_all(b"true").map_err(From::from)
        } else {
            self.0.write_all(b"false").map_err(From::from)
        }
    }

    #[inline]
    fn serialize_isize(&mut self, value: isize) -> Result<(), Error> {
        itoa::write(&mut self.0, value).map_err(From::from)
    }

    #[inline]
    fn serialize_i8(&mut self, value: i8) -> Result<(), Error> {
        itoa::write(&mut self.0, value).map_err(From::from)
    }

    #[inline]
    fn serialize_i16(&mut self, value: i16) -> Result<(), Error> {
        itoa::write(&mut self.0, value).map_err(From::from)
    }

    #[inline]
    fn serialize_i32(&mut self, value: i32) -> Result<(), Error> {
        itoa::write(&mut self.0, value).map_err(From::from)
    }

    #[inline]
    fn serialize_i64(&mut self, value: i64) -> Result<(), Error> {
        itoa::write(&mut self.0, value).map_err(From::from)
    }

    #[inline]
    fn serialize_usize(&mut self, value: usize) -> Result<(), Error> {
        itoa::write(&mut self.0, value).map_err(From::from)
    }

    #[inline]
    fn serialize_u8(&mut self, value: u8) -> Result<(), Error> {
        itoa::write(&mut self.0, value).map_err(From::from)
    }

    #[inline]
    fn serialize_u16(&mut self, value: u16) -> Result<(), Error> {
        itoa::write(&mut self.0, value).map_err(From::from)
    }

    #[inline]
    fn serialize_u32(&mut self, value: u32) -> Result<(), Error> {
        itoa::write(&mut self.0, value).map_err(From::from)
    }

    #[inline]
    fn serialize_u64(&mut self, value: u64) -> Result<(), Error> {
        itoa::write(&mut self.0, value).map_err(From::from)
    }

    #[inline]
    fn serialize_f32(&mut self, value: f32) -> Result<(), Error> {
        fmt_f32_or_null(&mut self.0, value).map_err(From::from)
    }

    #[inline]
    fn serialize_f64(&mut self, value: f64) -> Result<(), Error> {
        fmt_f64_or_null(&mut self.0, value).map_err(From::from)
    }

    #[inline]
    fn serialize_char(&mut self, value: char) -> Result<(), Error> {
        escape_char(&mut self.0, value).map_err(From::from)
    }

    #[inline]
    fn serialize_str(&mut self, value: &str) -> Result<(), Error> {
        escape_str(&mut self.0, value).map_err(From::from)
    }

    #[inline]
    fn serialize_bytes(&mut self, value: &[u8]) -> Result<(), Error> {
        let mut state = try!(self.serialize_seq(Some(value.len())));
        for byte in value {
            try!(self.serialize_seq_elt(&mut state, byte));
        }
        self.serialize_seq_end(state)
    }

    #[inline]
    fn serialize_unit(&mut self) -> Result<(), Error> {
        self.0.write_all(b"null").map_err(From::from)
    }

    #[inline]
    fn serialize_unit_struct(&mut self, _name: &'static str) -> Result<(), Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        &mut self,
        _name: &'static str,
        _variant_index: usize,
        variant: &'static str
    ) -> Result<(), Error> {
        self.serialize_str(variant)
    }

    /// Serialize newtypes without an object wrapper.
    #[inline]
    fn serialize_newtype_struct<T>(
        &mut self,
        _name: &'static str,
        value: T
    ) -> Result<(), Error>
        where T: ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        &mut self,
        _name: &'static str,
        _variant_index: usize,
        variant: &'static str,
        value: T
    ) -> Result<(), Error>
        where T: ser::Serialize,
    {
        try!(self.0.write_all(b"{"));
        try!(self.serialize_str(variant));
        try!(self.0.write_all(b":"));
        try!(value.serialize(self));
        self.0.write_all(b"}").map_err(From::from)
    }

    #[inline]
    fn serialize_none(&mut self) -> Result<(), Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(&mut self, value: T) -> Result<(), Error>
        where T: ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(&mut self, len: Option<usize>) -> Result<State, Error> {
        if len == Some(0) {
            try!(self.0.write_all(b"[]"));
            Ok(State::Empty)
        } else {
            try!(self.0.write_all(b"["));
            Ok(State::First)
        }
    }

    #[inline]
    fn serialize_seq_elt<T: ser::Serialize>(
        &mut self,
        state: &mut State,
        value: T
    ) -> Result<(), Error>
        where T: ser::Serialize,
    {
        if *state != State::First {
            try!(self.0.write_all(b","));
        }
        *state = State::Rest;

        value.serialize(self)
    }

    #[inline]
    fn serialize_seq_end(&mut self, state: State) -> Result<(), Error> {
        match state {
            State::Empty => Ok(()),
            _ => self.0.write_all(b"]").map_err(From::from),
        }
    }

    #[inline]
    fn serialize_seq_fixed_size(&mut self, size: usize) -> Result<State, Error> {
        self.serialize_seq(Some(size))
    }

    #[inline]
    fn serialize_tuple(&mut self, len: usize) -> Result<State, Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_elt<T: ser::Serialize>(
        &mut self,
        state: &mut State,
        value: T
    ) -> Result<(), Error> {
        self.serialize_seq_elt(state, value)
    }

    #[inline]
    fn serialize_tuple_end(&mut self, state: State) -> Result<(), Error> {
        self.serialize_seq_end(state)
    }

    #[inline]
    fn serialize_tuple_struct(
        &mut self,
        _name: &'static str,
        len: usize
    ) -> Result<State, Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct_elt<T: ser::Serialize>(
        &mut self,
        state: &mut State,
        value: T
    ) -> Result<(), Error> {
        self.serialize_seq_elt(state, value)
    }

    #[inline]
    fn serialize_tuple_struct_end(&mut self, state: State) -> Result<(), Error> {
        self.serialize_seq_end(state)
    }

    #[inline]
    fn serialize_tuple_variant(
        &mut self,
        _name: &'static str,
        _variant_index: usize,
        variant: &'static str,
        len: usize
    ) -> Result<State, Error> {
        try!(self.0.write_all(b"{"));
        try!(self.serialize_str(variant));
        try!(self.0.write_all(b":"));
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_variant_elt<T: ser::Serialize>(
        &mut self,
        state: &mut State,
        value: T
    ) -> Result<(), Error> {
        self.serialize_seq_elt(state, value)
    }

    #[inline]
    fn serialize_tuple_variant_end(&mut self, state: State) -> Result<(), Error> {
        try!(self.serialize_seq_end(state));
        self.0.write_all(b"}").map_err(From::from)
    }

    #[inline]
    fn serialize_map(&mut self, len: Option<usize>) -> Result<MapState, Error> {
        if len == Some(0) {
            try!(self.0.write_all(b"{}"));
            Ok(MapState { state: State::Empty, cur_key: None })
        } else {
            try!(self.0.write_all(b"{"));
            Ok(MapState { state: State::First, cur_key: None })
        }
    }

    #[inline]
    fn serialize_map_key<T: ser::Serialize>(
        &mut self,
        state: &mut MapState,
        key: T,
    ) -> Result<(), Error> {
        if state.state != State::First {
            try!(self.0.write_all(b","));
        }

        state.state = State::Rest;
        state.cur_key = {
            let mut key_serializer = AscendingKeySerializer {
                ser: self,
                cur_key: state.cur_key.take(),
            };
            try!(key.serialize(&mut key_serializer));
            key_serializer.cur_key.take()
        };

        self.0.write_all(b":").map_err(From::from)
    }

    #[inline]
    fn serialize_map_value<T: ser::Serialize>(
        &mut self,
        _: &mut MapState,
        value: T,
    ) -> Result<(), Error> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_map_end(&mut self, state: MapState) -> Result<(), Error> {
        match state.state {
            State::Empty => Ok(()),
            _ => self.0.write_all(b"}").map_err(From::from),
        }
    }

    #[inline]
    fn serialize_struct(
        &mut self,
        _name: &'static str,
        len: usize
    ) -> Result<MapState, Error> {
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_elt<V: ser::Serialize>(
        &mut self,
        state: &mut MapState,
        key: &'static str,
        value: V
    ) -> Result<(), Error> {
        try!(self.serialize_map_key(state, key));
        self.serialize_map_value(state, value)
    }

    #[inline]
    fn serialize_struct_end(&mut self, state: MapState) -> Result<(), Error> {
        self.serialize_map_end(state)
    }

    #[inline]
    fn serialize_struct_variant(
        &mut self,
        _name: &'static str,
        _variant_index: usize,
        variant: &'static str,
        len: usize
    ) -> Result<MapState, Error> {
        try!(self.0.write_all(b"{"));
        try!(self.serialize_str(variant));
        try!(self.0.write_all(b":"));
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_variant_elt<V: ser::Serialize>(
        &mut self,
        state: &mut MapState,
        key: &'static str,
        value: V
    ) -> Result<(), Error> {
        self.serialize_struct_elt(state, key, value)
    }

    #[inline]
    fn serialize_struct_variant_end(&mut self, state: MapState) -> Result<(), Error> {
        try!(self.serialize_struct_end(state));
        self.0.write_all(b"}").map_err(From::from)
    }
*/
