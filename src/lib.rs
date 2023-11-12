use std::sync::{Mutex, Arc};
use crate::dao::Database;

pub mod dao;
pub mod models;
pub mod service;
pub mod crypto;

pub struct AppState<'a> {
    pub connections: Mutex<u32>,
    pub context: Arc<Database<'a>>,
}
