// So in Towers, When only two towers are presented, the first reward shall NOT be greater the bet amount.
// If that is ever the case, the casino will end up in negative house edge.

pub enum TowerItem {
    Loss,
    Reward,
    Opened,
}

// Example:
// | --- | --- | x |
// | --- | --- | X |
// | X | --- | --- |
// | --- | X | --- |
// | --- | --- | X |
// | X | --- | --- |
// | --- | --- | X |
// | --- | --- | X |

pub type Tower = [TowerItem; 8];

pub struct Towers {
    pub towers: [Tower; 3],
}

pub enum TowerDifficulty {
    Easy,
    Medium,
    Hard,
}

impl Towers {
    pub fn new(difficulty: TowerDifficulty) -> Self {
        return Self {};
    }
}
