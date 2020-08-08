/* Analysis record

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

#[derive(Debug, PartialEq, Clone)]
pub struct ARecord {
    pub totalcases: i32,
    pub newcases: i32,
    pub newcaseavg: f64,
    pub totaldeaths: i32,
    pub newdeaths: i32,
    pub newdeathavg: f64,
    pub totalrecovered: i32,
    pub totalactive: i32,
    pub incidence_rate: f64,
    pub case_fatality_ratio: f64,
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
            totalrecovered: 0,
            totalactive: 0,
            incidence_rate: 0.0,
            case_fatality_ratio: 0.0,
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
