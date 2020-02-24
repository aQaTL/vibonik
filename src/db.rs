#![allow(dead_code)]
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
	id: i32,
	fb_id: String,
	access_token: Option<String>,
	login: Option<String>,
	password_hash: Option<String>,
	role: Option<String>,
	name: String,
	pesel: Option<i64>,
	email: Option<String>,
	birthday: Option<chrono::NaiveDate>,
	gender: Option<String>,
	food_preferences: Option<String>,
	paid: i64,
}