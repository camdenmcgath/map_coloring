use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::fs;

#[derive(Debug, Clone, Default)]
pub struct Region {
    pub name: String,
    pub id: usize,
    pub val: Option<usize>,
    pub domain: Vec<usize>,
    pub adjacent: Vec<usize>,
}
#[derive(Clone)]
pub struct Map {
    pub regions: Vec<Region>,
}

//TODO: optimize
impl Map {
    pub fn create(file_name: &str) -> Result<Map, &'static str> {
        let binding = fs::read_to_string(file_name).expect("unable to convert file to string");
        let input_lines = binding.lines().collect::<Vec<&str>>();
        let mut regions = Vec::new();
        for (i, line) in input_lines.into_iter().skip(1).enumerate() {
            regions.push(Region {
                name: line.split(',').next().unwrap().to_string(),
                id: i,
                val: None,
                domain: Vec::new(),
                adjacent: line
                    .split(',')
                    .into_iter()
                    .enumerate()
                    .skip(1)
                    .filter_map(|(n, x)| {
                        if x.parse::<usize>().unwrap() == 1 {
                            Some(n - 1)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<usize>>(),
            });
        }
        Ok(Map { regions })
    }
    pub fn get(&self, id: usize) -> Option<Region> {
        for reg in &self.regions {
            if reg.id == id {
                return Some(reg.clone());
            }
        }
        None
    }
    pub fn get_mut(&mut self, id: usize) -> Option<&mut Region> {
        for reg in &mut self.regions {
            if reg.id == id {
                return Some(reg);
            }
        }
        None
    }
    pub fn init_possible_vals(&mut self, n: usize) {
        self.regions.iter_mut().for_each(|reg| {
            reg.domain.clear();
            for val in 1..=n {
                reg.domain.push(val)
            }
        });
    }
    pub fn is_complete(&self) -> bool {
        for region in self.regions.iter() {
            if region.val == None {
                return false;
            }
        }
        true
    }
    pub fn is_consistent(&self, val: usize, id: usize) -> bool {
        self.get(id).map_or(false, |reg| {
            reg.adjacent
                .iter()
                .filter_map(|&adj| self.get(adj))
                .all(|adj_reg| adj_reg.val != Some(val) && !adj_reg.domain.is_empty())
        })
    }
    pub fn select_unassigned_id(&self) -> usize {
        self.regions
            .iter()
            .filter(|reg| reg.val == None)
            .min_by_key(|reg| {
                (
                    reg.domain.len(),
                    reg.adjacent
                        .iter()
                        .filter(|adj| self.get(**adj).unwrap().val != None)
                        .count(),
                )
            })
            .unwrap()
            .id
    }
    pub fn assign(&mut self, id: usize, val: usize) {
        if let Some(reg) = self.get_mut(id) {
            reg.val = Some(val);
        }
        if let Some(reg) = self.get(id) {
            for adj in &reg.adjacent {
                self.get_mut(*adj)
                    .and_then(|adj_reg| Some(adj_reg.domain.retain(|x| *x != val)));
            }
        }
    }
    pub fn order_domain(&self, id: usize) -> Vec<usize> {
        let choice_effect = |state: usize, reg: &Region| {
            if reg.val == None {
                reg.adjacent
                    .iter()
                    .filter(|&adj| self.get(*adj).unwrap().val == None && *adj != state)
                    .count()
            } else {
                0
            }
        };
        let current_reg = self.get(id).unwrap();
        let mut ordered_domain = current_reg.domain.clone();
        ordered_domain.sort_by_key(|_| {
            current_reg
                .adjacent
                .iter()
                .map(|&state| choice_effect(state, &self.get(state).unwrap()))
                .sum::<usize>()
        });
        ordered_domain
    }
    pub fn arc_consistency(&mut self) -> bool {
        let mut queue = VecDeque::new();
        for reg in self.regions.iter().filter(|r| r.val.is_some()) {
            for adj in reg.adjacent.iter() {
                queue.push_back((reg.id, *adj));
            }
        }
        while let Some((x1, x2)) = queue.pop_front() {
            if self.revise(x1, x2) {
                if self.get(x1).unwrap().domain.len() == 0 {
                    return false;
                }
                for xk in self.get(x1).unwrap().adjacent.iter() {
                    if *xk != x2 {
                        queue.push_back((*xk, x1));
                    }
                }
            }
        }
        true
    }
    fn revise(&mut self, x1: usize, x2: usize) -> bool {
        let mut revised = false;
        let mut satisfied;
        let mut to_delete = HashSet::new();
        let binding = self.get(x1).unwrap();
        for x in binding.domain.iter() {
            satisfied = false;
            for y in self.get(x2).unwrap().domain.iter() {
                if x != y {
                    satisfied = true;
                }
            }
            if !satisfied {
                to_delete.insert(x);
                revised = true;
            }
        }
        self.get_mut(x1)
            .and_then(|r| Some(r.domain.retain(|v| !to_delete.contains(&v))));
        return revised;
    }
}
