# FSPIOX-API

This is a Rust implementation of the FSPIOP-API. It aims to leverage Rust's type system to provide
a set of primitives that are correct by construction. Where present in the FSPIOP spec, it will
serialise to use the same attribute names and cases. Where not present, it will typically prefer
`snake_case`. Deserialisation will always be to idiomatic Rust names until and if this is ever a
hindrance.

### TODO:
- Create client pools
  - Use: https://github.com/bikeshedder/deadpool ?
  - Note: https://github.com/bikeshedder/deadpool#differences-to-other-connection-pool-implementations
- Search all uses of .unwrap and other panics and:
  - document whether and why they're valid, create tests to exercise them or prove they won't
      happen; or
  - obviate them
- Split types into a separate crate from all functions/impls? Probably keep everything in the same
    repo. Is this just a nuisance? Could have the following crates:
    - fspiox-api
    - mojaloop-api
    - mojaloop-kube
- Put the fspiox-api repo into the mojaloop-api repo (but keep the crates separate)?
- Rustdoc
- Replace usage of derive_more with strum?
- Logging feature
- If possible, compile with various feature combinations in CI
