use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::*;


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub session_id: String,
}


// By default, it will look for the plural of the struct (i.e., users), so
// must specify manually here.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

