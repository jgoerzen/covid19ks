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
mod arecord;
mod charts;
mod jhuparser;
mod nytparser;
mod parseutil;

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
    let datelist_full = analysis::alldates(&data_first_date, &data_last_date);
    let datelist_updated = analysis::alldates(&first_date, &data_last_date);

    // NYT-based data
    let file_path = get_nth_arg(1)
        .expect("need args: path-to-us-counties.csv path-to-csse_covid_19_daily_reports")
        .into_string()
        .expect("conversion issue");
    let mut rdr = parseutil::parse_init_file(file_path).expect("Couldn't init parser");
    let nytvr = nytparser::parse(&mut rdr);
    let nytfiltered = nytvr.filter(|x| x.state == Some(String::from("Kansas")));
    let nytbycounty = analysis::parser_to_county(nytfiltered, &datelist_full, 7);

    let (nytmasks, nytnomasks) = analysis::separate(&nytbycounty, &maskcounties, &datelist_full, 7);

    // JHU-based data
    let base_dir = get_nth_arg(2)
        .expect("need args: path-to-us-counties.csv path-to-csse_covid_19_daily_reports")
        .into_string()
        .expect("conversion issue");
    let jhuvr = jhuparser::parse(&base_dir, &datelist_full);
    let jhufiltered = jhuvr.filter(|x| x.state == Some(String::from("Kansas")));
    let jhubycounty = analysis::parser_to_county(jhufiltered, &datelist_full, 7);
    let (jhumasks, jhunomasks) = analysis::separate(&jhubycounty, &maskcounties, &datelist_full, 7);

    charts::write(
        "main.png",
        arecord::ARecord::getnewcaseavg,
        "COVID-19: Masks vs no-mask counties, KS",
        "7-day moving average of new cases, % relative to July 12",
        60f64,
        130f64,
        &nytmasks,
        &nytnomasks,
        &datelist_output,
    );
    charts::write(
        "images/main-jhu.png",
        arecord::ARecord::getnewcaseavg,
        "COVID-19: Masks vs no-mask counties, KS",
        "7-day moving average of new cases, % relative to July 12",
        60f64,
        150f64,
        &jhumasks,
        &jhunomasks,
        &datelist_output,
    );
    charts::write(
        "images/main-updated-nyt.png",
        arecord::ARecord::getnewcaseavg,
        "COVID-19: Masks vs no-mask counties, KS",
        "7-day moving average of new cases, % relative to July 12",
        60f64,
        130f64,
        &nytmasks,
        &nytnomasks,
        &datelist_updated,
    );
    charts::write(
        "images/main-updated-jhu.png",
        arecord::ARecord::getnewcaseavg,
        "COVID-19: Masks vs no-mask counties, KS",
        "7-day moving average of new cases, % relative to July 12",
        60f64,
        150f64,
        &jhumasks,
        &jhunomasks,
        &datelist_updated,
    );
    charts::write(
        "images/deaths-nyt.png",
        arecord::ARecord::getnewdeathavg,
        "COVID-19 deaths: Mask vs no-mask",
        "7-day moving average of new deaths, % relative to July 12",
        20f64,
        400f64,
        &nytmasks,
        &nytnomasks,
        &datelist_output,
    );
    charts::write(
        "images/deaths-jhu.png",
        arecord::ARecord::getnewdeathavg,
        "COVID-19 deaths: Mask vs no-mask",
        "7-day moving average of new deaths, % relative to July 12",
        20f64,
        400f64,
        &jhumasks,
        &jhunomasks,
        &datelist_output,
    );
    charts::write(
        "images/deaths-updated-nyt.png",
        arecord::ARecord::getnewdeathavg,
        "COVID-19 deaths: Mask vs no-mask",
        "7-day moving average of new deaths, % relative to July 12",
        20f64,
        400f64,
        &nytmasks,
        &nytnomasks,
        &datelist_updated,
    );
    charts::write(
        "images/deaths-updated-jhu.png",
        arecord::ARecord::getnewdeathavg,
        "COVID-19 deaths: Mask vs no-mask",
        "7-day moving average of new deaths, % relative to July 12",
        20f64,
        400f64,
        &jhumasks,
        &jhunomasks,
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
        &nytbycounty,
        &datelist_output,
    );
    charts::writecounties(
        "images/counties-jhu.png",
        arecord::ARecord::getnewcaseavg,
        "COVID-19 cases in Selected Counties, Kansas",
        "7-day moving average of new cases, % relative to July 12",
        10f64,
        300f64,
        &vec!["Marion", "McPherson", "Harvey", "Saline"],
        &jhubycounty,
        &datelist_output,
    );
    charts::writecounties(
        "images/counties-updated-nyt.png",
        arecord::ARecord::getnewcaseavg,
        "COVID-19 cases in Selected Counties, Kansas",
        "7-day moving average of new cases, % relative to July 12",
        20f64,
        200f64,
        &vec!["Marion", "McPherson", "Harvey", "Saline"],
        &nytbycounty,
        &datelist_updated,
    );
    charts::writecounties(
        "images/counties-updated-jhu.png",
        arecord::ARecord::getnewcaseavg,
        "COVID-19 cases in Selected Counties, Kansas",
        "7-day moving average of new cases, % relative to July 12",
        0f64,
        400f64,
        &vec!["Marion", "McPherson", "Harvey", "Saline"],
        &jhubycounty,
        &datelist_updated,
    );
}
