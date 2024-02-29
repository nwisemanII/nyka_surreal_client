// use rs_autotask_api::debug;
use crate::SurrealID;
use serde::Deserializer;
use serde_json::Map;
use serde_json::Value;
use surrealdb::sql::Id;

pub fn deserialize_id<'de, D>(deserializer: D) -> Result<SurrealID, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = SurrealID;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i64 or a map")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let str_val = value.parse::<String>().unwrap();;
            // Check for `:` in the string
            // TODO:
            Ok(SurrealID::from(("default".to_string(), str_val)))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let sid: SurrealID = SurrealID::new("default", Some(Id::from(value as i64)));
            Ok(sid)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let sid: SurrealID = SurrealID::new("default", Some(Id::from(value as u64)));
            Ok(sid)
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let _table: Option<(String, String)> = map.next_entry()?;
            let id: (String, Map<String, Value>) = map.next_entry()?.unwrap();

            let _id = id.1.get("Number").unwrap();

            let sid: SurrealID = SurrealID::new(_table.unwrap().1.as_str(), Some(Id::from(_id.to_string())));
            Ok(sid)
        }
    }

    deserializer.deserialize_any(Visitor)
}
