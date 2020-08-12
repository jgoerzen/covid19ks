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

use plotters::prelude::*;
use std::collections::HashMap;

use covid19db::dateutil::*;

pub fn write(
    filename: &str,
    title: &str,
    yaxis: &str,
    ymin: f64,
    ymax: f64,
    masks: &HashMap<i32, f64>,
    nomasks: &HashMap<i32, f64>,
    firstdate: i32,
    lastdate: i32,
) {
    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .margin(5)
        .caption(title, ("sans-serif", 50.0).into_font())
        .build_ranged(day_to_dateutc(firstdate)..(day_to_dateutc(lastdate)), ymin..ymax)
        .unwrap();

    chart
        .configure_mesh()
        .line_style_2(&WHITE)
        .y_desc(yaxis)
        .draw()
        .expect("draw");

    let masksday0 = masks.get(&firstdate).expect("Can't find first value");
    let nomasksday0 = nomasks.get(&firstdate).expect("Can't find first value");

    chart
        .draw_series(LineSeries::new(
            (firstdate..=lastdate)
                .into_iter()
                .filter_map(|d| masks.get(&d).map(|x| (d, x)))
                .map(|(x, y)| (day_to_dateutc(x), 100f64 * y / masksday0)),
            &RED
        ))
        .unwrap()
        .label("Masks")
        .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &RED));

    chart
        .draw_series(LineSeries::new(
            (firstdate..=lastdate)
                .into_iter()
                .filter_map(|d| nomasks.get(&d).map(|x| (d, x)))
                .map(|(x, y)| (day_to_dateutc(x), 100f64 * y / nomasksday0)),
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
    filename: &str,
    title: &str,
    yaxis: &str,
    ymin: f64,
    ymax: f64,
    counties: &Vec<&str>,
    bycounty: &HashMap<String, HashMap<i32, f64>>,
    firstdate: i32,
    lastdate: i32
)
{
    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .margin(5)
        .caption(title, ("sans-serif", 50.0).into_font())
        .build_ranged(day_to_dateutc(firstdate)..day_to_dateutc(lastdate), ymin..ymax)
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
        let day0 = data.get(&firstdate).expect("Can't find first value");

        chart
            .draw_series(LineSeries::new(
                (firstdate..=lastdate)
                    .into_iter()
                    .filter_map(|d| data.get(&d).map(|x| (d, x)))
                    .map(|(x, y)| (day_to_dateutc(x), 100f64 * y / day0)),
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
