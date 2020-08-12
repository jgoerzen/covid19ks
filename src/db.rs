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

use crate::counties::Counties;
use sqlx::prelude::*;
use std::collections::HashMap;

pub fn makemasksstr(
    dataset: &str,
    mainfield: &str,
    masks: bool,
    counties: &Counties<'_>,
) -> String {
    format!(
        "SELECT date_julian, SUM({}) FROM cdataset
            WHERE dataset = '{}' AND province = 'Kansas'
                  AND date_julian >= ? AND date_julian <= ? AND administrative {} IN {}
            GROUP BY date_julian ORDER BY date_julian",
        mainfield,
        dataset,
        if masks { "" } else { "not" },
        counties.sqlclause().as_str()
    )
}

pub fn makemasks100kstr(
    dataset: &str,
    mainfield: &str,
    masks: bool,
    counties: &Counties<'_>,
) -> String {
    format!(
        "SELECT date_julian, 100000 * SUM({}) / SUM(factbook_population) FROM cdataset
            WHERE dataset = '{}' AND province = 'Kansas'
                  AND date_julian >= ? AND date_julian <= ? AND administrative {} IN {}
            GROUP BY date_julian ORDER BY date_julian",
        mainfield,
        dataset,
        if masks { "" } else { "not" },
        counties.sqlclause().as_str()
    )
}

/// Read in the summarized data for mask or no-mask counties, returning a HashMap from date_julian to given field
pub async fn getmaskdata(
    pool: &sqlx::SqlitePool,
    dataset: &str,
    field: &str,
    masks: bool,
    counties: &Counties<'_>,
    first_date: i32,
    last_date: i32,
) -> HashMap<i32, f64> {
    let query = makemasksstr(dataset, field, masks, counties);
    println!("{}", query);
    sqlx::query_as::<_, (i32, i64)>(query.as_str())
        .bind(first_date)
        .bind(last_date)
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .map(|(x, y)| (x, y as f64))
        .collect()
}

/// Read in the summarized data for mask or no-mask counties, returning a HashMap from date_julian to given field
pub async fn getmask100kdata(
    pool: &sqlx::SqlitePool,
    dataset: &str,
    field: &str,
    masks: bool,
    counties: &Counties<'_>,
    first_date: i32,
    last_date: i32,
) -> HashMap<i32, f64> {
    let query = makemasks100kstr(dataset, field, masks, counties);
    println!("{}", query);
    sqlx::query_as::<_, (i32, i64)>(query.as_str())
        .bind(first_date)
        .bind(last_date)
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .map(|(x, y)| (x, y as f64))
        .collect()
}

/// Read in the summarized data per-county, returning a HashMap of counties to a HashMap from date_julian to given field
pub async fn getcountymaskdata(
    pool: &sqlx::SqlitePool,
    dataset: &str,
    field: &str,
    first_date: i32,
    last_date: i32,
) -> HashMap<String, HashMap<i32, f64>> {
    let query = format!(
        "SELECT administrative, date_julian, SUM({}) FROM cdataset
            WHERE dataset = '{}' AND province = 'Kansas'
                  AND date_julian >= ? AND date_julian <= ?  AND administrative IS NOT NULL
            GROUP BY date_julian, administrative ORDER BY administrative, date_julian",
        field, dataset
    );
    let mut hm = HashMap::new();
    println!("{}", query);
    sqlx::query_as::<_, (String, i32, i64)>(query.as_str())
        .bind(first_date)
        .bind(last_date)
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .for_each(|(county, x, y)| {
            hm.entry(county)
                .or_insert(HashMap::new())
                .insert(x, y as f64);
        });
    hm
}
