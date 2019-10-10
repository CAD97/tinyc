use {
    glob::glob,
    heck::*,
    serde::{Deserialize, Serialize},
    std::{
        collections::HashMap,
        env,
        error::Error,
        fs,
        path::{Path, PathBuf},
    },
    tera::{self, Context, Tera, Value},
    toml,
};

const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");
const SYNTAX_CONFIG: &str = "meta/syntax.toml";
const TEMPLATE_DIR: &str = "templates";

pub const SYNTAX_KINDS: &str = "syntax_kinds.rs";

fn project_root() -> &'static Path {
    Path::new(MANIFEST).ancestors().nth(2).unwrap()
}

#[derive(Serialize, Deserialize)]
struct SyntaxConfig {
    keywords: Vec<String>,
    literals: Vec<String>,
    punctuation: Vec<PunctuationConfig>,
    tokens: Vec<String>,
}

#[derive(Serialize)]
struct PunctuationConfig {
    character: char,
    name: String,
}

impl<'de> Deserialize<'de> for PunctuationConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::de::Deserializer<'de>>::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "PunctuationConfig")]
        struct Helper(char, String);
        Helper::deserialize(deserializer).map(|helper| PunctuationConfig {
            character: helper.0,
            name: helper.1,
        })
    }
}

fn make_filter_fn<'a, T: Into<Value> + serde::de::DeserializeOwned>(
    name: &'a str,
    f: impl Fn(T) -> T + Sync + Send + 'a,
) -> impl tera::Filter + 'a {
    move |value: &Value, _: &HashMap<String, Value>| -> tera::Result<Value> {
        let val = tera::try_get_value!(name, "value", T, value);
        Ok(f(val).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = project_root();
    let templates = root.join(TEMPLATE_DIR);
    let out = PathBuf::from(env::var("OUT_DIR")?);
    let syntax_config = root.join(SYNTAX_CONFIG);

    println!("cargo:rerun-if-changed={}", syntax_config.to_string_lossy());
    for path in glob(&templates.to_string_lossy())? {
        println!("cargo:rerun-if-changed={}", path?.to_string_lossy());
    }

    let tera = {
        let mut tera = Tera::new(&root.join(templates.join("**/*")).to_string_lossy())
            .unwrap_or_else(|e| panic!("{}", e));
        tera.register_filter(
            "camel_case",
            make_filter_fn("camel_case", |s: String| s.to_camel_case()),
        );
        tera.register_filter(
            "snake_case",
            make_filter_fn("snake_case", |s: String| s.to_snake_case()),
        );
        tera
    };

    let config: SyntaxConfig = toml::from_str(&fs::read_to_string(syntax_config)?)?;
    let context = Context::from_serialize(config)?;

    fs::write(
        out.join(SYNTAX_KINDS),
        tera.render(SYNTAX_KINDS, context.clone())?,
    )?;
    Ok(())
}
