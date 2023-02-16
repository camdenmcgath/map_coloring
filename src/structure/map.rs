use std::collections::VecDeque;
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

impl Region {
    fn default() -> Self {
        Default::default()
    }
}
#[derive(Clone)]
pub struct Map {
    pub regions: Vec<Region>,
}

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
        /*for adj in self.get(id).unwrap().adjacent {
            if self.get(adj).unwrap().val == Some(val) || self.get(adj).unwrap().domain.is_empty() {
                return false;
            }
        }
        true*/
        self.get(id).map_or(false, |reg| {
            reg.adjacent
                .iter()
                .filter_map(|&adj| self.get(adj))
                .all(|adj_reg| adj_reg.val != Some(val) && !adj_reg.domain.is_empty())
        })
    }
    pub fn select_unassigned_id(&mut self) -> usize {
        self.regions
            .sort_by(|a, b| b.domain.len().cmp(&a.domain.len()));
        //add pruning by degree
        self.regions[0].id
    }
    pub fn assign(&mut self, id: usize, val: usize) {
        self.get_mut(id).val = Some(val);
        for adj in self.get(id).adjacent {
            if let Some(pos) = self.get(adj).domain.iter().position(|v| *v == val) {
                self.get_mut(adj).domain.swap_remove(pos);
            }
        }
    }
    pub fn order_domain(&self, id: usize) -> Vec<usize> {
        let choice_effect = |reg| {
            let mut sum = 0;
            for state in self.get(reg).adjacent {
                if self.get(state).val == None {
                    for val in self.get(id).domain {
                        if state != val {
                            sum += 1;
                        }
                    }
                }
            }
            sum
        };
        let mut region = self.get(id).clone();
        region
            .domain
            .sort_by(|a, b| choice_effect(*b).cmp(&choice_effect(*a)));
        region.domain
    }
    pub fn maintain_arc_consistency(&mut self, id: usize) -> Option<()> {
        let mut queue = VecDeque::new();
        for adj in self.get(id).adjacent {
            if self.get(adj).val.is_none() {
                queue.push_back(self.get(adj).id);
            }
        }
        while let Some(adj) = queue.pop_front() {
            if self.revise(id, adj) {
                if self.get(id).domain.is_empty() {
                    return None;
                }
                for neighbor in self.get(id).adjacent {
                    if neighbor != adj && self.get(neighbor).val.is_none() {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        Some(())
    }

    fn revise(&mut self, origin: usize, neighbor: usize) -> bool {
        let mut revised = false;
        for val in self.get(origin).domain {
            let consistent = self
                .get(neighbor)
                .domain
                .iter()
                .any(|y| self.is_consistent(val, *y));
            if !consistent {
                self.get(origin).domain.remove(
                    self.get(origin)
                        .domain
                        .iter()
                        .position(|x| *x == val)
                        .unwrap(),
                );
                self.get_mut(origin).val = self.get(origin).domain.iter().next().cloned();
                revised = true;
            }
        }
        return revised;
    }
}
