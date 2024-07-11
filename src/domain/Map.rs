pub struct Map {
    space: Vec<Vec<Option<String>>>
}

impl Map {
    pub fn new(rows: usize, cols: usize) -> Map {
        let space = vec![vec![None; cols]; rows];

        Map { space }
    }

    pub fn print(&self) -> None {
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

    pub fn set_value(&mut self, row: usize, col: usize, value: String) -> None {
        if row < self.space.len() && col < self.space[row].len() {
            self.space[row][col] = Some(value);
        }
    }
}