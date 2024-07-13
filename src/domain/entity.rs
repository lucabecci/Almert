use crate::domain::enums::direction::Direction;
use crate::domain::position::Position;
pub trait Entity {
    fn set_position(&mut self, direction: Direction);
    fn get_position(&self) -> Position;
    fn accredit_experience(&mut self, amount: u32);
    fn increment_health(&mut self, amount: u32);
    fn discount_health(&mut self, amount: u32);
    fn attack(&self) -> u32;
}