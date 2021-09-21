# FSPIOX-API

This is a Rust implementation of the FSPIOP-API. It aims to leverage Rust's type system to provide
a set of primitives that are correct by construction. Where present in the FSPIOP spec, it will
serialise to use the same attribute names and cases. Where not present, it will typically prefer
`snake_case`. Deserialisation will always be to idiomatic Rust names until and if this is ever a
hindrance.

### TODO:
- Search all uses of .unwrap and other panics, document whether and why they're valid, or fix them
- Split types into a separate crate from all functions/impls? Is this just a nuisance?
- Rustdoc
- Replace usage of derive_more with strum?
- Logging feature
- Put postgres ToSql usage behind a feature
- If possible, compile with various feature combinations in CI
