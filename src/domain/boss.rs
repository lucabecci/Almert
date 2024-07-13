use crate::domain::position::Position;
use crate::domain::entity::Entity;

pub struct Boss {
    name: String,
    position: Position,
    experience: u32,
    health: u32,
    attack_power: u32,
}


impl Boss {
    pub fn new(name: String, x: i32, y: i32, health: u32, attack_power: u32) -> Boss {
        Boss {
            name,
            position: Position::new(x, y),
            experience: 0,
            health,
            attack_power,
        }
    }
}

impl Entity for Boss {
    fn set_position(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.position.y += 1,
            Direction::Down => self.position.y -= 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
    }

    fn get_position(&self) -> Position {
        return *self.position
    }

    fn accredit_experience(&mut self, amount: u32) {
        self.experience += amount;
    }

    fn increment_health(&mut self, amount: u32) {
        self.health += amount;
    }

    fn discount_health(&mut self, amount: u32) {
        if self.health > amount {
            self.health -= amount;
        } else {
            self.health = 0;
        }
    }

    fn attack(&self) -> u32 {
        self.attack_power
    }
}