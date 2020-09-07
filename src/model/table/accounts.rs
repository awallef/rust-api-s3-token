use super::super::schema::accounts;

use diesel;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Account
{
    pub id: u32,
    pub account_type_id: u32,
    pub name: String,
    pub slug: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub old_id: Option<String>,
}

pub fn all(connection: &MysqlConnection) -> QueryResult<Vec<Account>> {
    accounts::table.load::<Account>(&*connection)
}
