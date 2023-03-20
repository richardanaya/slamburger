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

    pub fn next_max(&mut self, max_to_stay_below: usize) -> usize {
        let max = max_to_stay_below as u64;
        let rand = self.next() % max;
        rand as usize
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
        // choose multiple and don't repeat
        let mut chosen = Vec::with_capacity(n);
        let mut indices = (0..self.len()).collect::<Vec<_>>();
        for _ in 0..n {
            let index = rng.next_max(indices.len());
            let index = indices.remove(index);
            chosen.push(self[index].clone());
        }
        chosen
    }
}
