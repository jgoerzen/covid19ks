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

use crate::arecord::ARecord;
use chrono::{offset::TimeZone, Date, NaiveDate, Utc};
use plotters::prelude::*;
use std::collections::HashMap;
use std::ops::Range;

fn n2d(naive: &NaiveDate) -> Date<Utc> {
    Utc.from_utc_date(naive)
}

pub fn write(&ARecord) -> f64>(
    filename: &str,
    title: &str,
    yaxis: &str,
    ymin: f64,
    ymax: f64,
    masks: &Vec<i32, f64>,
    nomasks: &Vec<i32, f64>,
    datelist: &Range
) {
    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .margin(5)
        .caption(title, ("sans-serif", 50.0).into_font())
        .build_ranged(n2d(&datelist[0])..n2d(datelist.last().unwrap()), ymin..ymax)
        .unwrap();

    chart
        .configure_mesh()
        .line_style_2(&WHITE)
        .y_desc(yaxis)
        .draw()
        .expect("draw");

    let masksday0 = func(masks.get(&datelist[0]).unwrap());
    let nomasksday0 = func(nomasks.get(&datelist[0]).unwrap());

    chart
        .draw_series(LineSeries::new(
            datelist
                .iter()
                .map(|d| (n2d(d), 100f64 * func(masks.get(d).unwrap()) / masksday0)),
            &RED,
        ))
        .unwrap()
        .label("Masks")
        .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &RED));

    chart
        .draw_series(LineSeries::new(
            datelist
                .iter()
                .map(|d| (n2d(d), 100f64 * func(nomasks.get(d).unwrap()) / nomasksday0)),
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

pub fn writecounties<F>(
    filename: &str,
    func: F,
    title: &str,
    yaxis: &str,
    ymin: f64,
    ymax: f64,
    counties: &Vec<&str>,
    bycounty: &HashMap<String, HashMap<NaiveDate, ARecord>>,
    datelist: &Vec<NaiveDate>,
) where
    F: Fn(&ARecord) -> f64,
{
    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .margin(5)
        .caption(title, ("sans-serif", 50.0).into_font())
        .build_ranged(n2d(&datelist[0])..n2d(datelist.last().unwrap()), ymin..ymax)
        .unwrap();

    chart
        .configure_mesh()
        .line_style_2(&WHITE)
        .y_desc(yaxis)
        .draw()
        .expect("draw");

    let mut idx = 0;

    for county in counties {
        let data = bycounty.get(&String::from(*county)).unwrap();
        let day0 = func(data.get(&datelist[0]).unwrap());

        chart
            .draw_series(LineSeries::new(
                datelist
                    .iter()
                    .map(|d| (n2d(d), 100f64 * func(data.get(d).unwrap()) / day0)),
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
