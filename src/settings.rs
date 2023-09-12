use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Deserialize, Clone)]
pub struct Infer {
    pub min_domain_occurs: i32,
    pub max_ns: i32,
    pub iri_max_length: i32,
    pub min_ns_size: i32,
    pub iri_trie_size: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Community {
    pub url: String,
    pub dir: String,
    pub path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Visualization {
    pub render_dir: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub infer: Infer,
    pub community: Community,
    pub visualization: Visualization,
    pub env: ENV,
}

const CONFIG_FILE_PATH: &str = "./config/Default.toml";
const CONFIG_FILE_PREFIX: &str = "./config/";

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Testing,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Testing => write!(f, "Testing"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Development".into());
        let mut s = Config::new();
        s.set("env", env.clone())?;

        s.merge(File::with_name(CONFIG_FILE_PATH))?;
        s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;

        // This makes it so "EA_SERVER__PORT overrides server.port
        s.merge(Environment::with_prefix("ea").separator("__"))?;

        s.try_into()
    }
}
