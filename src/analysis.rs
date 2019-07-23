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
use rctree::Node;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum TreeEntry {
    TreeData(parser::Record),
    TreeCalc(String, i64, bool),              // title, calculation, and whether to include in calculations
}

impl TreeEntry {
    pub fn setdeaths(&mut self, newdeaths: i64) {
        match self {
            TreeEntry::TreeData(x) => x.deaths = newdeaths,
            TreeEntry::TreeCalc(t, _, r) => *self = TreeEntry::TreeCalc(t.clone(), newdeaths, *r),
        };
    }

    pub fn getdeaths(&self) -> i64 {
        match self {
            TreeEntry::TreeData(x) => x.deaths,
            TreeEntry::TreeCalc(_, d, _) => *d,
        }
    }

    pub fn gettitle(&self) -> &str {
        match self {
            TreeEntry::TreeData(x) => &x.causeofdeath,
            TreeEntry::TreeCalc(t, _, _) => t,
        }
    }
}

pub fn mktree<'a>(reciter: impl Iterator<Item = parser::Record> + 'a) -> rctree::Node<TreeEntry> {
    let empty = TreeEntry::TreeCalc(String::from("Deaths"), -1, false);

    let mut top = Node::new(empty);
    
    let inserter = | prevcode: Option<(Node<TreeEntry>, Node<TreeEntry>)>, new: parser::Record | {
        let newparent = TreeEntry::TreeCalc(new.chapter.clone(), -1, false);
        let newsub = TreeEntry::TreeCalc(new.subchapter.clone(), -1, false);
        let (parentchap, mut parentsub) = match prevcode {
            Some((x, y)) => {
                let mut newpnode = if *(x.borrow()) == newparent { x } else {
                    let p = Node::new(newparent);
                    top.append(p);
                    top.last_child().unwrap() }; // was just "p" but borrow checker
                let newsubnode = if *(y.borrow()) == newsub { y } else {
                    let p = Node::new(newsub);
                    newpnode.append(p);
                    newpnode.last_child().unwrap() }; // was just "p" but borrow checker
                (newpnode, newsubnode) },
            _ => {
                let mut pnode = Node::new(newparent);
                let psub = Node::new(newsub);
                pnode.append(psub);
                top.append(pnode);
                (top.last_child().unwrap(),
                 top.last_child().unwrap().last_child().unwrap())  // (pnode, psub)
            }
        };
        let node = Node::new(TreeEntry::TreeData(new));
        parentsub.append(node);
        Some((parentchap, parentsub))
    };

    reciter.fold(None, inserter);
    top
}

fn docalc(accum: i64, item: &mut Node<TreeEntry>) -> i64 {
    match item.borrow().clone() {
        /* Exit now so we don't hold the borrow later. */
        TreeEntry::TreeData(x) => return accum + x.deaths,
        /* If we are told to include this TreeCalc in future calculations -- due to
           a coalesce creating it -- include it now.  This also implies no children.
          Normally, skip it, because it's the sum of children. */
        TreeEntry::TreeCalc(_, d, true) => return accum + d,
        _ => ()
    }
    let newdeaths = item.children().fold(0, | a, mut i| docalc(a, &mut i));
    let mut m: std::cell::RefMut<TreeEntry> = item.borrow_mut();
    m.setdeaths(newdeaths);
    accum + newdeaths
}    

/* Update the calculations in the tree. */
pub fn treecalcs(top: &mut Node<TreeEntry>) -> () {
    top.children().fold(0, | a, mut i| docalc(a, &mut i));
    docalc(0, top);
}

/* Coalesce minor items into "other". */
pub fn coalesce(top: &mut Node<TreeEntry>, maxperlevel: usize) -> () {
    let mut items: Vec<Node<TreeEntry>> = top.children().collect();
    if items.len() > maxperlevel {
        items.sort_unstable_by(|a, b| a.borrow().getdeaths().partial_cmp(&b.borrow().getdeaths()).unwrap());
        items.reverse();
        /* We need to pick maxperlevel - 1 items in order to make sure we have
        room to stick in the "other". */
        let extras = &mut items[maxperlevel - 1..];
        let sum = extras.into_iter().fold(0, |a, i| a + i.borrow().getdeaths());
        for item in extras {
            item.detach();
        }
        let newtitle = format!("Other {}", top.borrow().gettitle());
        top.append(Node::new(TreeEntry::TreeCalc(newtitle, sum, true)));
    }
    top.children().for_each(|mut i| coalesce(&mut i, maxperlevel));
}

/* Coalesce minor items into "other" by percent of the top of the tree.
Assumes tree calculation has already been done. 
minfract should be something like 0.05 for 5 percent.  reduction is multiplied by minfract
when recursing to children; set to 1.0 for children to have the same minfract as the parent call. */
pub fn coalescepct(top: &mut Node<TreeEntry>, minfract: f64, reduction: f64) -> () {
    pub fn worker(top: &mut Node<TreeEntry>, totaldeaths: i64, minfract: f64, reduction: f64) -> () {
        let mut items: Vec<Node<TreeEntry>> = top.children().collect();
        let mindeaths = (totaldeaths as f64) * minfract;
        let mut sum = 0;
        for item in &mut items {
            if (item.borrow().getdeaths() as f64) < mindeaths {
                sum += item.borrow().getdeaths();
                item.detach();
            }
        }
        if sum > 0 {
            let newtitle = format!("Other {}", top.borrow().gettitle());
            top.append(Node::new(TreeEntry::TreeCalc(newtitle, sum, true)));
        }
        top.children().for_each(|mut i| worker(&mut i, totaldeaths, minfract * reduction, reduction));
    }
    let totaldeaths = top.borrow().getdeaths();
    worker(top, totaldeaths, minfract, reduction);
}

/* sankeymatic requires unique titles. */
pub fn aretitlesok(top: &Node<TreeEntry>) -> bool {
    let mut seen_titles = HashMap::new();
    for item in top.traverse() {
        match item {
            rctree::NodeEdge::Start(x) => {
                let title = String::from(x.borrow().gettitle());
                if seen_titles.contains_key(&title) {
                    return false;
                }
                seen_titles.insert(title, ());
            }
            _ => { }
        }
    }
    return true;
}

#[cfg(test)]
mod test{
    use super::*;

    fn setup() -> Node<TreeEntry> {
        let mut rdr = parser::parse_init_file(String::from("srcdata.tsv")).unwrap();
        let vr = parser::parse(&mut rdr);
        let mut t = mktree(vr);
        treecalcs(&mut t);
        return t;
    }
    
    #[test]
    fn test_treecalcs() {
        let mut t = setup();
        assert_eq!(t.borrow().getdeaths(), 251308);
        assert_eq!(t.borrow().gettitle(), "Grand Total");
        let mut children = t.children();
        let i = children.next().unwrap();
        assert_eq!(i.borrow().getdeaths(), 8777);
        assert_eq!(i.borrow().gettitle(), "Certain infectious and parasitic diseases");
        let i = i.children().next().unwrap();
        assert_eq!(i.borrow().getdeaths(), 389);
        assert_eq!(i.borrow().gettitle(), "Intestinal infectious diseases");

        // Recalculate and make sure we're still the same
        treecalcs(&mut t);
        assert_eq!(t.borrow().getdeaths(), 251308);        
    }

    #[test]
    fn test_coalesce() {
        let mut t = setup();
        coalesce(&mut t, 3);
        assert_eq!(t.borrow().getdeaths(), 251308);
        assert_eq!(t.borrow().gettitle(), "Grand Total");

        let mut children = t.children();
        let i = children.next().unwrap();
        assert_eq!(i.borrow().gettitle(), "Diseases of the circulatory system");
        assert_eq!(i.borrow().getdeaths(), 56408);

        let i = children.next().unwrap();
        assert_eq!(i.borrow().gettitle(), "External causes of morbidity and mortality");
        assert_eq!(i.borrow().getdeaths(), 68275);

        let i = children.next().unwrap();
        assert_eq!(i.borrow().gettitle(), "Other");
        assert_eq!(i.borrow().getdeaths(), 251308 - 56408 - 68275);
        
        // Recalculate
        treecalcs(&mut t);
        assert_eq!(t.borrow().getdeaths(), 251308);
    }
}
