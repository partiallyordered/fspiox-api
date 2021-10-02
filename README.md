# FSPIOX-API

This is a Rust implementation of the FSPIOP-API. It aims to leverage Rust's type system to provide
a set of primitives that are correct by construction. Where present in the FSPIOP spec, it will
serialise to use the same attribute names and cases. Where not present, it will typically prefer
`snake_case`. Deserialisation will always be to idiomatic Rust names until and if this is ever a
hindrance.

### TODO:
- If we use `type_alias_impl_trait` (nightly) we should be able to constrain the client API more,
    such that every request type has a corresponding response type, and client.send returns the
    appropriate type raw, not wrapped in an enum. See the `Unstable Features` section of this
    answer: https://stackoverflow.com/a/65924807 . The reason we don't do this is because our
    client `.send` methods are async, and I couldn't figure out how to type-parameterise them such
    at this works. Perhaps try this again, though. Details in the aforementioned SO answer. Is this
    an example?: https://docs.rs/hyper/0.14.13/src/hyper/client/conn.rs.html#806-864
- Create client pools
  - Use: https://github.com/bikeshedder/deadpool ?
  - Note: https://github.com/bikeshedder/deadpool#differences-to-other-connection-pool-implementations
- Search all uses of .unwrap and other panics and:
  - document whether and why they're valid, create tests to exercise them or prove they won't
      happen; or
  - obviate them
- Split types into a separate crate from all functions/impls? Probably keep everything in the same
    repo. Is this just a nuisance?
- Put the fspiox-api repo into the mojaloop-api repo (but keep the crates separate)?
- Rustdoc
- Replace usage of derive_more with strum?
- Logging feature
- If possible, compile with various feature combinations in CI
