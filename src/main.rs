/*

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

use chrono::naive::NaiveDate;
use std::env;
use std::error::Error;
use std::ffi::OsString;

mod analysis;
mod charts;
mod parser;

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
fn main() {
    let first_date = NaiveDate::from_ymd(2020, 7, 12);
    let last_date = NaiveDate::from_ymd(2020, 8, 3);

    // Source: https://www.kansas.com/news/politics-government/article244091222.html
    let maskcounties = vec![
        "Jewell",
        "Mitchell",
        "Saline",
        "Dickinson",
        "Atchison",
        "Douglas",
        "Johnson",
        "Wyandotte",
        "Franklin",
        "Allen",
        "Bourbon",
        "Crawford",
        "Montgomery",
    ];
    let datelist_output = analysis::alldates(&first_date, &last_date);
    let datelist_full = analysis::alldates(
        &NaiveDate::from_ymd(2020, 1, 21),
        &NaiveDate::from_ymd(2020, 8, 6),
    );

    let file_path = get_first_arg()
        .expect("need args")
        .into_string()
        .expect("conversion issue");
    let mut rdr = parser::parse_init_file(file_path).expect("Couldn't init parser");
    let vr = parser::parse(&mut rdr);
    let filtered = vr.filter(|x| x.state == "Kansas");
    let bycounty = analysis::parser_to_county(filtered, &datelist_full, 7);

    let (masks, nomasks) = analysis::separate(&bycounty, &maskcounties, &datelist_full, 7);
    charts::write(&masks, &nomasks, &datelist_output);
    charts::writecounties(
        &bycounty,
        &vec!["Marion", "McPherson", "Harvey", "Saline"],
        &datelist_output,
    );
}
