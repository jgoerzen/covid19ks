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

use crate::arecord::ARecord;
use chrono::naive::NaiveDate;
use std::collections::HashMap;

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
        let mut previousrecovered = 0;
        let mut previousactive = 0;
        let mut prevdate = item.pred();
        while prevdate >= datelist[0] {
            if let Some(rec) = hm.get(&prevdate) {
                previouscases = rec.totalcases;
                previousdeaths = rec.totaldeaths;
                previousrecovered = rec.totalrecovered;
                previousactive = rec.totalactive;
                break;
            }
            prevdate = prevdate.pred();
        }

        hm.entry(*item).and_modify(|rec| {
            rec.newcases = rec.totalcases - previouscases;
            rec.newdeaths = rec.totaldeaths - previousdeaths;
            rec.chgrecovered = rec.totalrecovered - previousrecovered;
            rec.chgactive = rec.totalactive - previousactive;
        });
    }
}

/// Populate the moving average in the hashmap.  Must be done after setnew
pub fn setnewavg(hm: &mut HashMap<NaiveDate, ARecord>, datelist: &Vec<NaiveDate>, window: u32) {
    for item in datelist {
        // First, find the previous cases.
        let mut caseaccum = 0;
        let mut deathaccum = 0;
        let mut recoveredaccum = 0;
        let mut activeaccum = 0;
        let mut thisdate = item.clone();
        let mut counter = window;
        while counter > 0 {
            let (newcases, newdeaths, newrecovered, newactive) =
                hm.get(&thisdate).map_or((0, 0, 0, 0), |rec| {
                    (rec.newcases, rec.newdeaths, rec.chgrecovered, rec.chgactive)
                });
            caseaccum += newcases;
            deathaccum += newdeaths;
            recoveredaccum += newrecovered;
            activeaccum += newactive;
            thisdate = thisdate.pred();
            counter -= 1;
        }

        let caseavg = f64::from(caseaccum) / f64::from(window);
        let deathavg = f64::from(deathaccum) / f64::from(window);
        let recoveredavg = f64::from(recoveredaccum) / f64::from(window);
        let activeavg = f64::from(activeaccum) / f64::from(window);
        hm.entry(*item).and_modify(|rec| {
            rec.newcaseavg = caseavg;
            rec.newdeathavg = deathavg;
            rec.chgrecoveredavg = recoveredavg;
            rec.chgactiveavg = activeavg;
        });
    }
}

/// Separate by county
pub fn parser_to_county<I: Iterator<Item = ARecord>>(
    input: I,
    datelist: &Vec<NaiveDate>,
    window: u32,
) -> HashMap<String, HashMap<NaiveDate, ARecord>> {
    let mut hm = HashMap::new();
    let templatehm = newhashmap(datelist);
    for item in input {
        let county = item.county.clone().unwrap();
        hm.entry(county)
            .or_insert(templatehm.clone())
            .entry(item.date.unwrap())
            .and_modify(|rec| rec.combine(&item));
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
