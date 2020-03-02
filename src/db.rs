#![allow(dead_code)]
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use std::io::Write;
use uuid::Uuid;

use crate::schema::users;

#[derive(Queryable, Insertable, Serialize, Deserialize, Default)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
pub struct User {
	pub id: i32,
	pub fb_id: String,
	pub access_token: Option<String>,
	pub uuid: Uuid,
	pub login: Option<String>,
	pub password_hash: Option<String>,
	pub role: Option<Role>,
	pub name: String,
	pub pesel: Option<i64>,
	pub email: Option<String>,
	pub birthday: Option<chrono::NaiveDate>,
	pub gender: Option<String>,
	pub food_preferences: Option<String>,
	pub paid: i64,
}

#[derive(Insertable, Default)]
#[table_name = "users"]
pub struct NewUser<'a> {
	pub fb_id: &'a str,
	pub access_token: Option<&'a str>,
	pub login: Option<&'a str>,
	pub uuid: Uuid,
	pub password_hash: Option<&'a str>,
	pub role: Option<Role>,
	pub name: &'a str,
	pub pesel: Option<i64>,
	pub email: Option<&'a str>,
	pub birthday: Option<&'a chrono::NaiveDate>,
	pub gender: Option<&'a str>,
	pub food_preferences: Option<&'a str>,
	pub paid: i64,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub enum Role {
	User,
	Mod,
	Root,
	DJ,
	Security,
}

impl ToSql<Text, Pg> for Role {
	fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
		write!(out, "{:?}", self)?;
		Ok(IsNull::No)
	}
}
impl FromSql<Text, Pg> for Role {
	fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
		match not_none!(bytes) {
			b"User" => Ok(Role::User),
			b"Mod" => Ok(Role::Mod),
			b"Root" => Ok(Role::Root),
			b"DJ" => Ok(Role::DJ),
			b"Security" => Ok(Role::Security),
			_ => Err("Unrecognized enum variant".into()),
		}
	}
}
