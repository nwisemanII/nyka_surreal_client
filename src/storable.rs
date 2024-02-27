
use std::fmt::Display;
use std::pin::Pin;
use std::fmt::Debug;
use std::ops::Deref;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::RecordIdData;
use crate::ident::SurrealData;
use crate::ident::HasSurrealIdentifier;
use crate::ident::SurrealIDFactory;
use crate::ident::SurrealIDIdent;
use crate::ident::SurrealIDTable;
use crate::prelude::*;
use crate::Error;

// impl<T: DBThings + SurrealIDIdent + SurrealIDTable> From<T> for Record<T> {
//     fn from(data: T) -> Record<T> {
//         Record::RecordIdData(
//             RecordIdData::new(
//                 data.table().as_str(), 
//                 Some(Id::from(data.id())),
//                 data
//             ))
//     }
// }
impl<'a, T: Storable> From<T> for Record<T> {
    fn from(storable: T) -> Self {
        let id = (&storable).id();
        let table = (&storable).table();
        let data = storable.data();
        let record: Record<T> = Record::new(table.as_str(), id.as_str(), Some(Box::new(data)));
        record
    }
}

#[async_trait]
pub trait Storable
where 
    Self: Sized + DBThings + HasSurrealIdentifier + SurrealData + From<Record<Self>>
    // T: HasSurrealIdentifier + SurrealData + DBThings,
{
    async fn save(self) -> Result<Vec<Self>, Error> {
        let _ = connect(None).await.ok();

        let ret: Vec<Self> = create_record(
            Record::new(
                self.table().as_str(), 
                self.id().as_str(), 
                Some(Box::new(self))
            )
        ).await.expect("Whoops");
        Ok(ret)
    }

    async fn select(&self) -> Result<Option<Record<Self>>, Error> {
        let _ = connect(None).await.ok();
        let rec: Option<Record<Self>> = get_record(self.table().as_str(), self.id().as_str()).await?;
        Ok(rec)
    }

    async fn delete(&self) -> Result<Pin<Box<Self>>, Error> {
        let _ = connect(None).await.ok();
        let rec: Result<Self, Error> = delete_record(self.table().as_str(), self.id().as_str()).await;
        match rec {
            Ok(rec) => Ok(Box::pin(rec)),
            Err(e) => Err(e),
        }
    }
}

pub trait DBThings: Debug + Serialize + DeserializeOwned + Sized + Clone {}

pub trait StorableId<T>: DBThings
where
    Self: HasSurrealIdentifier + SurrealData,
    T: HasSurrealIdentifier + SurrealData,
{
    type Item: DBThings;

    // fn to_record<T>(self) -> Record<T>
    // where
    // {
    //     let record: Record<T> = Record::new(self.table().as_str(), self.id().as_str(), Some(self.data()));
    //     record
    // }

    fn as_thing(&self) -> Thing
    where
    {
        Thing::from((self.table(), self.id()))
    }
}