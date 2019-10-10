use {
    conformance, serde_yaml,
    tinyc_lexer::{tokenize, Token},
};

#[conformance::tests(exact, serde=serde_yaml, file="tests/main.yaml.test")]
fn lex_tokens(s: &str) -> Vec<Token> {
    tokenize(s).collect()
}
