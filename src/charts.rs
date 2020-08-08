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
use std::collections::HashMap;
use chrono::{Date, Utc, NaiveDate, offset::TimeZone};
use plotters::prelude::*;

fn n2d(naive: &NaiveDate) -> Date<Utc> {
    Utc.from_utc_date(naive)
}

pub fn write(masks: &HashMap<NaiveDate, ARecord>, nomasks: &HashMap<NaiveDate, ARecord>, datelist: &Vec<NaiveDate>) {
    let root = BitMapBackend::new("main.png", (1024,768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Caption 1", ("sans-serif", 50.0).into_font())
        .build_ranged(n2d(&datelist[0])..n2d(datelist.last().unwrap()), 60f64..130f64).unwrap();

    chart.configure_mesh().line_style_2(&WHITE).draw().expect("draw");


    let masksday0 = masks.get(&datelist[0]).unwrap().newcaseavg;
    let nomasksday0 = nomasks.get(&datelist[0]).unwrap().newcaseavg;

    chart.draw_series(LineSeries::new(
        datelist.iter().map(|d| (n2d(d), 100f64 * masks.get(d).unwrap().newcaseavg / masksday0)),
            &BLUE
    )).unwrap()
        .label("Masks");


    chart.draw_series(LineSeries::new(
        datelist.iter().map(|d| (n2d(d), 100f64 * nomasks.get(d).unwrap().newcaseavg / nomasksday0)),
            &RED
    )).unwrap()
        .label("Masks");


}
