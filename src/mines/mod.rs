use crate::core;

#[derive(Debug)]
pub enum MineItem {
    Reward = 0,
    Loss = 1,
}

pub type Mine = MineItem;

pub type MineColumn = Vec<Mine>;

pub struct MineGame {
    // Vec<MineColumn> = Multiple Columns
    pub columns: Vec<MineColumn>,

    // More Risk = More Reward,
    // Basically a multiplier.
    pub risk: usize,
}

impl MineGame {
    // Size defined length and width, Size(6) = 6x6 game.
    // This is a very poor implementation of the mines game.
    pub fn new(size: usize, risk: usize) -> Self {
        let mut items = Vec::new();
        let flat_size = (size * size) as usize;

        for _ in 0..(flat_size - risk) {
            items.push(MineItem::Reward);
        }

        for _ in 0..risk {
            items.push(MineItem::Loss);
        }

        core::vector::shuffle_vector(&mut items);

        let mut columns = Vec::new();

        // Loop through columns
        for _ in 0..size {
            let mut col = Vec::new();

            // Loop through rows in the column
            for _ in 0..size {
                let item = items.pop().expect("Unexpected end of items");
                col.push(item);
            }

            columns.push(col);
        }

        if items.len() != 0 {
            println!("Too many items, some were not used");
        }

        Self { columns, risk }
    }
}
