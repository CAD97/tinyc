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

#[conformance::tests(exact, serde=serde_yaml, file="tests/simple.yaml.test")]
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

The `serde` argument stands in for three arguments
that may be provided, in order, in its place:
- `ser`: `fn<T>(&T) -> String` (default `serde::to_string`)
- `de`: `fn(&str) -> Result<value, impl Error>` (default `serde::from_str`)
- `value`: A type that be passed to `ser` (default `serde::Value`)

You can also just supply `ser` and `de`,
and `value` defaults to the produced type.

For more information, see the [dev.to announcement post][blog]
or @ me [on Discord][Discord].

  [blog]: <https://dev.to/cad97/conformance-testing-in-rust-3h5m>
  [Discord]: <https://discord.gg/FuPE9JE>
