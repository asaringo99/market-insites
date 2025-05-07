pub mod country;
pub mod value;
pub mod naics;
pub mod time;

pub trait FieldPairs {
	fn put_into(&self, map: &mut ::serde_json::Map<String, ::serde_json::Value>);
}
