# FSPIOX-API

This is a Rust implementation of the FSPIOP-API. It aims to leverage Rust's type system to provide
a set of primitives that are correct by construction. Where present in the FSPIOP spec, it will
serialise to use the same attribute names and cases. Where not present, it will typically prefer
`snake_case`. Deserialisation will always be to idiomatic Rust names until and if this is ever a
hindrance.

### TODO:
- rustdoc
- replace usage of derive_more with strum?
- logging feature
- put postgres ToSql usage behind a feature
