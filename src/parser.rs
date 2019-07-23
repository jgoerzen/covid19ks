/* Parser

Copyright (c) 2019 John Goerzen

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
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Record {
    #[serde(rename = "Notes")]
    pub notes: String,
    #[serde(rename = "ICD Chapter")]
    pub chapter: String,
    #[serde(rename = "ICD Chapter Code")]
    pub chaptercode: String,
    #[serde(rename = "ICD Sub-Chapter")]
    pub subchapter: String,
    #[serde(rename = "ICD Sub-Chapter Code")]
    pub subchaptercode: String,
    #[serde(rename = "Cause of death")]
    pub causeofdeath: String,
    #[serde(rename = "Cause of death Code")]
    pub causeofdeathcode: String,
    #[serde(rename = "Deaths")]
    pub deaths: i64
}

/* The input data is a bunch of 1-column notes at the end, citation details, etc.
These won't parse as a record, so just discard them. */
pub fn rec_to_struct(record: csv::StringRecord) -> Option<Record> {
    if record.len() != 1 {
        let rec: Record = record.deserialize(None).expect("rec_to_struct");
        // Keep the periodic "Total" lines out of the output.
        if rec.notes != "Total" {
            return Some(rec);
        }
    }
    return None;
}

pub fn parse_init_file(filename: String) -> Result<csv::Reader<File>, Box<Error>> {
    let file = File::open(filename)?;
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
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

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_parse_records() {
        let mut rdr = parse_init_file(String::from("srcdata.tsv")).expect("open");
        let byterecs = rdr.byte_records();
        let recs = parse_records(byterecs);
        let v: Vec<csv::StringRecord> = recs.collect();
        assert_eq!(2568, v.len());
        assert_eq!(String::from(&v[0][1]), "Certain infectious and parasitic diseases");
    }

    #[test]
    fn test_parse_to_final() {
        let mut rdr = parse_init_file(String::from("srcdata.tsv")).expect("open");
        let vr: Vec<Record> = parse(&mut rdr).collect();

        // grep -P '\t' srcdata.tsv | grep -v '^"Total' | wc -l   (-1 for hearder)
        assert_eq!(2348, vr.len());
        assert_eq!(vr[0].deaths, 1);
        assert_eq!(vr[1].causeofdeathcode, "A03.9");
    }

    #[test]
    #[should_panic]
    fn test_parse_error() {
        let mut rdr = parse_init_file(String::from("baddata.tsv")).expect("open");
        let recs = parse(&mut rdr);
        let _: Vec<Record> = recs.collect();
        
    }
}
