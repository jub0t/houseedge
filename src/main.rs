use mines::MineGame;

pub mod core;
pub mod mines;
pub mod roulette;

fn main() {
    let size = 6;
    let risk = 3;

    let mines = MineGame::new(size, risk);
    println!("{:#?}", mines.columns);
}

#[cfg(test)]
mod tests {
    use crate::mines::{MineGame, MineItem};

    #[test]
    pub fn test_mines() {
        let size = 6;
        let risk = 3;

        let mines = MineGame::new(size, risk);

        // Ensure the number of columns matches the size
        assert_eq!(mines.columns.len(), size as usize);

        // Ensure each column has the correct number of items
        for column in mines.columns.iter() {
            assert_eq!(column.len(), size as usize);
        }

        // Count the number of Reward and Loss items
        let mut rewards = 0;
        let mut losses = 0;
        for column in mines.columns.iter() {
            for item in column.iter() {
                match item {
                    MineItem::Reward => rewards += 1,
                    MineItem::Loss => losses += 1,
                }
            }
        }

        // Ensure the total number of items matches the flat size minus risk
        assert_eq!(rewards + losses, size * size);
    }
}
