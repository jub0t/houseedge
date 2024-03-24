pub mod vector;

pub fn house_edge(reward: i32, win_probability: i32, loss: i32, lose_probability: i32) -> i32 {
    let ev = ((reward) * (win_probability)) + ((loss) + (lose_probability)); // (Reward)(WinPercent) + (Loss)(LossPercent)
    return -(ev); // Use "-" To Reverse it
}
