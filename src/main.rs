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
use sqlx::prelude::*;
use sqlx::sqlite::SqlitePool;
use covid19db::dateutil::*;

mod counties;
mod analysis;
mod arecord;
mod charts;
mod db;

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_nth_arg(arg: usize) -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(arg) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

#[tokio::main]
async fn main() {
    let first_date = NaiveDate::from_ymd(2020, 7, 12);
    let last_date = NaiveDate::from_ymd(2020, 8, 3);

    let data_first_date = NaiveDate::from_ymd(2020, 5, 29);
    let data_last_date = NaiveDate::from_ymd(2020, 8, 9);

    // Source: https://www.kansas.com/news/politics-government/article244091222.html
    let maskcounties = counties::Counties::new(vec![
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
    ]);

    let filename = match get_nth_arg(1) {
        Ok(x) => x.to_str().unwrap(),
        Err(_) => {
            println!("Database file not specified; trying covid19.db in current directory");
           "covid19.db"
        } };


    let mut inputpool = SqlitePool::builder()
        .max_size(5)
        .build(format!("sqlite::{}", filename).as_ref())
        .await
        .expect("Error building");

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
