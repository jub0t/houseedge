#[derive(Clone, Copy)]
pub enum TowerItem {
    Loss,
    Reward(f32), // Reward now holds the amount
    Opened,
}

pub type Tower = [TowerItem; 8];

#[derive(Clone, Copy)]
pub struct Towers {
    pub towers: [Tower; 3],
}

impl Default for Towers {
    fn default() -> Self {
        Self {
            towers: [
                [TowerItem::Reward(0.0); 8], // Default rewards initialized to 0.0
                [TowerItem::Reward(0.0); 8],
                [TowerItem::Reward(0.0); 8],
            ],
        }
    }
}

pub enum TowerDifficulty {
    Easy,
    Medium,
    Hard,
}

impl Towers {
    pub fn new(difficulty: TowerDifficulty, bet_amount: f32) -> Self {
        let mut instance = Towers::default();
        instance.initialize_towers(difficulty, bet_amount);
        instance
    }

    fn initialize_towers(&mut self, difficulty: TowerDifficulty, bet_amount: f32) {
        match difficulty {
            TowerDifficulty::Easy => self.set_easy_rewards(bet_amount),
            TowerDifficulty::Medium => self.set_medium_rewards(bet_amount),
            TowerDifficulty::Hard => self.set_hard_rewards(bet_amount),
        }
    }

    fn set_easy_rewards(&mut self, bet_amount: f32) {
        // Initialize the towers for easy difficulty
        // For simplicity, let's make sure the first reward is not greater than bet_amount
        for tower in self.towers.iter_mut() {
            tower[0] = TowerItem::Reward(bet_amount); // Ensure the first reward is the bet amount
            for i in 1..tower.len() {
                tower[i] = TowerItem::Reward(bet_amount * (i as f32 + 1.0));
            }
        }
    }

    fn set_medium_rewards(&mut self, bet_amount: f32) {
        // Initialize the towers for medium difficulty
        for tower in self.towers.iter_mut() {
            tower[0] = TowerItem::Reward(bet_amount); // Ensure the first reward is the bet amount
            for i in 1..tower.len() {
                tower[i] = TowerItem::Reward(bet_amount * (i as f32 + 2.0));
            }
        }
    }

    fn set_hard_rewards(&mut self, bet_amount: f32) {
        // Initialize the towers for hard difficulty
        for tower in self.towers.iter_mut() {
            tower[0] = TowerItem::Reward(bet_amount); // Ensure the first reward is the bet amount
            for i in 1..tower.len() {
                tower[i] = TowerItem::Reward(bet_amount * (i as f32 + 3.0));
            }
        }
    }
}
