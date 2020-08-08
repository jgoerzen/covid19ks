/* Parser

Copyright (c) 2019-2020 John Goerzen

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

use chrono;
use chrono::naive::NaiveDate;
use crate::arecord::ARecord;
pub use crate::parseutil::*;
use csv;
use serde::{de, Deserialize, Deserializer};
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::str::FromStr;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Record {
    #[serde(rename = "Date", deserialize_with = "crate::parseutil::date_from_str")]
    pub date: NaiveDate,
    #[serde(rename = "County")]
    pub county: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "FIPS")]
    pub fips: String,
    #[serde(rename = "Cases")]
    pub cases: i32,
    #[serde(rename = "Deaths")]
    pub deaths: i32,
}

/// Convert to (County, ARecord) tuple.
pub fn struct_to_arecord(rec: Option<Record>) -> Option<ARecord> {
    match rec {
        Some(r) =>
            Some(ARecord { state: Some(r.state),
                           county: Some(r.county),
                           date: Some(r.date),
                           totalcases: r.cases,
                           totaldeaths: r.deaths,
                           ..ARecord::default()}),
        None => None,
    }
}

pub fn parse_to_final<A: Iterator<Item = csv::StringRecord>>(
    striter: A,
) -> impl Iterator<Item = ARecord> {
    striter.filter_map(|x| struct_to_arecord(rec_to_struct(x)))
}

/* Will panic on parse error.  */
pub fn parse<'a, A: std::io::Read>(
    file_path: String,
) -> impl Iterator<Item = ARecord> + 'a {
    let mut rdr = parse_init_file(file_path).expect("Couldn't init parser");
    let recs = parse_records(rdr.byte_records());
    parse_to_final(recs)
}
