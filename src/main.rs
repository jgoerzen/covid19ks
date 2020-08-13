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

use chrono::Local;
use covid19db::dateutil::*;
use sqlx::sqlite::SqlitePool;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

mod analysis;
mod charts;
mod counties;
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
    let first_date = ymd_to_day(2020, 7, 12); // Dr. Norman's original chart used 2020-07-12
    let last_date = ymd_to_day(2020, 8, 3);

    let data_first_date = ymd_to_day(2020, 5, 29);
    let data_last_date = dateutc_to_day(&datelocal_to_dateutc(&Local::today())) - 1;

    let _daterange_output = first_date..=last_date;
    let _daterange_full = data_first_date..=data_last_date;
    let _daterange_updated = first_date..data_last_date;

    // Source: https://www.kansas.com/news/politics-government/article244091222
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

    let mut bightml = File::create("html-fragments/all.html").unwrap();

    let filename = match get_nth_arg(1) {
        Ok(x) => String::from(x.to_str().unwrap()),
        Err(_) => {
            println!("Database file not specified; trying covid19.db in current directory");
            String::from("covid19.db")
        }
    };

    if !Path::new(filename.as_str()).exists() {
        panic!(
            "{} does not exist; download or specify alternative path on command line",
            filename
        )
    }
    let pool = SqlitePool::builder()
        .max_size(5)
        .build(format!("sqlite::{}", filename).as_ref())
        .await
        .expect("Error building");

    let mut nytmasks = db::getmask100kdata(
        &pool,
        "nytimes/us-counties",
        "delta_confirmed",
        true,
        &maskcounties,
        data_first_date,
        data_last_date,
    )
    .await;
    let mut nytnomasks = db::getmask100kdata(
        &pool,
        "nytimes/us-counties",
        "delta_confirmed",
        false,
        &maskcounties,
        data_first_date,
        data_last_date,
    )
    .await;

    nytmasks = analysis::calcsimplema(&nytmasks, 7);
    nytnomasks = analysis::calcsimplema(&nytnomasks, 7);

    charts::write_generic(
        "main-pop100k-nyt",
        &mut bightml,
        "COVID-19: Masks vs no-mask counties, KS (NYT)",
        "7-day moving average of new cases per 100,000 population",
        vec![("Masks", &nytmasks), ("No masks", &nytnomasks)],
        first_date,
        data_last_date,
    );

    let mut nytbycounty100k = db::getcountymaskdata_100k(
        &pool,
        "nytimes/us-counties",
        "delta_confirmed",
        data_first_date,
        data_last_date,
    )
    .await;
    for item in nytbycounty100k.values_mut() {
        *item = analysis::calcsimplema(item, 7);
    }

    /*
    charts::writecounties_100k(
        "counties-100k-nyt",
        &mut bightml,
        "New COVID-19 cases in Selected Counties, Kansas (NYT)",
        "7-day moving average of new cases per 100,000 population",
        &vec!["Marion", "McPherson", "Harvey", "Saline", "Sedgwick"],
        &nytbycounty100k,
        first_date,
        data_last_date,
    );
    */

    charts::writecounties_100k(
        "counties-100k-nyt",
        &mut bightml,
        "New COVID-19 cases in Selected Counties, Kansas (NYT)",
        "7-day moving average of new cases per 100,000 population",
        &vec!["Marion", "Harvey", "Sedgwick"],
        &nytbycounty100k,
        data_first_date,
        data_last_date,
    );

    ////////////////////// TEST DATA

    let mut cttest_ks = db::gettestdata(&pool, Some("KS"), ymd_to_day(2020, 3, 6), data_last_date).await;
    let mut cttest_us = db::gettestdata(&pool, None, ymd_to_day(2020, 3, 6), data_last_date).await;
    let cttest_recommended : HashMap<i32, f64> =
        // recommended rate is 5% per https://coronavirus.jhu.edu/testing/testing-positivity
        (ymd_to_day(2020, 3, 6)..=data_last_date).map(|x| (x, 5.0)).collect();
    charts::write_generic(
        "test-ctp",
        &mut bightml,
        "COVID-19 Test Positivity Rate in Kansas (Covid Tracking Project)",
        "Positivity rate (% of tests results positive)",
        vec![("Kansas", &cttest_ks), ("Overall USA", &cttest_us),
             ("Recommended Maximum", &cttest_recommended)],
        ymd_to_day(2020, 6, 6),
        data_last_date,
    );


    ////////////////////// DEATHS

    /*
    let mut nytbycounty100k = db::getcountymaskdata_100k(
        &pool,
        "nytimes/us-counties",
        "delta_deaths",
        data_first_date,
        data_last_date,
    )
    .await;
    for item in nytbycounty100k.values_mut() {
        *item = analysis::calcsimplema(item, 7);
    }
    charts::writecounties_100k(
        "counties-100k-deaths-nyt",
        &mut bightml,
        "COVID-19 deaths in Selected Counties, Kansas (NYT)",
        "7-day moving average of new cases per 100,000 population",
        &vec!["Marion", "Harvey", "Sedgwick"],
        &nytbycounty100k,
        data_first_date,
        data_last_date,
    );
    */


    //////////////////  Percentage
    /*
    let mut nytmasks = db::getmaskdata(
        &pool,
        "nytimes/us-counties",
        "delta_confirmed",
        true,
        &maskcounties,
        data_first_date,
        data_last_date,
    )
    .await;
    let mut nytnomasks = db::getmaskdata(
        &pool,
        "nytimes/us-counties",
        "delta_confirmed",
        false,
        &maskcounties,
        data_first_date,
        data_last_date,
    )
    .await;
    let mut jhumasks = db::getmaskdata(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        true,
        &maskcounties,
        data_first_date,
        data_last_date,
    )
    .await;
    let mut jhunomasks = db::getmaskdata(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        false,
        &maskcounties,
        data_first_date,
        data_last_date,
    )
    .await;
    jhumasks = analysis::calcsimplema(&jhumasks, 7);
    nytnomasks = analysis::calcsimplema(&nytnomasks, 7);
    nytmasks = analysis::calcsimplema(&nytmasks, 7);
    jhunomasks = analysis::calcsimplema(&jhunomasks, 7);

    */
    /*
    charts::write(
        "main-nyt",
        &mut bightml,
        "COVID-19: Masks vs no-mask counties, KS (NYT)",
        "7-day moving average of new cases, % relative to July 12",
        &nytmasks,
        &nytnomasks,
        first_date,
        last_date
    );

    charts::write(
        "main-jhu",
        &mut bightml,
        "COVID-19: Masks vs no-mask counties, KS (JHU)",
        "7-day moving average of new cases, % relative to July 12",
        &jhumasks,
        &jhunomasks,
        first_date,
        last_date,
    );
    */
    /*

    charts::write(
        "main-updated-nyt",
        &mut bightml,
        "COVID-19: Masks vs no-mask counties, KS (Updated NYT)",
        "7-day moving average of new cases, % relative to July 12",
        &nytmasks,
        &nytnomasks,
        first_date,
        data_last_date,
    );

    charts::write(
        "main-updated-jhu",
        &mut bightml,
        "COVID-19: Masks vs no-mask counties, KS (Updated JHU)",
        "7-day moving average of new cases, % relative to July 12",
        &jhumasks,
        &jhunomasks,
        first_date,
        data_last_date,
    );

    ////////////// Replace the in-ram data with the deaths data.

    let mut nytmasks = db::getmaskdata(
        &pool,
        "nytimes/us-counties",
        "delta_deaths",
        true,
        &maskcounties,
        data_first_date,
        data_last_date,
    )
    .await;
    nytmasks = analysis::calcsimplema(&nytmasks, 7);
    let mut nytnomasks = db::getmaskdata(
        &pool,
        "nytimes/us-counties",
        "delta_deaths",
        false,
        &maskcounties,
        data_first_date,
        data_last_date,
    )
    .await;
    nytnomasks = analysis::calcsimplema(&nytnomasks, 7);
    let mut jhumasks = db::getmaskdata(
        &pool,
        "jhu/daily",
        "delta_deaths",
        true,
        &maskcounties,
        data_first_date,
        data_last_date,
    )
    .await;
    jhumasks = analysis::calcsimplema(&jhumasks, 7);
    let mut jhunomasks = db::getmaskdata(
        &pool,
        "jhu/daily",
        "delta_deaths",
        false,
        &maskcounties,
        data_first_date,
        data_last_date,
    )
    .await;
    jhunomasks = analysis::calcsimplema(&jhunomasks, 7);

    /*
    charts::write(
        "deaths-nyt",
        &mut bightml,
        "COVID-19 deaths: Mask vs no-mask (NYT)",
        "7-day moving average of new deaths, % relative to July 12",
        &nytmasks,
        &nytnomasks,
        first_date,
        last_date,
    );
    charts::write(
        "deaths-jhu",
        &mut bightml,
        "COVID-19 deaths: Mask vs no-mask (JHU)",
        "7-day moving average of new deaths, % relative to July 12",
        &jhumasks,
        &jhunomasks,
        first_date,
        last_date
    ); */
    charts::write(
        "deaths-updated-nyt",
        &mut bightml,
        "COVID-19 deaths: Mask vs no-mask (Updated NYT)",
        "7-day moving average of new deaths, % relative to July 12",
        &nytmasks,
        &nytnomasks,
        first_date,
        data_last_date,
    );
    charts::write(
        "deaths-updated-jhu",
        &mut bightml,
        "COVID-19 deaths: Mask vs no-mask (Updated JHU)",
        "7-day moving average of new deaths, % relative to July 12",
        &jhumasks,
        &jhunomasks,
        first_date,
        data_last_date,
    );

    /////////////////// Counties

    let mut nytbycounty = db::getcountymaskdata(
        &pool,
        "nytimes/us-counties",
        "delta_confirmed",
        data_first_date,
        data_last_date,
    )
    .await;
    for item in nytbycounty.values_mut() {
        *item = analysis::calcsimplema(item, 7);
    }
    let mut jhubycounty = db::getcountymaskdata(
        &pool,
        "nytimes/us-counties",
        "delta_confirmed",
        data_first_date,
        data_last_date,
    )
    .await;
    for item in jhubycounty.values_mut() {
        *item = analysis::calcsimplema(item, 7);
    }
    */
    /*
    charts::writecounties(
        "counties-nyt",
        &mut bightml,
        "COVID-19 cases in Selected Counties, Kansas (NYT)",
        "7-day moving average of new cases, % relative to July 12",
        &vec!["Marion", "McPherson", "Harvey", "Saline"],
        &nytbycounty,
        first_date,
        last_date,
    );
    charts::writecounties(
        "counties-jhu",
        &mut bightml,
        "COVID-19 cases in Selected Counties, Kansas (JHU)",
        "7-day moving average of new cases, % relative to July 12",
        &vec!["Marion", "McPherson", "Harvey", "Saline"],
        &jhubycounty,
        first_date,
        last_date,
    );
    */
    /*
    charts::writecounties(
        "counties-updated-nyt",
        &mut bightml,
        "COVID-19 cases in Selected Counties, Kansas (Updated NYT)",
        "7-day moving average of new cases, % relative to July 12",
        &vec!["Marion", "McPherson", "Harvey", "Saline"],
        &nytbycounty,
        first_date,
        data_last_date,
    );
    charts::writecounties(
        "counties-updated-jhu",
        &mut bightml,
        "COVID-19 cases in Selected Counties, Kansas (Updated JHU)",
        "7-day moving average of new cases, % relative to July 12",
        &vec!["Marion", "McPherson", "Harvey", "Saline"],
        &jhubycounty,
        first_date,
        data_last_date,
    );
    */
}
