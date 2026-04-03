// use crate::ast::Json;

pub trait ToString {
    fn to_string(&self) -> String;
}

pub trait FromString: Sized {
    fn from_string(s: &str) -> Result<Self, String>;
}

// impl FromString for String {
//     fn from_string(s: Json) -> Result<Self, String> {
//         if let Json::String(v) = s {
//             Ok(v)
//         } else {
//             Err("Expected a string".into())
//         }
//     }
// }
impl ToString for u64 {
    fn to_string(&self) -> String {
        std::string::ToString::to_string(&self)
    }
}

impl FromString for u64 {
    fn from_string(s: &str) -> Result<Self, String> {
        s.parse::<u64>().map_err(|e| e.to_string())
    }
}