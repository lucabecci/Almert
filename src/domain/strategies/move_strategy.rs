use crate::domain::entity::Entity;
use crate::domain::enums::direction::Direction;
use crate::domain::map::{Map, MapDomain};

impl MoveStrategy {
    fn move_up(&mut self) {
        if self.current_position.0 > 0 {
            self.current_position.0 -= 1;
        }
    }
    fn move_down(&mut self) {
        if self.current_position.0 > 0 {
            self.current_position.0 -= 1;
        }
    }
    pub fn move_direction(&mut self, direction: Direction, map: *MapDomain, entity: * dyn Entity) {
        std::print!("{}", &map);
        match direction {
            Direction::Up => self.move_up(),
            Direction::Down => std::print!("Down"),
            Direction::Left => std::print!("Left"),
            Direction::Right => std::print!("Right"),
        }
    }


}