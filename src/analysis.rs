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

/// Populate the simple moving average in the second element of the list, modifying it in-place.
pub fn calcsimplema(hm: &mut HashMap<i32, f64>, window: usize) {
    let mut history: Vec<f64> = Vec::new();
    let mut keys: Vec<i32> = hm.keys().map(|x| x.clone()).collect();
    keys.sort();
    for key in keys.into_iter() {
        let entry = hm.get_mut(&key).unwrap();
        history.push(*entry);
        if history.len() > window {
            history.remove(0);
        }
        *entry = history.iter().sum::<f64>() / (window as f64);
    }
}

/*
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
            updatehm.entry(*date).and_modify(|rec| rec.combine(countyrec));
        }
    }

    setnew(&mut maskshm, datelist);
    setnewavg(&mut maskshm, datelist, window);

    setnew(&mut nomaskshm, datelist);
    setnewavg(&mut nomaskshm, datelist, window);

    (maskshm, nomaskshm)
}
*/
