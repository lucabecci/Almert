pub struct MapDomain {
    space: Vec<Vec<Option<String>>>
}

pub trait Map {
    fn new(rows: usize, cols: usize) -> Self;
    fn print(&self);
    fn set_value(&mut self, row: usize, col: usize, value: String);
}

impl Map for MapDomain {
    fn new(rows: usize, cols: usize) -> Self {
        let space = vec![vec![None; cols]; rows];
        MapDomain { space }
    }

    fn print(&self) {
        for row in &self.space {
            for cell in row {
                match cell {
                    Some(value) => print!("{} ", value),
                    None => print!("None "),
                }
            }
            println!();
        }
    }

    fn set_value(&mut self, row: usize, col: usize, value: String) {
        if row < self.space.len() && col < self.space[row].len() {
            self.space[row][col] = Some(value);
        }
    }
}