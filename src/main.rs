mod algos;
mod structure;
use algos::backtracking::backtracking_search;
use structure::map::Map;
fn main() {
    let mut map = Map::create(&std::env::args().nth(1).unwrap()).unwrap();
    if let Some(result) = backtracking_search(&mut map) {
        print!(
            "{}",
            result
                .regions
                .iter()
                .map(|r| format!("{}: {}", r.name, r.val.unwrap()))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}
