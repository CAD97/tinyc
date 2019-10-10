# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
with the `0.MAJOR.PATCH` extension.

## [Unreleased]

## [0.2.0] - 2019-10-10

### Added

- `conformance::tests` now takes an optional third path argument, `value`,
a path to some generic de/serializable object
(e.g. [`serde_json::Value`](https://docs.serde.rs/serde_json/enum.Value.html)).
If `value` is provided,
the function return value does not have to implement `Deserialize`.
- `conformance::tests` now optionally takes a shortcut argument `serde`
before `ser`, `de`, and `value`.
If present, `serde` defaults the values of `ser`, `de`, and `value` to
`serde::to_string`, `serde::from_str`, and `serde::value`, respectively.
`ser`, `de`, and `value` may still appear afterwards to override said defaults.
Example: `serde=serde_json, ser=serde_json::to_string_pretty`
will use `serde_json` to test, as the prettified JSON rather than minified.

## 0.1.0 - 2019-10-4

Initial release.

### Added

- `conformance::tests` attribute macro
  - arguments: `(exact, ser = $path, de = $path, file = "file/path")`
  - apply to: function declaration returning some `impl Serialize + Deserialize`

  [Unreleased]: <https://github.com/CAD97/tinyc/compare/conformance-0.2...master>
  [0.2.0]: <https://github.com/CAD97/tinyc/compare/b70fd58...conformance-0.2>