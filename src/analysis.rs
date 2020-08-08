/* Analysis

Copyright (c) 2019 John Goerzen

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

use crate::parser;
use chrono::naive::NaiveDate;
use rctree::Node;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct ARecord {
    pub totalcases: i32,
    pub newcases: i32,
    pub newcaseavg: f64,
}

impl Default for ARecord {
    fn default() -> ARecord {
        ARecord {
            totalcases: 0,
            newcases: 0,
            newcaseavg: 0.0,
        }
    }
}

/// Returns all dates in the inclusive range.
pub fn alldates(first_date: &NaiveDate, last_date: &NaiveDate) -> Vec<NaiveDate> {
    let mut date = first_date.clone();
    let mut retval = Vec::new();
    while date <= *last_date {
        retval.push(date);
        date = date.succ();
    }
    retval
}

/// Initialize the hashmap
pub fn newhashmap(datelist: &Vec<NaiveDate>) -> HashMap<NaiveDate, ARecord> {
    let mut retval = HashMap::new();
    for item in datelist {
        retval.insert(item.clone(), ARecord::default());
    }
    retval
}

/// Populate the new cases in the hashmap
pub fn setnewcase(hm: &mut HashMap<NaiveDate, ARecord>, datelist: &Vec<NaiveDate>) {
    for item in datelist {
        // Find the previous cases
        let mut previouscases = 0;
        let mut prevdate = item.pred();
        while prevdate >= datelist[0] {
            if let Some(rec) = hm.get(&prevdate) {
                previouscases = rec.totalcases;
                break;
            }
            prevdate = prevdate.pred();
        }

        hm.entry(*item)
            .and_modify(|rec| rec.newcases = rec.totalcases - previouscases);
    }
}

/// Populate the moving average in the hashmap.  Must be done after setnewcase
pub fn setnewcaseavg(hm: &mut HashMap<NaiveDate, ARecord>, datelist: &Vec<NaiveDate>, window: u32) {
    for item in datelist {
        // First, find the previous cases.
        let mut accum = 0;
        let mut thisdate = item.clone();
        let mut counter = window;
        while counter > 0 {
            accum += hm.get(&thisdate).map_or(0, |rec| rec.newcases);
            thisdate = thisdate.pred();
            counter -= 1;
        }

        let avg = f64::from(accum) / f64::from(window);
        hm.entry(*item).and_modify(|rec| rec.newcaseavg = avg);
    }
}

/// Separate by county
pub fn parser_to_county<I: Iterator<Item = parser::Record>>(
    input: I,
    datelist: &Vec<NaiveDate>,
    window: u32,
) -> HashMap<String, HashMap<NaiveDate, ARecord>> {
    let mut hm = HashMap::new();
    let templatehm = newhashmap(datelist);
    for item in input {
        hm.entry(item.county.clone())
            .or_insert(templatehm.clone())
            .entry(item.date)
            .and_modify(|rec| rec.totalcases = item.cases);
    }

    for (_key, val) in &mut hm {
        setnewcase(val, datelist);
        setnewcaseavg(val, datelist, window);
    }

    hm
}

/// Separate by mask
pub fn separate(
    input: HashMap<String, HashMap<NaiveDate, ARecord>>,
    maskcounties: &Vec<&str>,
    datelist: &Vec<NaiveDate>,
    window: u32,
) -> (HashMap<NaiveDate, ARecord>, HashMap<NaiveDate, ARecord>) {
    let mut maskshm = newhashmap(datelist);
    let mut nomaskshm = newhashmap(datelist);

    for (county, countyhm) in input {
        let updatehm = if maskcounties.contains(&county.as_str()) {
            &mut maskshm
        } else {
            &mut nomaskshm
        };
        for (date, countyrec) in countyhm {
            updatehm
                .entry(date)
                .and_modify(|rec| rec.totalcases += countyrec.totalcases);
        }
    }

    setnewcase(&mut maskshm, datelist);
    setnewcaseavg(&mut maskshm, datelist, window);

    setnewcase(&mut nomaskshm, datelist);
    setnewcaseavg(&mut nomaskshm, datelist, window);

    (maskshm, nomaskshm)
}
