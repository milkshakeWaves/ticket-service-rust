use super::models::{User, InternalUser};
use super::models::Event;
use super::models::Ticket;
use super::models::SoldTicketPerUser;

pub mod db_context;
mod user_dao;
mod event_dao;
mod ticket_dao;

pub type Database<'c> = db_context::Database<'c>;
pub type Table<'c, T> = db_context::Table<'c, T>;
pub type JoinTable<'c, T1, T2> = db_context::JoinTable<'c, T1, T2>;