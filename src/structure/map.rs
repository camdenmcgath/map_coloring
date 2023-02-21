use std::cmp::Reverse;
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
        self.regions.iter_mut().find(|reg| reg.id == id)
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
            if region.val.is_none() {
                return false;
            }
        }
        true
    }
    pub fn is_consistent(&self, val: usize, id: usize) -> bool {
        for adj in self.get(id).unwrap().adjacent {
            if self.get(adj).unwrap().val == Some(val) {
                return false;
            }
        }
        true
    }
    pub fn select_unassigned_id(&self) -> usize {
        self.regions
            .iter()
            .filter(|reg| reg.val.is_none())
            .min_by_key(|reg| {
                (
                    //MRV
                    reg.domain.len(),
                )
            })
            .unwrap()
            .id
    }
    pub fn assign(&mut self, id: usize, val: usize) {
        if let Some(reg) = self.get_mut(id) {
            reg.val = Some(val);
        }
        //prune ajacent domains
        if let Some(reg) = self.get(id) {
            for adj in &reg.adjacent {
                if let Some(adj_reg) = self.get_mut(*adj) {
                    adj_reg.domain.retain(|x| *x != val)
                }
            }
        }
    }

    pub fn order_domain(&self, id: usize) -> Vec<usize> {
        //closure for counting unassigned adjacent territories
        let choice_effect = |state: usize, reg: &Region| {
            if reg.val.is_none() {
                reg.adjacent
                    .iter()
                    .filter(|&adj| self.get(*adj).unwrap().val.is_none() && *adj != state)
                    .count()
            } else {
                0
            }
        };
        let current_reg = self.get(id).unwrap();
        let mut ordered_domain = current_reg.domain.clone();
        //sorts most adjaent unassigned regions to least
        ordered_domain.sort_by_key(|_| {
            Reverse(
                current_reg
                    .adjacent
                    .iter()
                    .map(|&state| choice_effect(state, &self.get(state).unwrap()))
                    .sum::<usize>(),
            )
        });
        ordered_domain
    }
}
