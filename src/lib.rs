pub mod traits;
pub mod ast;
pub mod parser;

use crate::traits::{ToString, FromString};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn to_string<T: ToString>(value: &T) -> String {
    value.to_string()
}

pub fn from_str<T: FromString>(s: &str) -> Result<T, String> {
    // serde_json::from_str(s).unwrap()
    T::from_string(s)
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

    // Person implements ToString and FromString traits here for testing purposes
    impl ToString for Person {
        fn to_string(&self) -> String {
            format!("{{\"name\":\"{}\",\"age\":{},\"tags\":[{}]}}",
                self.name,
                self.age,
                self.tags.iter().map(|t| format!("\"{}\"", t)).collect::<Vec<String>>().join(","))
        }
    }

    impl FromString for Person {
        fn from_string(s: &str) -> Result<Self, String> {
            let json = crate::parser::parse(s)?;
            if let Json::Object(map) = json {
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
        let s = to_string(&person);
        let parsed: Person = from_str(&s).expect("deserialize");
        assert_eq!(person, parsed);
    }

    #[test]
    fn test_num() {
        let num: u64 = 25;
        let s = to_string(&num);
        let parsed: u64 = from_str(&s).expect("deseriallize");
        assert_eq!(num, parsed);
    }
}
