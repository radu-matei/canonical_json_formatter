use itoa;
use serde_json::*;
use std::io;

/// This formatter implements the canonical JSON schema that can be found at
/// http://wiki.laptop.org/go/Canonical_JSON
pub struct Canonical {}

impl ser::Formatter for Canonical {
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
