use rand::seq::SliceRandom;

// Pseudo-random Shuffling
pub fn shuffle_vector<T>(items: &mut Vec<T>) -> &Vec<T> {
    let mut rng = rand::thread_rng();
    items.shuffle(&mut rng);
    items
}
