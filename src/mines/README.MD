# Determining the Payout in the Mines Game

## Problem Statement
We need to determine a profit amount for the user in a Mines game such that the user feels like they are making a profit in the short term, but the house retains a statistical edge in the long term.

## Steps to Calculate

### 1. Calculate the Probability of Winning a Single Cell
- The total number of cells in the grid is `size^2`.
- The probability of hitting a reward (instead of a loss) is:
  ```math
    P(\text{Reward}) = \frac{\text{total cells} - \text{risk}}{\text{total cells}} = \frac{size^2 - \text{risk}}{size^2}
  ```
  For a 6x6 grid with 12 risk cells:
  ```math 
    P(\text{Reward}) = \frac{36 - 12}{36} = \frac{24}{36} = \frac{2}{3}
  ```

```rust
// Calculate the probability of hitting a reward
let total_cells = size * size;
let reward_probability = (total_cells - risk) as f32 / total_cells as f32;
```

### 2. Determine the Expected Payout
- Set the reward such that the expected value (EV) is slightly less than the bet to ensure the house edge.
- Suppose the base bet amount is \( B \). The reward payout \( R \) needs to be set such that the expected value favors the house.
- The equation for the expected value (EV) of a single cell is:
  ```math
    EV = P(\text{Reward}) \times R - P(\text{Loss}) \times B
  ```
  The probability of loss \( P(\text{Loss}) \) is:
  ```math
    P(\text{Loss}) = 1 - P(\text{Reward}) = \frac{1}{3}
  ```

```rust
  // Set the base bet amount
let base_bet_amount = 10;

// Set the reward payout to ensure a house edge
let reward_payout = 1.5 * base_bet_amount;
```

### 3. Solving for \( R \)
- Rearrange the inequality to ensure the expected value is less than the bet amount:

```math
 \frac{2}{3}R - \frac{1}{3}B < B 
\frac{2}{3}R < B + \frac{1}{3}B 
 \frac{2}{3}R < \frac{4}{3}B
 R < 2B $$
```

```rust
// Calculate the maximum reward payout to maintain house edge
let max_reward_payout = 2 * base_bet_amount;
```

- This means the reward payout \( R \) should be less than twice the base bet amount \( B \).

### Conclusion
To maintain a house edge, the reward payout \( R \) for a winning cell should be set to a value less than twice the base bet amount \( B \). For example, you can set \( R \) to 1.5 times \( B \).

### Example Implementation
If the base bet amount is \( 10 \), set the reward payout to:

```math
    R = 1.5 \times B = 1.5 \times 10 = 15
```

This ensures the expected value calculation favors the house in the long term while still providing players with a sense of profit in the short term.
