/*

Copyright (c) 2020 John Goerzen

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

use crate::analysis::ARecord;
use chrono::{offset::TimeZone, Date, NaiveDate, Utc};
use plotters::prelude::*;
use std::collections::HashMap;

fn n2d(naive: &NaiveDate) -> Date<Utc> {
    Utc.from_utc_date(naive)
}

pub fn write(
    masks: &HashMap<NaiveDate, ARecord>,
    nomasks: &HashMap<NaiveDate, ARecord>,
    datelist: &Vec<NaiveDate>,
) {
    let root = BitMapBackend::new("main.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .margin(5)
        .caption(
            "COVID-19 cases: Mask vs. no-mask counties",
            ("sans-serif", 50.0).into_font(),
        )
        .build_ranged(
            n2d(&datelist[0])..n2d(datelist.last().unwrap()),
            60f64..130f64,
        )
        .unwrap();

    chart
        .configure_mesh()
        .line_style_2(&WHITE)
        .y_desc("7-day moving average of cases, % change relative to July 12")
        .draw()
        .expect("draw");

    let masksday0 = masks.get(&datelist[0]).unwrap().newcaseavg;
    let nomasksday0 = nomasks.get(&datelist[0]).unwrap().newcaseavg;

    chart
        .draw_series(LineSeries::new(
            datelist.iter().map(|d| {
                (
                    n2d(d),
                    100f64 * masks.get(d).unwrap().newcaseavg / masksday0,
                )
            }),
            &RED,
        ))
        .unwrap()
        .label("Masks")
        .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &RED));

    chart
        .draw_series(LineSeries::new(
            datelist.iter().map(|d| {
                (
                    n2d(d),
                    100f64 * nomasks.get(d).unwrap().newcaseavg / nomasksday0,
                )
            }),
            &BLUE,
        ))
        .unwrap()
        .label("No Masks")
        .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
}

pub fn writecounties(
    bycounty: &HashMap<String, HashMap<NaiveDate, ARecord>>,
    counties: &Vec<&str>,
    datelist: &Vec<NaiveDate>,
) {
    let root = BitMapBackend::new("counties.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .margin(5)
        .caption(
            "COVID-19 cases in Selected Counties, Kansas",
            ("sans-serif", 50.0).into_font(),
        )
        .build_ranged(
            n2d(&datelist[0])..n2d(datelist.last().unwrap()),
            20f64..200f64,
        )
        .unwrap();

    chart
        .configure_mesh()
        .line_style_2(&WHITE)
        .y_desc("7-day moving average of cases, % change relative to July 12")
        .draw()
        .expect("draw");

    let mut idx = 0;

    for county in counties {
        let data = bycounty.get(&String::from(*county)).unwrap();
        let day0 = data.get(&datelist[0]).unwrap().newcaseavg;

        chart
            .draw_series(LineSeries::new(
                datelist
                    .iter()
                    .map(|d| (n2d(d), 100f64 * data.get(d).unwrap().newcaseavg / day0)),
                &Palette99::pick(idx),
            ))
            .unwrap()
            .label(&String::from(*county))
            .legend(move |(x, y)| {
                Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(idx))
            });
        idx += 1;
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
}
