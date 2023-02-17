use std::time::Instant;

use crate::structure::map::Map;

//TODO: look at moving some functions out of map struct

pub(crate) fn backtrack_search(map: &mut Map, vals: usize) -> Option<Map> {
    let time = Instant::now();
    map.init_possible_vals(vals);
    let mut stack = Vec::new();
    stack.push(map.clone());
    let mut visited = 0;
    while let Some(state) = stack.pop() {
        visited += 1;
        if state.is_complete() {
            println!("Search took {} milliseconds", time.elapsed().as_millis());
            println!("Visited {} states", visited);
            return Some(state);
        }
        let id = state.select_unassigned_id();
        for value in state.order_domain(id) {
            if state.is_consistent(value, id) {
                let mut new_state = state.clone();
                new_state.assign(id, value);
                if new_state.arc_consistency() {
                    stack.push(new_state);
                }
            }
        }
    }
    None
}
