use {
    conformance, serde_yaml,
    tinyc_lexer::{tokenize, Token},
};

mod ron_pretty {
    use serde::Serialize;

    pub fn to_string<T>(value: &T) -> ron::ser::Result<String>
    where
        T: Serialize,
    {
        ron::ser::to_string_pretty(value, Default::default())
    }
}

#[conformance::tests(exact,
    ser = serde_yaml::to_string,
    de = serde_yaml::from_str,
    file = "tests/KartikTalwar.yaml.test")]
#[conformance::tests(exact,
    ser = ron_pretty::to_string,
    de = ron::de::from_str,
    file = "tests/KartikTalwar.ron.test")]
#[conformance::tests(exact,
    ser = serde_json::to_string_pretty,
    de = serde_json::from_str,
    file = "tests/KartikTalwar.json.test")]
fn lex_tokens(s: &str) -> Vec<Token> {
    tokenize(s).collect()
}
