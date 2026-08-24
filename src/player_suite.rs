use bevy::prelude::Resource;

#[derive(Resource)]
pub struct PlayerSuiteResource {
    pub health: u16,
    pub shield: u16,
    pub points: u16,
    pub money: u16,
}

impl Default for PlayerSuiteResource {
    fn default() -> Self {
        PlayerSuiteResource { health: 100, shield: 0, points: 0, money: 200 }
    }
}

#[derive(PartialEq, Eq)]
pub enum TransactionReturnStatus {
    Success,
    NotEnoughMoney,
}

#[derive(PartialEq, Eq)]
pub enum ReturnStatus {
    Positive,
    NotPositive,
}

impl PlayerSuiteResource {
    pub fn health(&self) -> u16 {
        self.health
    }

    pub fn shield(&self) -> u16 {
        self.shield
    }

    pub fn points(&self) -> u16 {
        self.points
    }

    pub fn money(&self) -> u16 {
        self.money
    }

    pub fn add_health(&mut self, health: u16) {
        self.health += health;
    }

    pub fn subtract_health(&mut self, health: i16) -> ReturnStatus {
        let current_health = self.health as i16;

        if current_health - health <= 0 {
            self.health = 0;
            ReturnStatus::NotPositive
        } else {
            self.health -= health as u16;
            ReturnStatus::Positive
        }
    }

    pub fn add_shield(&mut self, shield: u16) {
        self.shield += shield;
    }

    pub fn subtract_shield(&mut self, shield: i16) -> ReturnStatus {
        let current_shield = self.shield as i16;

        if current_shield - shield <= 0 {
            self.shield = 0;
            ReturnStatus::NotPositive
        } else {
            self.shield -= shield as u16;
            ReturnStatus::Positive
        }
    }

    pub fn add_points(&mut self, points: u16) {
        self.points += points;
    }

    pub fn subtract_points(&mut self, points: i16) -> ReturnStatus {
        let current_points = self.points as i16;

        if current_points - points <= 0 {
            self.points = 0;
            ReturnStatus::NotPositive
        } else {
            self.points -= points as u16;
            ReturnStatus::Positive
        }
    }

    pub fn add_money(&mut self, money: u16) {
        self.money += money;
    }

    pub fn subtract_money(&mut self, money: i16) -> ReturnStatus {
        let current_money = self.money as i16;

        if current_money - money <= 0 {
            self.money = 0;
            ReturnStatus::NotPositive
        } else {
            self.money -= money as u16;
            ReturnStatus::Positive
        }
    }

    pub fn perform_transaction(&mut self, price: u16) -> TransactionReturnStatus {
        if self.money >= price {
            self.money -= price;
            TransactionReturnStatus::Success
        } else {
            TransactionReturnStatus::NotEnoughMoney
        }
    }
}
