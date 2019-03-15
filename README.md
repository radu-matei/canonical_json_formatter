# Canonical JSON Formatter for Rust

This is a work-in-progress for creating a canonical JSON formatter in Rust, compatible with `serde_json` and compliant with the Canonical JSON Specification.

Ideally, this would be used as shown below:

```
let mut ser = serde_json::Serializer::with_formatter(&mut out, canonical_json::Formatter);
t.serialize(&mut ser)?;
```