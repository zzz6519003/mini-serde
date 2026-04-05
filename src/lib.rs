pub mod traits;
pub mod ast;
pub mod parser;

pub use mini_serde_macros::*;
pub use ast::Json;
pub use traits::{ToJSONString, FromString};
pub use traits::{Serialize, Deserialize};

// use crate::traits::{ToString, FromString};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn to_json_string<T: ToJSONString>(value: &T) -> String {
    value.to_json_value().to_string()
}

pub fn from_str<T: Deserialize>(s: &str) -> Result<T, String> {
    let json = crate::parser::parse(s)?;
    T::from_json_value(json)
}

#[cfg(test)]
mod tests {
    use crate::ast::Json;

    use super::*;

    #[derive(Debug, PartialEq)]
    struct Person {
        name: String,
        age: u8,
        tags: Vec<String>,
    }

    // Person implements ToJSONString and Deserialize traits here for testing purposes
    impl ToJSONString for Person {
        fn to_json_value(&self) -> Json {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            map.insert("name".to_string(), Json::String(self.name.clone()));
            map.insert("age".to_string(), Json::Number(self.age as f64));
            map.insert("tags".to_string(), Json::Array(
                self.tags.iter().map(|t| Json::String(t.clone())).collect()
            ));
            Json::Object(map)
        }
    }

    impl Deserialize for Person {
        fn from_json_value(v: Json) -> Result<Self, String> {
            if let Json::Object(map) = v {
                let name = if let Some(Json::String(n)) = map.get("name") { n.clone() } else { return Err("Missing name".into()) };
                let age = if let Some(Json::Number(a)) = map.get("age") { *a as u8 } else { return Err("Missing age".into()) };
                let tags = if let Some(Json::Array(t)) = map.get("tags") {
                    t.iter().map(|v| {
                        if let Json::String(s) = v { Ok(s.clone()) } else { Err("Invalid tag".into()) }
                    }).collect::<Result<Vec<String>, String>>()?
                } else { return Err("Missing tags".into()) };
                Ok(Person { name, age, tags })
            } else {
                Err("Expected an object".into())
            }
        }
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn person_roundtrip() {
        let person = Person {
            name: "Alice".into(),
            age: 30,
            tags: vec!["developer".into(), "rustacean".into()],
        };
        let s = to_json_string(&person);
        let parsed: Person = from_str(&s).expect("deserialize");
        assert_eq!(person, parsed);
    }

    #[test]
    fn test_num() {
        let num: u64 = 25;
        let s = to_json_string(&num);
        let parsed: u64 = from_str(&s).expect("deseriallize");
        assert_eq!(num, parsed);
    }
}
