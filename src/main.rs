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
mod nytparser;
mod arecord;
mod parseutil;
mod jhuparser;

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_nth_arg(arg: usize) -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(arg) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
fn main() {
    let first_date = NaiveDate::from_ymd(2020, 7, 12);
    let last_date = NaiveDate::from_ymd(2020, 8, 3);

    let data_first_date = NaiveDate::from_ymd(2020, 5, 29);
    let data_last_date = NaiveDate::from_ymd(2020, 8, 7);

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
        &data_first_date,
        &data_last_date,
    );
    let datelist_updated = analysis::alldates(&first_date, &data_last_date);

    let file_path = get_nth_arg(1)
        .expect("need args")
        .into_string()
        .expect("conversion issue");
    let vr = nytparser::parse(file_path);
    let filtered = vr.filter(|x| x.state == Some(String::from("Kansas")));
    let bycounty = analysis::parser_to_county(filtered, &datelist_full, 7);

    let (masks, nomasks) = analysis::separate(&bycounty, &maskcounties, &datelist_full, 7);
    charts::write(
        "main.png",
        arecord::ARecord::getnewcaseavg,
        "COVID-19: Masks vs no-mask counties, KS",
        "7-day moving average of new cases, % relative to July 12",
        60f64,
        130f64,
        &masks,
        &nomasks,
        &datelist_output,
    );
    charts::write(
        "main-updated.png",
        arecord::ARecord::getnewcaseavg,
        "COVID-19: Masks vs no-mask counties, KS",
        "7-day moving average of new cases, % relative to July 12",
        60f64,
        130f64,
        &masks,
        &nomasks,
        &datelist_updated,
    );
    charts::write(
        "deaths.png",
        arecord::ARecord::getnewdeathavg,
        "COVID-19 deaths: Mask vs no-mask",
        "7-day moving average of new deaths, % relative to July 12",
        20f64,
        400f64,
        &masks,
        &nomasks,
        &datelist_output,
    );
    charts::write(
        "deaths-updated.png",
        arecord::ARecord::getnewdeathavg,
        "COVID-19 deaths: Mask vs no-mask",
        "7-day moving average of new deaths, % relative to July 12",
        20f64,
        400f64,
        &masks,
        &nomasks,
        &datelist_updated,
    );
    charts::writecounties(
        "counties.png",
        arecord::ARecord::getnewcaseavg,
        "COVID-19 cases in Selected Counties, Kansas",
        "7-day moving average of new cases, % relative to July 12",
        20f64,
        200f64,
        &vec!["Marion", "McPherson", "Harvey", "Saline"],
        &bycounty,
        &datelist_output,
    );
    charts::writecounties(
        "counties-updated.png",
        arecord::ARecord::getnewcaseavg,
        "COVID-19 cases in Selected Counties, Kansas",
        "7-day moving average of new cases, % relative to July 12",
        20f64,
        200f64,
        &vec!["Marion", "McPherson", "Harvey", "Saline"],
        &bycounty,
        &datelist_updated,
    );
}
