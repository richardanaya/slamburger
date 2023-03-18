use std::ops::RangeInclusive;

pub struct Rand(u64);

impl Rand {
    pub fn new_with_seed(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(0x6C078965).wrapping_add(1);
        self.0
    }

    pub fn gen_range(&mut self, range: RangeInclusive<f32>) -> f32 {
        let min = *range.start();
        let max = *range.end();
        let range = max - min;
        let rand = self.next() as f32 / u64::MAX as f32;
        min + rand * range
    }
}

// implement choose_multiple for Vec

pub trait ChooseMultiple {
    fn choose_multiple(&self, rng: &mut Rand, n: usize) -> Self
    where
        Self: Sized;
}

impl<T> ChooseMultiple for Vec<T>
where
    T: Clone,
{
    fn choose_multiple(&self, rng: &mut Rand, n: usize) -> Self {
        let mut result = Vec::new();
        for _ in 0..n {
            let index = rng.next() as usize % self.len();
            result.push(self[index].clone());
        }
        result
    }
}
