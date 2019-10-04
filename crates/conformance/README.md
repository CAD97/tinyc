# Conformance tests

`simple.yaml.test`:

```yaml
1
===
a=b=c=2<3;
---
- Identifier: 1
- EqualsSign: 1
- Identifier: 1
- EqualsSign: 1
- Identifier: 1
- EqualsSign: 1
- Integer: 1
- LessThanSign: 1
- Integer: 1
- Semicolon: 1
...
```

`test.rs`:

```rust
use {
    conformance, serde_yaml,
    tinyc_lexer::{tokenize, Token},
};

#[conformance::tests(exact,
    ser = serde_yaml::to_string,
    de = serde_yaml::from_str,
    file = "tests/simple.yaml.test")]
fn lex_tokens(s: &str) -> Vec<Token> {
    tokenize(s).collect()
}
```

This grabs the input from between `===` and `---`,
passes it to the test function,
then serializes it with the `ser` function.
The output is grabbed from between `---` and `...`,
then normalized by `de`serializing and then re`ser`ializing.
The two serialized forms are compared with `assert_eq!`.
The file path is relative to the Cargo manifest.

Any number of tests can be included in one conformance test file.
The file name and the test name (above the `===`) are combined
and used to name the test given to the standard Rust test runner.

The `ser` and `de` functions don't have to be `serde`, they just
have to meet the shape of `fn<T>(&T) -> String` for serialization
and `fn<T>(&str) -> Result<T, impl Error>` for deserialization.

For more information, see the [dev.to announcement post][blog].

  [blog]: <https://dev.to/cad97/conformance-testing-in-rust-3h5m>
