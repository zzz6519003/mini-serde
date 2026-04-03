use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

impl Json {
    pub fn to_string(&self) -> String {
        match self {
            Json::Null => "null".into(),
            Json::Bool(b) => b.to_string(),
            Json::Number(n) => {
                if n.fract() == 0.0 { format!("{}", *n as i64) }
                else { n.to_string() }
            }
            Json::String(s) => format!("\"{}\"", escape_str(s)),
            Json::Array(a) => {
                let inner: Vec<String> = a.iter().map(|v| v.to_string()).collect();
                format!("[{}]", inner.join(","))
            }
            Json::Object(o) => {
                let mut parts: Vec<String> = o.iter().map(|(k, v)| format!("\"{}\":{}", escape_str(k), v.to_string())).collect();
                parts.sort();
                format!("{{{}}}", parts.join(","))
            }
        }
    }
}

fn escape_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
}
