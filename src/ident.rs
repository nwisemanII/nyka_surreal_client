use std::fmt::Display;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SurrealId(pub Thing);

impl Display for SurrealId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl<'de> Deserialize<'de> for SurrealId {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let val = Value::deserialize(deserializer)?;
//         println!("{:#?}", val.bright_yellow());
//         // let thing = Thing::from(obj);

//         let json: Result<Thing, serde_json::Error> = serde_json::from_value(val.into_json());
//         Ok(SurrealId(json.unwrap()))
//     }
// }
// pub trait SurrealIDFactory {
//     fn create(tb: &str, id: &str) -> SurrealID {
//         todo!("SurrealIDFactory::create()");
//         // SurrealID { tb: tb.to_string(), id: Some(Id::from(id)) }
//     }
//     fn random(tb: &str) -> SurrealID {
//         todo!("SurrealIDFactory::random()");
//         // SurrealID { tb: tb.to_string(), id: Some(Id::rand()) }
//     }
// }

// // pub trait HasSurrealIdentifier: SurrealIDTable + SurrealIDIdent {}
// pub trait HasSurrealIdentifier {
//     fn id(&self) -> Id;
//     fn table(&self) -> String;
// }

// pub trait SurrealData
// where
//     Self: Sized + Clone,
// {
//     fn data<T: std::convert::From<Self>>(self) -> T {
//         <Self as std::convert::Into<T>>::into(self.clone())
//     }
// }
// pub trait SurrealIDTable {
//     fn table(&self) -> String;
// }
// pub trait SurrealIDIdent {
//     fn id(&self) -> Id;
// }

// impl SurrealIDFactory for SurrealID {}
// impl SurrealIDIdent for SurrealID {
//     fn id(&self) -> Id {
//         // todo!( "SurrealID::id()" );
//         match self {
//             SurrealID::Thing(thing) => thing.id.clone(),
//             SurrealID::TableId(_, id) => id.clone(),
//             SurrealID::Id(id) => id.clone(),
//             _ => unimplemented!("SurrealID::id( !create ) "), // match create {
//                                                               //     true => unimplemented!("    SurrealID::id( create ) "),
//                                                               //     false => unimplemented!("    SurrealID::id( !create ) "),
//                                                               // }
//         }
//     }
// }
// impl SurrealIDTable for SurrealID {
//     fn table(&self) -> String {
//         // todo!(  "SurrealID::table()" );
//         match self {
//             SurrealID::Thing(thing) => thing.tb.clone(),
//             SurrealID::Table(tb) => tb.clone(),
//             SurrealID::TableId(tb, _) => tb.clone(),
//             _ => unimplemented!("SurrealID::table( !create ) "), // _ => match create {
//                                                                  //     true => unimplemented!("    SurrealID::table( create ) "),
//                                                                  //     false => unimplemented!("    SurrealID::table( !create ) "),
//                                                                  // }
//         }
//         // self.tb.clone()
//     }
// }

// impl Default for SurrealID {
//     fn default() -> Self {
//         todo!("SurrealID::default()");
//         // SurrealID::Thing(Thing::random())
//         // SurrealID { tb: "_".to_string(), id: Some(Id::rand()) }
//     }
// }

// impl Display for SurrealID {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             SurrealID::Thing(thing) => write!(f, "{}", thing),
//             SurrealID::TableId(tb, id) => write!(f, "{}:{}", tb, id),
//             SurrealID::Id(id) => write!(f, "{}", id),
//             SurrealID::Table(tb) => write!(f, "{}", tb),
//         }
//     }
// }

// impl From<Thing> for SurrealID {
//     fn from(thing: Thing) -> Self {
//         SurrealID::Thing(thing)
//     }
// }

// impl From<SurrealID> for Thing {
//     fn from(ident: SurrealID) -> Self {
//         todo!("SurrealID::from(SurrealID)");
//     }
// }

// impl From<(String, String)> for SurrealID {
//     fn from((tb, id): (String, String)) -> Self {
//         SurrealID::Thing(Thing::from((tb, id)))
//     }
// }

// impl From<(&str, &str)> for SurrealID {
//     fn from((tb, id): (&str, &str)) -> Self {
//         SurrealID::Thing(Thing::from((tb, id)))
//     }
// }

// impl From<(String, Id)> for SurrealID {
//     fn from((tb, id): (String, Id)) -> Self {
//         SurrealID::Thing(Thing::from((tb, id)))
//     }
// }

// impl From<(&str, Id)> for SurrealID {
//     fn from((tb, id): (&str, Id)) -> Self {
//         SurrealID::Thing(Thing::from((tb, id)))
//     }
// }

// impl FromStr for SurrealID {
//     type Err = ();
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         todo!("SurrealID::from_str");
//         // let id: Thing = s.parse().unwrap();
//         // Ok(SurrealID(id))
//     }
// }

// // impl &Ident {
// //     pub fn as_thing(&self) -> &Thing {
// //         &self.id
// //     }
// // }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_ident() {
//         let random = Thing::from(("random", "1"));
//         let ident: SurrealID = random.into();
//         println!("{:#?}", ident);
//     }
// }
