use super::{Group, User};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, SqlitePool};
use std::sync::Arc;

pub struct Table<'c, T>
where
    T: FromRow<'c, SqliteRow<'c>>,
{
    pub pool: Arc<SqlitePool>,
    _from_row: fn(&SqliteRow<'c>) -> Result<T, sqlx::Error>,
}

impl<'c, T> Table<'c, T>
where
    T: FromRow<'c, SqliteRow<'c>>,
{
    fn new(pool: Arc<SqlitePool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
        }
    }
}

pub struct JoinTable<'c, T1, T2>
where
    T1: FromRow<'c, SqliteRow<'c>>,
    T2: FromRow<'c, SqliteRow<'c>>,
{
    pub pool: Arc<SqlitePool>,
    _from_row: (
        fn(&SqliteRow<'c>) -> Result<T1, sqlx::Error>,
        fn(&SqliteRow<'c>) -> Result<T2, sqlx::Error>,
    ),
}

impl<'c, T1, T2> JoinTable<'c, T1, T2>
where
    T1: FromRow<'c, SqliteRow<'c>>,
    T2: FromRow<'c, SqliteRow<'c>>,
{
    fn new(pool: Arc<SqlitePool>) -> Self {
        JoinTable {
            pool,
            _from_row: (T1::from_row, T2::from_row),
        }
    }
}

pub struct Database<'c> {
    pub users: Arc<Table<'c, User>>,
    pub groups: Arc<Table<'c, Group>>,
    pub users_to_groups: Arc<JoinTable<'c, User, Group>>,
}

impl Database<'_> {
    pub async fn new(sql_url: &str) -> Database<'_> {
        let pool = SqlitePool::new(sql_url).await.unwrap();
        let pool = Arc::new(pool);

        Database {
            users: Arc::from(Table::new(pool.clone())),
            groups: Arc::from(Table::new(pool.clone())),
            users_to_groups: Arc::from(JoinTable::new(pool.clone())),
        }
    }
}
