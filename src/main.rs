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

async fn write_incidence_100k(pool: &SqlitePool, bightml: &mut File, first_date: i32, last_date: i32) {
    let mut nytbycounty100k = db::getcountydata_100k_nytcounties(
        &pool,
        "cases_new",
        first_date,
        last_date,
    )
    .await;

    let mut nytbycounty100k_sum = nytbycounty100k.clone();
    for item in nytbycounty100k_sum.values_mut() {
        *item = analysis::calcsimplesum(item, 14, true);
    }

    // 268 total cases on 2020-08-21; 200 on 2020-08-08, per NYT spreadsheet, so 68 cases
    let rate_20200822 = 100000f64 * 68.0 / 34429.0;
    assert!(rate_20200822 - 0.0000001 < *nytbycounty100k_sum.get("Harvey").unwrap().get(&ymd_to_day(2020, 8, 21)).unwrap());
    assert!(rate_20200822 + 0.0000001 > *nytbycounty100k_sum.get("Harvey").unwrap().get(&ymd_to_day(2020, 8, 21)).unwrap());

    charts::writecounties_100k(
        "counties-100k-sum-nyt",
        bightml,
        "14-day New COVID-19 Cases (NYT)",
        "14-day sum of new cases per 100,000 pop.",
        &vec!["Marion", "Harvey", "Sedgwick", "McPherson"],
        &nytbycounty100k_sum,
        first_date,
        last_date,
    );

    for item in nytbycounty100k.values_mut() {
        *item = analysis::calcsimplema(item, 7);
    }

    // 268 total cases on 2020-08-21; 224 on 2020-08-15 and there were 224 on 2020-08-14 as well.
    // So 44 new cases over that 7-day period.
    let rate_20200822 = 100000f64 * (44.0 / 7.0) / 34429.0;
    assert!(rate_20200822 - 0.0000001 < *nytbycounty100k.get("Harvey").unwrap().get(&ymd_to_day(2020, 8, 21)).unwrap());
    assert!(rate_20200822 + 0.0000001 > *nytbycounty100k.get("Harvey").unwrap().get(&ymd_to_day(2020, 8, 21)).unwrap());


    charts::writecounties_100k(
        "counties-100k-nyt",
        bightml,
        "New COVID-19 cases in Selected Counties, Kansas (NYT)",
        "7-day moving avg of new cases per 100,000 pop.",
        &vec!["Marion", "Harvey", "Sedgwick", "McPherson"],
        &nytbycounty100k,
        first_date,
        last_date,
    );

    let deltconfks = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = 'Kansas' and country_code = 'US' and location_type = 'total-province'",
        first_date,
        last_date,
    )
    .await;
    let rate_20200820 = 100000f64 * (35907.0 - 35419.0) / 2913314.0;
    //assert!(rate_20200820 + 0.0000001 > *deltconfks.get(&ymd_to_day(2020, 8, 20)).unwrap());
    //assert!(rate_20200820 - 0.0000001 < *deltconfks.get(&ymd_to_day(2020, 8, 20)).unwrap());


    let deltconfks = analysis::calcsimplema(&deltconfks, 7);

    // 35907 on 20200820; 32484 on 20200813; that day is included because the delta on 20200814 is nonzero
    let rate_20200820 = 100000f64 * ((35907.0 - 32484.0) / 7.0) / 2913314.0;
    //assert!(rate_20200820 + 0.0000001 > *deltconfks.get(&ymd_to_day(2020, 8, 20)).unwrap());
    //assert!(rate_20200820 - 0.0000001 < *deltconfks.get(&ymd_to_day(2020, 8, 20)).unwrap());

    let deltconfmo = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = 'Missouri' and country_code = 'US' and location_type = 'total-province'",
        first_date,
        last_date,
    )
    .await;
    let deltconfmo = analysis::calcsimplema(&deltconfmo, 7);
    let deltconfne = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = 'Nebraska' and country_code = 'US' and location_type = 'total-province'",
        first_date,
        last_date,
    )
    .await;
    let deltconfne = analysis::calcsimplema(&deltconfne, 7);
    let deltconfco = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = 'Colorado' and country_code = 'US' and location_type = 'total-province'",
        first_date,
        last_date,
    )
    .await;
    let deltconfco = analysis::calcsimplema(&deltconfco, 7);
    let deltconfok = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/daily",
        "delta_confirmed",
        "province = 'Oklahoma' and country_code = 'US' and location_type = 'total-province'",
        first_date,
        last_date,
    )
    .await;
    let deltconfok = analysis::calcsimplema(&deltconfok, 7);

    let deltconfus = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/series",
        "delta_confirmed",
        "province = '' and country_code = 'US' and location_type = 'total-country'",
        first_date,
        last_date,
    )
    .await;

    // 44023 from the graph on their website; on 9-1 it was showing 44036; on 9-2, back to 44023
    let rate_20200820 = 100000f64 * 44023.0 / 332639102.0;
    println!("{}, {}", rate_20200820, *deltconfus.get(&ymd_to_day(2020, 8, 20)).unwrap());
    // assert!(rate_20200820 + 0.0000001 > *deltconfus.get(&ymd_to_day(2020, 8, 20)).unwrap());
    // assert!(rate_20200820 - 0.0000001 < *deltconfus.get(&ymd_to_day(2020, 8, 20)).unwrap());

    let deltconfus = analysis::calcsimplema(&deltconfus, 7);

    let deltconfcan = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/series",
        "delta_confirmed",
        "province = '' and country_code = 'CA' and location_type = 'total-country'",
        first_date,
        last_date,
    )
    .await;
    let deltconfcan = analysis::calcsimplema(&deltconfcan, 7);

    let deltconfgbr = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/series",
        "delta_confirmed",
        "province = '' and country_code = 'GB' and location_type = 'total-country'",
        first_date,
        last_date,
    )
    .await;
    let deltconfgbr = analysis::calcsimplema(&deltconfgbr, 7);

    let deltconffra = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/series",
        "delta_confirmed",
        "province = '' and country_code = 'FR' and location_type = 'total-country'",
        first_date,
        last_date,
    )
    .await;
    let deltconffra = analysis::calcsimplema(&deltconffra, 7);

    let deltconftwn = db::getgeneralmaskdata_100k(
        &pool,
        "jhu/series",
        "delta_confirmed",
        "province = '' and country_code = 'TW' and location_type = 'total-country'",
        first_date,
        last_date,
    )
    .await;
    let deltconftwn = analysis::calcsimplema(&deltconftwn, 7);


    charts::write_generic(
        "centralusa-100k",
        bightml,
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
        first_date,
        last_date,
    );
    charts::write_generic(
        "global-100k",
        bightml,
        "New COVID-19 cases in Selected Regions (JHU + NYT where indicated)",
        "7-day moving avg of new cases per 100,000 pop.",
        vec![
            ("Kansas", &deltconfks),
            ("Sedgwick County (NYT)", nytbycounty100k.get("Sedgwick").unwrap()),
            ("USA", &deltconfus),
            ("Canada", &deltconfcan),
            ("United Kingdom", &deltconfgbr),
            ("France", &deltconffra),
            ("Taiwan", &deltconftwn),
        ],
        first_date,
        last_date,
    );

}

async fn write_testing(pool: &SqlitePool, bightml: &mut File, first_date: i32, last_date: i32) {
    let cttest_ks =
        db::gettestdata(pool, "KS", first_date - 15, last_date).await;
    assert_eq!((723, 5578), *cttest_ks.get(&ymd_to_day(2020, 8, 19)).unwrap());
    let cttest_ks = analysis::calcsimplerate_testdata(&cttest_ks, 14, false);

    let owidtest_usa =
        db::gettestdata_owid(pool, "USA", first_date - 15, last_date).await;
    let owidtest_usa = analysis::calcsimplerate_testdata(&owidtest_usa, 14, false);

    let owidtest_can =
        db::gettestdata_owid(pool, "CAN", first_date - 15, last_date).await;
    let owidtest_can = analysis::calcsimplerate_testdata(&owidtest_can, 14, false);
    let owidtest_gbr =
        db::gettestdata_owid(pool, "GBR", first_date - 15, last_date).await;
    let owidtest_gbr = analysis::calcsimplerate_testdata(&owidtest_gbr, 14, false);
    let owidtest_fra =
        db::gettestdata_owid(pool, "FRA", first_date - 15, last_date).await;
    let owidtest_fra = analysis::calcsimplerate_testdata(&owidtest_fra, 14, false);
    let owidtest_twn =
        db::gettestdata_owid(pool, "TWN", first_date - 15, last_date).await;
    let owidtest_twn = analysis::calcsimplerate_testdata(&owidtest_twn, 14, false);

    let cttest_recommended : HashMap<i32, f64> =
        // recommended rate is 5% per https://coronavirus.jhu.edu/testing/testing-positivity
        (ymd_to_day(2020, 3, 6)..=last_date).map(|x| (x, 5.0)).collect();

    charts::write_generic(
        "test-global",
        bightml,
        "COVID-19 Test Positivity Rate (OWID + Covid Tracking where indicated)",
        "14-day % of test results positive",
        vec![
            ("Kansas (CT)", &cttest_ks),
            ("USA", &owidtest_usa),
            ("Recommended Maximum", &cttest_recommended),
            ("Canada", &owidtest_can),
            ("United Kingdom", &owidtest_gbr),
            ("France", &owidtest_fra),
            ("Taiwan", &owidtest_twn),
        ],
        first_date,
        last_date,
    );
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

    write_incidence_100k(&pool, &mut bightml, data_first_date, data_last_date).await;
    write_testing(&pool, &mut bightml, ymd_to_day(2020, 6, 6), data_last_date).await;
}
