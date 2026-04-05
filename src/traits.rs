use crate::ast::Json;
use std::collections::HashMap;

pub trait Serialize { fn to_json_value(&self) -> Json; }
pub trait Deserialize: Sized { fn from_json_value(v: Json) -> Result<Self, String>; }

// Re-export for convenience
pub use Serialize as ToJSONString;
pub use Deserialize as FromString;

// Primitive impls
impl Serialize for String { fn to_json_value(&self) -> Json { Json::String(self.clone()) } }
impl Serialize for &str { fn to_json_value(&self) -> Json { Json::String(self.to_string()) } }
impl Serialize for bool { fn to_json_value(&self) -> Json { Json::Bool(*self) } }
impl Serialize for f64 { fn to_json_value(&self) -> Json { Json::Number(*self) } }
impl Serialize for f32 { fn to_json_value(&self) -> Json { Json::Number(*self as f64) } }
impl Serialize for i64 { fn to_json_value(&self) -> Json { Json::Number(*self as f64) } }
impl Serialize for i32 { fn to_json_value(&self) -> Json { Json::Number(*self as f64) } }
impl Serialize for u64 { fn to_json_value(&self) -> Json { Json::Number(*self as f64) } }
impl Serialize for u32 { fn to_json_value(&self) -> Json { Json::Number(*self as f64) } }
impl Serialize for usize { fn to_json_value(&self) -> Json { Json::Number(*self as f64) } }

impl Deserialize for String { fn from_json_value(v: Json) -> Result<Self, String> { if let Json::String(s) = v { Ok(s) } else { Err("expected string".into()) } } }
impl Deserialize for bool { fn from_json_value(v: Json) -> Result<Self, String> { if let Json::Bool(b) = v { Ok(b) } else { Err("expected bool".into()) } } }
impl Deserialize for f64 { fn from_json_value(v: Json) -> Result<Self, String> { if let Json::Number(n) = v { Ok(n) } else { Err("expected number".into()) } } }
impl Deserialize for f32 { fn from_json_value(v: Json) -> Result<Self, String> { if let Json::Number(n) = v { Ok(n as f32) } else { Err("expected number".into()) } } }
impl Deserialize for i64 { fn from_json_value(v: Json) -> Result<Self, String> { if let Json::Number(n) = v { Ok(n as i64) } else { Err("expected number".into()) } } }
impl Deserialize for i32 { fn from_json_value(v: Json) -> Result<Self, String> { if let Json::Number(n) = v { Ok(n as i32) } else { Err("expected number".into()) } } }
impl Deserialize for u64 { fn from_json_value(v: Json) -> Result<Self, String> { if let Json::Number(n) = v { Ok(n as u64) } else { Err("expected number".into()) } } }
impl Deserialize for u32 { fn from_json_value(v: Json) -> Result<Self, String> { if let Json::Number(n) = v { Ok(n as u32) } else { Err("expected number".into()) } } }
impl Deserialize for usize { fn from_json_value(v: Json) -> Result<Self, String> { if let Json::Number(n) = v { Ok(n as usize) } else { Err("expected number".into()) } } }

// Option
impl<T: Serialize> Serialize for Option<T> { fn to_json_value(&self) -> Json { match self { Some(v) => v.to_json_value(), None => Json::Null } } }
impl<T: Deserialize> Deserialize for Option<T> { fn from_json_value(v: Json) -> Result<Self, String> { match v { Json::Null => Ok(None), other => Ok(Some(Deserialize::from_json_value(other)?)) } } }

// Vec
impl<T: Serialize> Serialize for Vec<T> { fn to_json_value(&self) -> Json { Json::Array(self.iter().map(|v| v.to_json_value()).collect()) } }
impl<T: Deserialize> Deserialize for Vec<T> { fn from_json_value(v: Json) -> Result<Self, String> { if let Json::Array(arr) = v { arr.into_iter().map(|el| Deserialize::from_json_value(el)).collect() } else { Err("expected array".into()) } } }

// HashMap<String, T>
impl<T: Serialize> Serialize for HashMap<String, T> { fn to_json_value(&self) -> Json { let mut m = HashMap::new(); for (k, v) in self.iter() { m.insert(k.clone(), v.to_json_value()); } Json::Object(m) } }
impl<T: Deserialize> Deserialize for HashMap<String, T> { fn from_json_value(v: Json) -> Result<Self, String> { if let Json::Object(map) = v { let mut out = HashMap::new(); for (k, v) in map { out.insert(k, Deserialize::from_json_value(v)?); } Ok(out) } else { Err("expected object".into()) } } }
