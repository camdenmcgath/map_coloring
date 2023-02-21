use std::time::Instant;

use crate::structure::map::Map;

//some functions contained in map struct (map.rs)

pub(crate) fn backtrack_search(map: &mut Map, vals: usize) -> Option<Map> {
    let time = Instant::now();
    map.init_possible_vals(vals);
    let mut stack = Vec::new();
    stack.push(map.clone());
    let mut visited = 0;
    while let Some(state) = stack.pop() {
        if state.is_complete() {
            println!("Search took {} microseconds", time.elapsed().as_micros());
            println!("Visited {} states", visited);
            return Some(state);
        }
        let id = state.select_unassigned_id();
        visited += 1;
        for value in state.order_domain(id) {
            if state.is_consistent(value, id) {
                let mut new_state = state.clone();
                new_state.assign(id, value);
                stack.push(new_state);
            }
        }
    }
    println!("Search took {} microseconds", time.elapsed().as_micros());
    println!("Visited {} states", visited);
    None
}
