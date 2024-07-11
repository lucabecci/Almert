mod domain;

use crate::domain::map::{Map, MapDomain};

fn main() {
    let mut map = MapDomain::new(3, 3);
    map.print();
}
