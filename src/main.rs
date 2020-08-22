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
use std::collections::HashMap;
use std::cmp::max;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::path::Path;

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
    let data_last_date = dateutc_to_day(&datelocal_to_dateutc(&Local::today())) - 2;

    let _daterange_output = first_date..=last_date;
    let _daterange_full = data_first_date..=data_last_date;
    let _daterange_updated = first_date..data_last_date;

    // Original source: https://www.kansas.com/news/politics-government/article244091222
    // Updated 2020-08-13 per https://www.coronavirus.kdheks.gov/DocumentCenter/View/1424/COVID-19-Kansas-Mask-Vs-No-Mask-Counties-Data
    let maskcounties = counties::Counties::new(vec![
        "Allen",
        "Atchison",
        "Bourbon",
        "Crawford",
        "Dickinson",
        "Douglas",
        "Franklin",
        "Grant",
        "Jewell",
        "Johnson",
        "Mitchell",
        "Montgomery",
        "Saline",
        "Shawnee",
        "Wyandotte",
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
        "7-day moving avg of new cases per 100,000 pop.",
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

    let mut jhubycounty100k = db::getcountymaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        data_first_date,
        data_last_date,
    )
    .await;

    let deltconfks = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = 'Kansas' and country_code = 'US'",
        data_first_date,
        data_last_date,
    )
    .await;
    let deltconfks = analysis::calcsimplema(&deltconfks, 7);
    let deltconfmo = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = 'Missouri' and country_code = 'US'",
        data_first_date,
        data_last_date,
    )
    .await;
    let deltconfmo = analysis::calcsimplema(&deltconfmo, 7);
    let deltconfne = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = 'Nebraska' and country_code = 'US'",
        data_first_date,
        data_last_date,
    )
    .await;
    let deltconfne = analysis::calcsimplema(&deltconfne, 7);
    let deltconfco = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = 'Colorado' and country_code = 'US'",
        data_first_date,
        data_last_date,
    )
    .await;
    let deltconfco = analysis::calcsimplema(&deltconfco, 7);
    let deltconfok = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = 'Oklahoma' and country_code = 'US'",
        data_first_date,
        data_last_date,
    )
    .await;
    let deltconfok = analysis::calcsimplema(&deltconfok, 7);

    let deltconfus = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = '' and country_code = 'US'",
        data_first_date,
        data_last_date,
    )
    .await;
    let deltconfus = analysis::calcsimplema(&deltconfus, 7);

    let deltconfcan = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = '' and country_code = 'CA'",
        data_first_date,
        data_last_date,
    )
    .await;
    let deltconfcan = analysis::calcsimplema(&deltconfcan, 7);

    let deltconfdeu = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = '' and country_code = 'DE'",
        data_first_date,
        data_last_date,
    )
    .await;
    let deltconfdeu = analysis::calcsimplema(&deltconfdeu, 7);

    let deltconffra = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = '' and country_code = 'FR'",
        data_first_date,
        data_last_date,
    )
    .await;
    let deltconffra = analysis::calcsimplema(&deltconffra, 7);

    let deltconftwn = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = '' and country_code = 'TW'",
        data_first_date,
        data_last_date,
    )
    .await;
    let deltconftwn = analysis::calcsimplema(&deltconftwn, 7);

    let mut nytbycounty100k_sum = nytbycounty100k.clone();
    for item in nytbycounty100k_sum.values_mut() {
        *item = analysis::calcsimplesum(item, 14);
    }

    charts::writecounties_100k(
        "counties-100k-sum-nyt",
        &mut bightml,
        "14-day New COVID-19 Cases (NYT)",
        "14-day sum of new cases per 100,000 pop.",
        &vec!["Marion", "Harvey", "Sedgwick"],
        &nytbycounty100k_sum,
        data_first_date,
        data_last_date,
    );

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
        "7-day moving avg of new cases per 100,000 pop.",
        &vec!["Marion", "Harvey", "Sedgwick"],
        &nytbycounty100k,
        data_first_date,
        data_last_date,
    );

    charts::write_generic(
        "centralusa-100k",
        &mut bightml,
        "New COVID-19 cases in Central USA (JHU)",
        "7-day moving avg of new cases per 100,000 pop.",
        vec![
            ("Kansas", &deltconfks),
            ("Missouri", &deltconfmo),
            ("Colorado", &deltconfco),
            ("Nebraska", &deltconfne),
            ("Oklahoma", &deltconfok),
            ("USA", &deltconfus),
        ],
        data_first_date,
        data_last_date,
    );
    charts::write_generic(
        "global-100k",
        &mut bightml,
        "New COVID-19 cases in Selected Regions (NYT / JHU)",
        "7-day moving avg of new cases per 100,000 pop.",
        vec![
            ("Kansas", &deltconfks),
            ("Sedgwick County", nytbycounty100k.get("Sedgwick").unwrap()),
            ("USA", &deltconfus),
            ("Canada", &deltconfcan),
            ("Germany", &deltconfdeu),
            ("France", &deltconffra),
            ("Taiwan", &deltconftwn),
        ],
        data_first_date,
        data_last_date,
    );

    ////////////////////// TEST DATA

    let cttest_ks =
        db::gettestdata(&pool, Some("KS"), ymd_to_day(2020, 6, 6), data_last_date).await;
    let cttest_us = db::gettestdata(&pool, None, ymd_to_day(2020, 6, 6), data_last_date).await;
    // let owidtest_us = db::gettestdata_owid(&pool, "USA", ymd_to_day(2020, 6, 6), data_last_date).await;
    let owidtest_can =
        db::gettestdata_owid(&pool, "CAN", ymd_to_day(2020, 6, 6), data_last_date).await;
    let owidtest_deu =
        db::gettestdata_owid(&pool, "DEU", ymd_to_day(2020, 6, 6), data_last_date).await;
    let owidtest_fra =
        db::gettestdata_owid(&pool, "FRA", ymd_to_day(2020, 6, 6), data_last_date).await;
    let owidtest_twn =
        db::gettestdata_owid(&pool, "TWN", ymd_to_day(2020, 6, 6), data_last_date).await;
    let harveyco_kdhe =
        db::gettestdata_harveyco(&pool, "kdhe", ymd_to_day(2020, 6, 6)).await;
    let harveyco_harveyco =
        db::gettestdata_harveyco(&pool, "harveyco", ymd_to_day(2020, 6, 6)).await;
    // (pos, total)
    assert_eq!(
        (3, 44),
        *harveyco_kdhe.get(&ymd_to_day(2020, 8, 16)).unwrap()
    );
    assert_eq!(
        (10, 59),
        *harveyco_harveyco.get(&ymd_to_day(2020, 8, 16)).unwrap()
    );

    assert_eq!(
        (0, 13),
        *harveyco_kdhe.get(&ymd_to_day(2020, 6, 9)).unwrap()
    );
    assert_eq!(
        (0, 0),
        *harveyco_harveyco.get(&ymd_to_day(2020, 6, 9)).unwrap()
    );

    let harveyco_kdhe = analysis::calcsimplerate_testdata(&harveyco_kdhe, 14);
    let harveyco_harveyco = analysis::calcsimplerate_testdata(&harveyco_harveyco, 14);
    let kdheval = *harveyco_kdhe.get(&ymd_to_day(2020, 8, 14)).unwrap();
    let harveycoval = *harveyco_harveyco.get(&ymd_to_day(2020, 8, 14)).unwrap();

    // =100*SUM(C70:C83)/(SUM(C70:C83)+SUM(B70:B83))
    assert!(5.708245242 <= kdheval && 5.708245245 >= kdheval);

    // =100*SUM(E70:E83)/(SUM(D70:D83)+SUM(E70:E83))
    assert!(7.632600258 <= harveycoval && 7.632600261 >= harveycoval);
    let harveyco_enddate = max(analysis::largestkey(&harveyco_kdhe).unwrap(), analysis::largestkey(&harveyco_harveyco).unwrap());

    charts::write_generic(
        "test-harveyco",
        &mut bightml,
        "COVID-19 Test Positivity in Harvey Co, KS",
        "14-day Positive Rate",
        vec![
            ("KDHE data", &harveyco_kdhe),
            ("HV Co Health data", &harveyco_harveyco),
        ],
        ymd_to_day(2020, 6, 6),
        *harveyco_enddate,
    );

    let cttest_recommended : HashMap<i32, f64> =
        // recommended rate is 5% per https://coronavirus.jhu.edu/testing/testing-positivity
        (ymd_to_day(2020, 3, 6)..=data_last_date).map(|x| (x, 5.0)).collect();
    charts::write_generic(
        "test-ctp",
        &mut bightml,
        "COVID-19 Test Positivity Rate (Covid Tracking / OWID)",
        "% of tests results positive",
        vec![
            ("Kansas", &cttest_ks),
            ("Overall USA", &cttest_us),
            ("Recommended Maximum", &cttest_recommended),
            // ("USA (OWID)", &owidtest_us),
            ("Canada", &owidtest_can),
            ("Germany", &owidtest_deu),
            ("France", &owidtest_fra),
            ("Taiwan", &owidtest_twn),
        ],
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
