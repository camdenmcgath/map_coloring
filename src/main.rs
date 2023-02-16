mod algos;
mod structure;
use algos::backtracking::backtracking_search;
use structure::map::Map;
fn main() {
    let mut map = Map::create(&std::env::args().nth(1).unwrap()).unwrap();
    backtracking_search(&mut map);
}
