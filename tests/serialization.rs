use mini_serde::{from_str, to_json_string, Serialize, Deserialize};
// use crate::li

#[test]
fn test_num() {
    let num: u64 = 25;
    let s = to_json_string(&num);
    let parsed: u64 = from_str(&s).expect("deseriallize");
    assert_eq!(num, parsed);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
    email: Option<String>,
    hobbies: Vec<String>,
    foo: std::collections::HashMap<String, String>,
}

#[test]
fn user_roundtrip() {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        email: Some("alice@example.com".to_string()),
        hobbies: vec!["reading".to_string(), "coding".to_string()],
        foo: [("key1".to_string(), "value1".to_string()), ("key2".to_string(), "value2".to_string())].iter().cloned().collect(),
    };

    // Serialize to JSON
    let json_str = to_json_string(&user);
    println!("Serialized: {}", json_str);

    // Deserialize from JSON
    let deserialized: User = from_str(&json_str).expect("Failed to deserialize");
    println!("Deserialized: {:?}", deserialized);

    assert_eq!(user, deserialized);
    println!("Roundtrip successful!");
}