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
use crate::parseutil::*;
use csv;
use serde::{de, Deserialize, Deserializer};
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::str::FromStr;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Record {
    #[serde(rename = "FIPS")]
    pub fips: String,
    #[serde(rename = "Admin2")]
    pub county: String,
    #[serde(rename = "Province_State")]
    pub state: String,
    #[serde(rename = "Country_Region")]
    pub country: String,
    #[serde(rename = "Last_Update")]
    pub last_update: String,
    #[serde(rename = "Lat")]
    pub lat: String,
    #[serde(rename = "Long_")]
    pub long: String,
    #[serde(rename = "Confirmed")]
    pub cases: i32,
    #[serde(rename = "Deaths")]
    pub deaths: i32,
    #[serde(rename = "Recovered")]
    pub recovered: i32,
    #[serde(rename = "Active")]
    pub active: i32,
    #[serde(rename = "Combined_Key")]
    pub combined_key: String,
    #[serde(rename = "Incidence_Rate")]
    pub incidence_rate: f64,
    #[serde(rename = "Case-Fatality_Ratio")]
    pub case_fatality_ratio: f64,

}

/// Convert to (County, ARecord) tuple.
pub fn struct_to_arecord(date: NaiveDate, rec: Option<Record>) -> Option<ARecord> {
    match rec {
        Some(r) =>
            Some(ARecord { state: Some(r.state),
                           county: Some(r.county),
                           date: Some(date),
                           totalcases: r.cases,
                           totaldeaths: r.deaths,
                           totalrecovered: r.recovered,
                           totalactive: r.active,
                           incidence_rate: r.incidence_rate,
                           case_fatality_ratio: r.case_fatality_ratio,
                           ..ARecord::default()}),
        None => None,
    }
}

pub fn parse_to_final<'a, A: 'a + Iterator<Item = csv::StringRecord>>(
    date: &'a NaiveDate,
    striter: A,
) -> impl Iterator<Item = ARecord> + 'a {
    striter.filter_map(move |x| struct_to_arecord(date.clone(), rec_to_struct(&x)))
}

/* Will panic on parse error.  */
pub fn parse_init<'a>(
    date: &NaiveDate,
    base_dir: &str,
) -> csv::ByteRecordsIter<'a, File> {
    let filename = format!("{}/{}.csv", base_dir, date.format("%m-%d-%Y"));
    let mut rdr = parse_init_file(filename).expect("Couldn't init parser")
}

/* Will panic on parse error.  */
pub fn parse<'a>(
    base_dir: &'a str,
    datelist: &'a Vec<NaiveDate>)
 -> impl Iterator<Item = ARecord> + 'a {

    datelist.iter().flat_map(move |date| {
        parse_to_final(date, parse_records(parse_init(date, base_dir).byte_records()))
    })


}
