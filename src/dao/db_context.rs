use super::{User, Event};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool};
use std::marker::PhantomData;
use std::sync::Arc;

pub struct Table<'c, T>
where
    T: FromRow<'c, PgRow>,
{
    pub pool: Arc<PgPool>,
    _from_row: fn(&'c PgRow) -> Result<T, sqlx::Error>,
    _marker: PhantomData<&'c T>,
}

impl<'c, T> Table<'c, T>
where
    T: FromRow<'c, PgRow>,
{
    fn new(pool: Arc<PgPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
            _marker: PhantomData,
        }
    }
}

pub struct JoinTable<'c, T1, T2>
where
    T1: FromRow<'c, PgRow>,
    T2: FromRow<'c, PgRow>,
{
    pub pool: Arc<PgPool>,
    _from_row: (
        fn(&'c PgRow) -> Result<T1, sqlx::Error>,
        fn(&'c PgRow) -> Result<T2, sqlx::Error>,
    ),
    _marker_t1: PhantomData<&'c T1>,
    _marker_t2: PhantomData<&'c T2>,
}

impl<'c, T1, T2> JoinTable<'c, T1, T2>
where
    T1: FromRow<'c, PgRow>,
    T2: FromRow<'c, PgRow>,
{
    fn new(pool: Arc<PgPool>) -> Self {
        JoinTable {
            pool,
            _from_row: (T1::from_row, T2::from_row),
            _marker_t1: PhantomData,
            _marker_t2: PhantomData,
        }
    }
}

pub struct Database<'c> {
    pub users: Arc<Table<'c, User>>,
    pub events: Arc<Table<'c, Event>>
}

impl<'a> Database<'a> {
    pub async fn new(sql_url: &str) -> Database<'a> {
        let connection = PgPool::connect(&sql_url).await.unwrap();
        let pool = Arc::new(connection);

        Database {
            users: Arc::from(Table::new(pool.clone())),
            events: Arc::from(Table::new(pool.clone()))
        }
    }
}
