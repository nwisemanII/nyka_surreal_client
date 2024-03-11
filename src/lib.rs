mod config;
mod creds;
mod deserialize_id;
mod error;
mod ident;
mod record;
mod storable;

pub use config::{setup, DbConfig};
pub use deserialize_id::deserialize_id;
pub use error::Error;
pub use ident::SurrealId;
use once_cell::sync::Lazy;
pub use record::Record;
pub use serde::{Deserialize, Serialize};
pub use storable::{DBThings, Storable};
use surrealdb::opt::auth::Root;
use surrealdb::{engine::any::Any, opt::auth::Jwt, Response, Surreal};

static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);

// static CONFIG: Lazy<config::DbConfig> = Lazy::new(config::setup);

pub mod prelude {
    pub use surrealdb::sql::Id;
    pub use surrealdb::sql::Thing;
    pub use surrealdb::sql::Value;
    pub use surrealdb::Error as SDBError;

    pub use super::{
        connect,
        create_record,
        // update_record,
        delete_record,
        deserialize_id,
        get_record,
        query,
        DBThings,
        Deserialize,
        Error,
        // HasSurrealIdentifier,
        Record,
        Serialize,
        Storable,
        // SurrealData,
        SurrealId,
        // SurrealIDIdent,
        // SurrealIDTable,
    };
}

pub async fn create_record<T>(record: Record<T>) -> Result<Vec<T>, Error>
where
    T: DBThings + From<Record<T>>,
{
    let _id = record.id();
    let table = record.table();

    if table == "_" {
        return Err(Error::TableNameUnset);
    }
    let data = record.data();

    if data.is_none() {
        return Err(Error::NoDataStored("No data stored in record".to_string()));
    }

    let created: Vec<T> = DB
        .create(table)
        .content(data.unwrap())
        .await?;
    Ok(created)
}

pub async fn updata_record<'a, T>(
    record: Record<T>,
) -> Result<Option<Record<T>>, Error>
where
    T: DBThings,
{
    let updated: Option<Record<T>>;

    let _id = record.id();
    let table = record.table();

    if table == "_" {
        return Err(Error::TableNameUnset);
    }

    if let Some(data) = record.data() {
        updated = DB.update((table, _id)).content(data).await?;
    } else {
        updated = DB.update((table, _id)).await?;
    }
    Ok(updated)
}

pub async fn get_record<T>(record: Record<T>) -> Result<Option<Record<T>>, Error>
// pub async fn get_record<T>(table: &str, id: Id) -> Result<Option<Record<T>>, Error>
where
    T: DBThings,
{
    let _id = record.id();
    let table = record.table();

    if table == "_" {
        return Err(Error::TableNameUnset);
    }

    println!(
        "Getting record: {:?}:{:?}",
        &table,
        &_id.to_raw().to_string()
    );

    DB.select((&table, _id.clone())) // Implement the IntoResource<T> trait for surrealdb::sql::Thing
        .await
        .map_err(|_e| Error::NoRecordFound {
            table: table.to_string(),
            id: _id.to_string(),
            id_raw: _id.to_raw(), // msg: e
        })
}

pub async fn delete_record<T>(record: Record<T>) -> Result<Option<T>, Error>
// pub async fn delete_record<T>(table: &str, id: Id) -> Result<Option<T>, Error>
where
    T: DBThings,
{
    let table = record.table();

    if table == "_" {
        return Err(Error::TableNameUnset);
    }

    let id = record.id();
    
    Ok(DB.delete((table, id)).await?)
}

pub async fn query(query: &str) -> Result<Response, Error> {
    let results: Response = DB.query(query).await?;
    Ok(results)
}

pub async fn connect<'a>(config: &'a config::DbConfig) -> Result<(), Error> {
    DB.connect(&config.path).await?;
    let _result = DB
        .signin(Root {
            username: &config.user,
            password: &config.secret,
        })
        .await?;

    DB.use_ns(&config.ns).use_db(&config.db).await?;
    Ok(())
}

struct DBGuard {
    _token: Jwt,
}
impl DBGuard {
    fn new(token: Jwt) -> Self {
        Self { _token: token }
    }

    fn token(self) -> Jwt {
        self._token
    }
}

// impl Drop for DBGuard {
//     fn drop(&mut self) {
//         // let _ = DB.invalidate();
//     }
// }

pub async fn close() -> Result<(), Error> {
    DB.invalidate().await?;
    Ok(())
}
