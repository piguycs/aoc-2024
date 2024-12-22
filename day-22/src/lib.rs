#[derive(Debug, Clone, Copy)]
pub struct Secret {
    secret: usize,
}

impl Secret {
    pub fn new(secret: usize) -> Self {
        Self { secret }
    }

    fn mix(&mut self, num: usize) {
        self.secret ^= num;
    }

    fn prune(&mut self) {
        self.secret %= 16777216;
    }

    fn mix_and_prune(&mut self, num: usize) {
        self.mix(num);
        self.prune();
    }

    fn get_next(&mut self) -> usize {
        self.mix_and_prune(self.secret * 64);
        self.mix_and_prune(self.secret / 32);
        self.mix_and_prune(self.secret * 2048);

        self.secret
    }
}

impl Iterator for Secret {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_next())
    }
}

pub fn get_nums(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|e| e.parse::<usize>().expect("a valid number is expected"))
        .collect()
}
