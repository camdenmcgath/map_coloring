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
        self.get(id).map_or(false, |reg| {
            reg.adjacent
                .iter()
                .filter_map(|&adj| self.get(adj))
                .all(|adj_reg| adj_reg.val != Some(val) && !adj_reg.domain.is_empty())
        })
    }
    pub fn select_unassigned_id(&self) -> usize {
        let mut unassigned = self
            .regions
            .iter()
            .filter(|&reg| reg.val == None)
            .collect::<Vec<_>>();
        unassigned.sort_by_key(|a| a.domain.len());
        unassigned[0].id
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
        let choice_effect = |reg| {
            let mut sum = 0;
            for state in self.get(reg).unwrap().adjacent {
                if self.get(state).unwrap().val == None {
                    for val in self.get(id).unwrap().domain {
                        if state != val {
                            sum += 1;
                        }
                    }
                }
            }
            sum
        };
        let mut ordered_domain = self.get(id).unwrap().domain.clone();
        ordered_domain.sort_by(|a, b| choice_effect(*b).cmp(&choice_effect(*a)));
        ordered_domain
    }
}
