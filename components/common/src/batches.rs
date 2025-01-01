#[derive(Clone)]
pub struct Batch {
    pub start: i32,
    pub end: i32,
    iteration: i32,
    size: i32,
}

impl Iterator for Batch {
    type Item = Batch;

    fn next(&mut self) -> Option<Self::Item> {
        let total = self.end / self.size + 1;

        if self.iteration > total {
            return None;
        }

        let mut batch = self.clone();

        batch.start = self.size * (self.iteration - 1);
        batch.end = self.size * self.iteration;

        self.iteration += 1;

        Some(batch)
    }
}

pub fn batches(start: i32, end: i32, size: i32) -> Batch {
    Batch {
        start: start,
        end: end,
        iteration: 1,
        size: size,
    }
}
