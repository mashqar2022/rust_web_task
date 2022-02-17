use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Group {
    pub id: i64,
    pub name: String,
}

impl<'c> FromRow<'c, SqliteRow<'c>> for Group {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Group {
            id: row.get(0),
            name: row.get(1),
        })
    }
}
