use std::result;

use crate::structure::map::{self, Map, Region};

//TODO: look at moving some functions out of map struct
pub fn backtracking_search(map: &mut Map) -> Option<Map> {
    let mut vals = 3;
    loop {
        if let Some(result) = backtrack(map, vals) {
            return Some(result);
        } else {
            vals += 1;
        }
    }
}

fn backtrack(map: &mut Map, vals: usize) -> Option<Map> {
    map.init_possible_vals(vals);
    let mut stack = Vec::new();
    stack.push(map.clone());
    while let Some(state) = stack.pop() {
        if state.is_complete() {
            return Some(state);
        }
        let id = state.select_unassigned_id();
        for value in state.order_domain(id) {
            if state.is_consistent(value, id) {
                let mut new_state = state.clone();
                new_state.assign(id, value);
                stack.push(new_state);
            }
        }
    }
    None
}
