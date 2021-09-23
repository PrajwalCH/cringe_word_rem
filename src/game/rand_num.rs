use rand::Rng;

pub fn generate(max_range: usize) -> usize {
    rand::thread_rng().gen_range(0..max_range)
}
