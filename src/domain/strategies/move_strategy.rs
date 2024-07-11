use crate::domain::enums::direction::Direction;
use crate::domain::map::{Map, MapDomain};

impl MoveStrategy {
    fn move_direction(&mut self, direction: Direction, map: *MapDomain) {
        std::print!("{}", &map);
        match direction {
            Direction::Up => std::print!("Up"),
            Direction::Down => std::print!("Down"),
            Direction::Left => std::print!("Left"),
            Direction::Right => std::print!("Right"),
        }
    }
}