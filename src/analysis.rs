/* Analysis

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

/// untested
#[allow(dead_code)]
pub fn calcweightedma(hm: &mut HashMap<i32, f64>, window: usize) {
    let mut history: Vec<f64> = Vec::new();
    let mut keys: Vec<i32> = hm.keys().map(|x| x.clone()).collect();
    keys.sort();
    for key in keys.into_iter() {
        let entry = hm.get_mut(&key).unwrap();
        history.push(*entry);
        if history.len() > window {
            history.remove(0);
        }
        let mut sum = 0.0;
        for (item, index) in history.iter().zip(1..) {
            sum += item * (index as f64);
        }
        *entry = sum / ((history.len() * (history.len() + 1)) as f64 / 2.0)
    }
}

pub fn pctofday0(hm: &HashMap<i32, f64>, firstdate: i32) -> HashMap<i32, f64> {
    let day0 = hm.get(&firstdate).expect("Can't find first value");
    let mut hm = hm.clone();
    for (_, val) in hm.iter_mut() {
        *val = 100f64 * *val / day0;
    }
    hm
}
