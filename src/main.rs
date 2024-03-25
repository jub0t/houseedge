use mines::MineGame;

pub mod core;
pub mod mines;
pub mod roulette;
pub mod towers;

fn main() {
    let size = 6;
    let risk = 3;

    let mines = MineGame::new(size, risk);
    println!("{:#?}", mines.columns);
}
