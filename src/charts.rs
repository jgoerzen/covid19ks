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

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use crate::analysis;
use covid19db::dateutil::*;

// use itertools_num::linspace;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::plot::ImageFormat;
use plotly::{Bar, NamedColor, Plot, Rgb, Rgba, Scatter};
// use rand_distr::{Distribution, Normal, Uniform};

pub fn hmtoseries<U: Clone>(hm: &HashMap<i32, U>, keylist: impl Iterator<Item = i32>) -> (Vec<i32>, Vec<U>) {
    let mut retkeys = Vec::new();
    let mut retvals = Vec::new();
    for key in keylist {
        if let Some(val) = hm.get(&key) {
            retkeys.push(key);
            retvals.push(val.clone());
        }
    }
    (retkeys, retvals)
}


pub fn write_generic(
    filename: &'static str, // grumble due to plotly library
    bightml: &mut File,
    title: &str,
    yaxis: &str,
    series: Vec<(&str, &HashMap<i32, f64>)>,
    firstdate: i32,
    lastdate: i32,
) {
    let mut plot = Plot::new();

    // See https://plotly.com/python/reference/scatter/#scatter-line-smoothing for smoothing
    // this line is unnecessary when not using it.
    let line = Line::new().shape(LineShape::Spline).smoothing(1.0);
    for (label, data) in series {
        let (keys, vals) = hmtoseries(data, firstdate..=lastdate);
        let trace = Scatter::new(
            keys.into_iter().map(day_to_nd),
            vals
        )
        .mode(Mode::Lines)
        .line(line.clone()) // only needed for smoothing
        .name(label);
        plot.add_trace(trace);
    }

    let layout = Layout::new()
        .title(Title::new(title))
        .y_axis(Axis::new().title(Title::new(yaxis)));
    plot.set_layout(layout);
    println!("Writing to {}", filename);
    // plot.show();
    // plot.save(filename, ImageFormat::SVG, 1024, 768, 1.0);
    // plot.show_png(1024, 768);
    plot.to_html(format!("html-entire/{}.html", filename));
    let inlinestr = plot.to_inline_html(Some(filename));
    File::create(format!("html-fragments/{}.html", filename))
        .unwrap()
        .write_all(inlinestr.as_ref())
        .unwrap();
    bightml.write_all(inlinestr.as_ref()).unwrap();
    bightml.write_all(b"<br/>\n").unwrap();
}

pub fn write(
    filename: &'static str, // grumble due to plotly library
    bightml: &mut File,
    title: &str,
    yaxis: &str,
    masks: &HashMap<i32, f64>,
    nomasks: &HashMap<i32, f64>,
    firstdate: i32,
    lastdate: i32,
) -> () {
    let maskshm = analysis::pctofday0(masks, firstdate);
    let nomaskshm = analysis::pctofday0(nomasks, firstdate);
    let series = vec![("Masks", &maskshm), ("No masks", &nomaskshm)];
    write_generic(filename, bightml, title, yaxis, series, firstdate, lastdate)
}

pub fn writecounties(
    filename: &'static str, // static grumble due to plotly
    bightml: &mut File,
    title: &str,
    yaxis: &str,
    counties: &Vec<&str>,
    bycounty: &HashMap<String, HashMap<i32, f64>>,
    firstdate: i32,
    lastdate: i32,
) {
    let countypcts: Vec<HashMap<i32, f64>> = counties
        .iter()
        .map(|county| {
            analysis::pctofday0(
                bycounty
                    .get(&String::from(*county))
                    .expect("Can't find county"),
                firstdate,
            )
        })
        .collect();
    let series: Vec<(&str, &HashMap<i32, f64>)> = counties
        .iter()
        .zip(countypcts.iter())
        .map(|(county, pct)| (*county, pct))
        .collect();

    write_generic(filename, bightml, title, yaxis, series, firstdate, lastdate)
}

pub fn writecounties_100k(
    filename: &'static str, // grumble due to plotly
    bightml: &mut File,
    title: &str,
    yaxis: &str,
    counties: &Vec<&str>,
    bycounty: &HashMap<String, HashMap<i32, f64>>,
    firstdate: i32,
    lastdate: i32,
) {
    let series: Vec<(&str, &HashMap<i32, f64>)> = counties
        .iter()
        .map(|county| (*county, bycounty.get(&String::from(*county)).unwrap()))
        .collect();

    write_generic(filename, bightml, title, yaxis, series, firstdate, lastdate)
}
