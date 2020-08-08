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
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct ARecord {
    pub totalcases: i32,
    pub newcases: i32,
    pub newcaseavg: f64,
    pub totaldeaths: i32,
    pub newdeaths: i32,
    pub newdeathavg: f64,
}

impl Default for ARecord {
    fn default() -> ARecord {
        ARecord {
            totalcases: 0,
            newcases: 0,
            newcaseavg: 0.0,
            totaldeaths: 0,
            newdeaths: 0,
            newdeathavg: 0.0,
        }
    }
}

impl ARecord {
    pub fn getnewcaseavg(&self) -> f64 {
        self.newcaseavg
    }

    pub fn getnewdeathavg(&self) -> f64 {
        self.newdeathavg
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

/// Populate the new cases and deaths in the hashmap
pub fn setnew(hm: &mut HashMap<NaiveDate, ARecord>, datelist: &Vec<NaiveDate>) {
    for item in datelist {
        // Find the previous cases
        let mut previouscases = 0;
        let mut previousdeaths = 0;
        let mut prevdate = item.pred();
        while prevdate >= datelist[0] {
            if let Some(rec) = hm.get(&prevdate) {
                previouscases = rec.totalcases;
                previousdeaths = rec.totaldeaths;
                break;
            }
            prevdate = prevdate.pred();
        }

        hm.entry(*item).and_modify(|rec| {
            rec.newcases = rec.totalcases - previouscases;
            rec.newdeaths = rec.totaldeaths - previousdeaths
        });
    }
}

/// Populate the moving average in the hashmap.  Must be done after setnew
pub fn setnewavg(hm: &mut HashMap<NaiveDate, ARecord>, datelist: &Vec<NaiveDate>, window: u32) {
    for item in datelist {
        // First, find the previous cases.
        let mut caseaccum = 0;
        let mut deathaccum = 0;
        let mut thisdate = item.clone();
        let mut counter = window;
        while counter > 0 {
            let (newcases, newdeaths) = hm
                .get(&thisdate)
                .map_or((0, 0), |rec| (rec.newcases, rec.newdeaths));
            caseaccum += newcases;
            deathaccum += newdeaths;
            thisdate = thisdate.pred();
            counter -= 1;
        }

        let caseavg = f64::from(caseaccum) / f64::from(window);
        let deathavg = f64::from(deathaccum) / f64::from(window);
        hm.entry(*item).and_modify(|rec| {
            rec.newcaseavg = caseavg;
            rec.newdeathavg = deathavg
        });
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
            .and_modify(|rec| {
                rec.totalcases = item.cases;
                rec.totaldeaths = item.deaths
            });
    }

    for (_key, val) in &mut hm {
        setnew(val, datelist);
        setnewavg(val, datelist, window);
    }

    hm
}

/// Separate by mask
pub fn separate(
    input: &HashMap<String, HashMap<NaiveDate, ARecord>>,
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
            updatehm.entry(*date).and_modify(|rec| {
                rec.totalcases += countyrec.totalcases;
                rec.totaldeaths += countyrec.totaldeaths
            });
        }
    }

    setnew(&mut maskshm, datelist);
    setnewavg(&mut maskshm, datelist, window);

    setnew(&mut nomaskshm, datelist);
    setnewavg(&mut nomaskshm, datelist, window);

    (maskshm, nomaskshm)
}
