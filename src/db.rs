/* Database tools

Copyright (c) 2020 John Goerzen

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use covid19db::dateutil::*;
use chrono::NaiveDate;
use sqlx::prelude::*;
use sqlx::sqlite::SqliteRow;
use crate::counties::Counties;

pub struct DB<'a> {
    pub pool: &'a mut sqlx::SqlitePool,
    pub maskcounties: Counties<'a>,
}

impl<'a> DB<'a> {
    pub fn new(pool: &'a mut sqlx::SqlitePool, maskcounties: Counties<'a>) -> DB<'a> {
        DB{pool, maskcounties}
    }

    pub async fn query_as<T: Send + for<'d> FromRow<'d, SqliteRow<'d>>>(&self, query: &str) -> Vec<T> {
        sqlx::query_as::<_, T>(query)
            .fetch_all(&mut self.pool.acquire().await.unwrap()).await.unwrap()
    }

    /// Takes a SELECT statement ending in WHERE, inserts masks clause between the first and second parts
    pub async fn query_as_masks<T: Send + for<'d> FromRow<'d, SqliteRow<'d>>>(&self, query: &str, query2: &str) -> Vec<T> {
        self.query_as(format!("{} in ({}) {}", query, self.maskcounties.sqlclause(), query2).as_str()).await
    }
}
