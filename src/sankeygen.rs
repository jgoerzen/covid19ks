/* Sankey Generator

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

use crate::analysis::*;
use rctree::Node;

fn traverse_start_only<A>(x: rctree::NodeEdge<A>) -> Option<rctree::Node<A>> {
    match x {
        rctree::NodeEdge::Start(y) => Some(y),
        _ => None
    }
}

pub fn sankeymatic(top: &Node<TreeEntry>) -> Vec<String> {
    let mut retval = Vec::new();
    
    fn docalc(x: &Node<TreeEntry>) -> String {
        match x.parent() {
            None => format!("' All Deaths [{}] {}", x.borrow().getdeaths(), x.borrow().gettitle()),
            Some(parent) => format!("{} [{}] {}", parent.borrow().gettitle(), x.borrow().getdeaths(), x.borrow().gettitle())
        }
    }

    fn procitem(accum: &mut Vec<String>, item: &Node<TreeEntry>) -> () {
        accum.push(docalc(item));
        let mut children: Vec<Node<TreeEntry>> = item.children().collect();
        children.sort_unstable_by(|a, b| a.borrow().getdeaths().partial_cmp(&b.borrow().getdeaths()).unwrap());
        children.reverse();
        for child in children {
            procitem(accum, &child);
        }
    }

    procitem(&mut retval, top);
    retval
}
