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

use csv;
use std::error::Error;
use std::fs::File;
use serde::{de, Deserialize, Deserializer};
use chrono;
use chrono::naive::NaiveDate;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Record {
    #[serde(rename = "Date", deserialize_with = "date_from_str")]
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

fn date_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,      // Required for S::from_str...
    S::Err: Display, // Required for .map_err(de::Error::custom)
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}

/* The input data is a bunch of 1-column notes at the end, citation details, etc.
These won't parse as a record, so just discard them. */
pub fn rec_to_struct(record: csv::StringRecord) -> Option<Record> {
    if record.len() != 1 {
        let rec: Record = record.deserialize(None).expect("rec_to_struct");
        return Some(rec);
    }
    return None;
}

pub fn parse_init_file(filename: String) -> Result<csv::Reader<File>, Box<Error>> {
    let file = File::open(filename)?;
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .flexible(true)
        .from_reader(file);
    Ok(rdr)
}

/* 

This type signature with hints from https://stackoverflow.com/questions/27535289/what-is-the-correct-way-to-return-an-iterator-or-any-other-trait
*/
pub fn parse_records<'a, A: std::io::Read>(byteiter: csv::ByteRecordsIter<'a, A>) -> impl Iterator<Item = csv::StringRecord> + 'a {
    byteiter.map(|x| csv::StringRecord::from_byte_record_lossy(x.expect("Error in parse_records")))
}


pub fn parse_to_final<A: Iterator<Item = csv::StringRecord>>(striter: A) -> impl Iterator<Item = Record> {
    striter.filter_map(|x| rec_to_struct(x))
}

/* Will panic on parse error. */
pub fn parse<'a, A: std::io::Read>(rdr: &'a mut csv::Reader<A>) -> impl Iterator<Item = Record> + 'a {
    let recs = parse_records(rdr.byte_records());
    parse_to_final(recs)
}

