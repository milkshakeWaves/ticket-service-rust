use super::models::{User, InternalUser};

pub mod db_context;
mod user_dao;

pub type Database<'c> = db_context::Database<'c>;
pub type Table<'c, T> = db_context::Table<'c, T>;
pub type JoinTable<'c, T1, T2> = db_context::JoinTable<'c, T1, T2>;