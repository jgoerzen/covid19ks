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

#[derive(PartialEq, Debug)]
pub struct Counties<'a> {
    pub clist: Vec<&'a str>,
}

impl<'a> Counties<'a> {
    pub fn new(list: Vec<&'a str>) -> Counties<'a> {
        Counties { clist: list }
    }

    /// Return a SQL "where" clause for this list of counties.
    pub fn sqlclause(&self) -> String {
        format!(
            "({})",
            self.clist
                .iter()
                .map(|x| format!("'{}'", x))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
