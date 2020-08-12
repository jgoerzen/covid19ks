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

use std::env;
use std::error::Error;
use std::ffi::OsString;
use sqlx::sqlite::SqlitePool;
use covid19db::dateutil::*;
use std::path::Path;
use chrono::Local;
use std::fs::File;

mod counties;
mod analysis;
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
    let first_date = ymd_to_day(2020, 7, 12);
    let last_date = ymd_to_day(2020, 8, 3);

    let data_first_date = ymd_to_day(2020, 5, 29);
    let data_last_date = dateutc_to_day(&datelocal_to_dateutc(&Local::today())) - 1;

    let _daterange_output = first_date..=last_date;
    let _daterange_full = data_first_date..=data_last_date;
    let _daterange_updated = first_date..data_last_date;

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

    let mut bightml = File::create("all.html").unwrap();

    let filename = match get_nth_arg(1) {
        Ok(x) => String::from(x.to_str().unwrap()),
        Err(_) => {
            println!("Database file not specified; trying covid19.db in current directory");
           String::from("covid19.db")
        } };


    if ! Path::new(filename.as_str()).exists() {
        panic!("{} does not exist; download or specify alternative path on command line", filename)
    }
    let pool = SqlitePool::builder()
        .max_size(5)
        .build(format!("sqlite::{}", filename).as_ref())
        .await
        .expect("Error building");

    let mut nytmasks = db::getmaskdata(&pool, "nytimes/us-counties", "delta_confirmed", true, &maskcounties, data_first_date, data_last_date).await;
    analysis::calcsimplema(&mut nytmasks, 7);
    let mut nytnomasks = db::getmaskdata(&pool, "nytimes/us-counties", "delta_confirmed", false, &maskcounties, data_first_date, data_last_date).await;
    analysis::calcsimplema(&mut nytnomasks, 7);
    let mut jhumasks = db::getmaskdata(&pool, "jhu/daily", "delta_confirmed", true, &maskcounties, data_first_date, data_last_date).await;
    analysis::calcsimplema(&mut jhumasks, 7);
    let mut jhunomasks = db::getmaskdata(&pool, "jhu/daily", "delta_confirmed", false, &maskcounties, data_first_date, data_last_date).await;
    analysis::calcsimplema(&mut jhunomasks, 7);

    charts::write(
        "images/main-nyt.html",
        &mut bightml,
        "COVID-19: Masks vs no-mask counties, KS (NYT)",
        "7-day moving average of new cases, % relative to July 12",
        &nytmasks,
        &nytnomasks,
        first_date,
        last_date
    );

    charts::write(
        "images/main-jhu.html",
        &mut bightml,
        "COVID-19: Masks vs no-mask counties, KS (JHU)",
        "7-day moving average of new cases, % relative to July 12",
        &jhumasks,
        &jhunomasks,
        first_date,
        last_date,
    );

    charts::write(
        "images/main-updated-nyt.html",
        &mut bightml,
        "COVID-19: Masks vs no-mask counties, KS (Updated NYT)",
        "7-day moving average of new cases, % relative to July 12",
        &nytmasks,
        &nytnomasks,
        first_date,
        data_last_date,
    );

    charts::write(
        "images/main-updated-jhu.html",
        &mut bightml,
        "COVID-19: Masks vs no-mask counties, KS (Updated JHU)",
        "7-day moving average of new cases, % relative to July 12",
        &jhumasks,
        &jhunomasks,
        first_date,
        data_last_date,
    );

    ////////////// Replace the in-ram data with the deaths data.

    let mut nytmasks = db::getmaskdata(&pool, "nytimes/us-counties", "delta_deaths", true, &maskcounties, data_first_date, data_last_date).await;
    analysis::calcsimplema(&mut nytmasks, 7);
    let mut nytnomasks = db::getmaskdata(&pool, "nytimes/us-counties", "delta_deaths", false, &maskcounties, data_first_date, data_last_date).await;
    analysis::calcsimplema(&mut nytnomasks, 7);
    let mut jhumasks = db::getmaskdata(&pool, "jhu/daily", "delta_deaths", true, &maskcounties, data_first_date, data_last_date).await;
    analysis::calcsimplema(&mut jhumasks, 7);
    let mut jhunomasks = db::getmaskdata(&pool, "jhu/daily", "delta_deaths", false, &maskcounties, data_first_date, data_last_date).await;
    analysis::calcsimplema(&mut jhunomasks, 7);

    charts::write(
        "images/deaths-nyt.html",
        &mut bightml,
        "COVID-19 deaths: Mask vs no-mask (NYT)",
        "7-day moving average of new deaths, % relative to July 12",
        &nytmasks,
        &nytnomasks,
        first_date,
        last_date,
    );
    charts::write(
        "images/deaths-jhu.html",
        &mut bightml,
        "COVID-19 deaths: Mask vs no-mask (JHU)",
        "7-day moving average of new deaths, % relative to July 12",
        &jhumasks,
        &jhunomasks,
        first_date,
        last_date
    );
    charts::write(
        "images/deaths-updated-nyt.html",
        &mut bightml,
        "COVID-19 deaths: Mask vs no-mask (Updated NYT)",
        "7-day moving average of new deaths, % relative to July 12",
        &nytmasks,
        &nytnomasks,
        first_date,
        data_last_date,
    );
    charts::write(
        "images/deaths-updated-jhu.html",
        &mut bightml,
        "COVID-19 deaths: Mask vs no-mask (Updated JHU)",
        "7-day moving average of new deaths, % relative to July 12",
        &jhumasks,
        &jhunomasks,
        first_date,
        data_last_date,
    );

    /////////////////// Counties

    let mut nytbycounty = db::getcountymaskdata(&pool, "nytimes/us-counties", "delta_confirmed", data_first_date, data_last_date).await;
    for item in nytbycounty.values_mut() {
        analysis::calcsimplema(item, 7);
    }
    let mut jhubycounty = db::getcountymaskdata(&pool, "nytimes/us-counties", "delta_confirmed", data_first_date, data_last_date).await;
    for item in jhubycounty.values_mut() {
        analysis::calcsimplema(item, 7);
    }
    charts::writecounties(
        "images/counties-nyt.html",
        &mut bightml,
        "COVID-19 cases in Selected Counties, Kansas (NYT)",
        "7-day moving average of new cases, % relative to July 12",
        &vec!["Marion", "McPherson", "Harvey", "Saline"],
        &nytbycounty,
        first_date,
        last_date,
    );
    charts::writecounties(
        "images/counties-jhu.html",
        &mut bightml,
        "COVID-19 cases in Selected Counties, Kansas (JHU)",
        "7-day moving average of new cases, % relative to July 12",
        &vec!["Marion", "McPherson", "Harvey", "Saline"],
        &jhubycounty,
        first_date,
        last_date,
    );
    charts::writecounties(
        "images/counties-updated-nyt.html",
        &mut bightml,
        "COVID-19 cases in Selected Counties, Kansas (Updated NYT)",
        "7-day moving average of new cases, % relative to July 12",
        &vec!["Marion", "McPherson", "Harvey", "Saline"],
        &nytbycounty,
        first_date,
        data_last_date,
    );
    charts::writecounties(
        "images/counties-updated-jhu.html",
        &mut bightml,
        "COVID-19 cases in Selected Counties, Kansas (Updated JHU)",
        "7-day moving average of new cases, % relative to July 12",
        &vec!["Marion", "McPherson", "Harvey", "Saline"],
        &jhubycounty,
        first_date,
        data_last_date,
    );
}
