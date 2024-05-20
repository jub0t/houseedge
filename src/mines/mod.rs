use crate::core;

#[derive(Debug, Clone, Copy)]
pub enum MineItem {
    Reward = 0,
    Loss = 1,
    Opened = 2,
}

pub type MineColumn = Vec<MineItem>;

pub struct MineGame {
    pub columns: Vec<MineColumn>,
    pub risk: usize,
}

impl MineGame {
    /// Creates a new MineGame with a given size and risk.
    /// Size defines the length and width of the grid (size x size).
    pub fn new(size: usize, risk: usize) -> Self {
        let flat_size = size * size;
        assert!(
            risk < flat_size,
            "Risk must be less than the total number of cells"
        );

        let mut items: Vec<MineItem> = Vec::with_capacity(flat_size);

        // Fill with rewards and losses
        items.extend((0..(flat_size - risk)).map(|_| MineItem::Reward));
        items.extend((0..risk).map(|_| MineItem::Loss));

        // Shuffle the items
        core::vector::shuffle_vector(&mut items);

        // Create columns from the shuffled items
        let mut columns = Vec::with_capacity(size);
        for _ in 0..size {
            let col: MineColumn = items.drain((items.len() - size)..).collect();
            columns.push(col);
        }

        Self { columns, risk }
    }

    /// Attempts to eliminate an item at the specified row and column.
    /// Returns `Ok(true)` if successful, or an appropriate `EliminateError`.
    pub fn eliminate(&mut self, row: usize, column: usize) -> Result<bool, EliminateError> {
        if row >= self.columns.len() {
            return Err(EliminateError::NullRow);
        }
        if column >= self.columns[row].len() {
            return Err(EliminateError::NullColumn);
        }

        match self.columns[row][column] {
            MineItem::Reward | MineItem::Loss => {
                self.columns[row][column] = MineItem::Opened;
                Ok(true)
            }
            MineItem::Opened => Err(EliminateError::AlreadyOpened),
        }
    }

    /// Returns a string representation of the current game state for display.
    pub fn display(&self) -> String {
        let mut display_string = String::new();
        for row in &self.columns {
            for &item in row {
                let symbol = match item {
                    MineItem::Reward => "R",
                    MineItem::Loss => "L",
                    MineItem::Opened => "O",
                };
                display_string.push_str(symbol);
                display_string.push(' '); // Adding space for better readability
            }
            display_string.push('\n'); // Newline at the end of each row
        }
        display_string
    }
}

#[derive(Debug)]
pub enum EliminateError {
    AlreadyOpened,
    NullColumn,
    NullRow,
}
