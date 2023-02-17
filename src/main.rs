mod algos;
mod structure;
use algos::backtracking::backtrack_search;
use structure::map::Map;
fn main() {
    let mut map = Map::create(&std::env::args().nth(1).unwrap()).unwrap();
    let mut vals = 0;
    loop {
        if let Some(result) = backtrack_search(&mut map, vals) {
            println!("Solution found using {} colors!", vals);
            print!(
                "{}",
                result
                    .regions
                    .iter()
                    .map(|r| format!("{}: {}", r.name, r.val.unwrap()))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
            return;
        } else {
            vals += 1;
        }
    }
}
