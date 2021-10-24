#[derive(Clone, Debug, Serialize, Deserialize)]
struct User {
	id: u64,
	name: Name,
}

// Serializeを追加
#[derive(Clone, Debug, Serialize)]
struct Name(String);

// Deserializeの実装を行う
impl<'de> de::Deserialize<'de> for Name {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: de::Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		Name::new(&s).map_err(de::Error::custom)
	}
}
