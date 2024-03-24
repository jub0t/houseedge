use rand::seq::SliceRandom;

pub fn shuffle_vector<T>(items: &mut Vec<T>) -> &Vec<T> {
    let mut rng = rand::thread_rng();
    items.shuffle(&mut rng);
    items
}
