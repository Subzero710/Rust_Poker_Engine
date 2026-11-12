use super::{Card, CardBitSet};

/// An iterator over all distinct `k`-card combinations from a `CardBitSet`.
#[derive(Debug, Clone)]
pub struct CardIter {
    pool: Vec<Card>,
    indices: Vec<usize>,
    done: bool,
}

impl CardIter {
    /// Creates a new iterator yielding every `k`-card combination from `cards`.
    #[must_use]
    pub fn new(cards: CardBitSet, k: usize) -> Self {
        let pool: Vec<Card> = cards.into_iter().collect();
        let n = pool.len();

        if k > n {
            Self {
                pool: Vec::new(),
                indices: Vec::new(),
                done: true,
            }
        } else {
            Self {
                pool,
                indices: (0..k).collect(),
                done: false,
            }
        }
    }
}

impl Iterator for CardIter {
    type Item = CardBitSet;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let k = self.indices.len();
        let n = self.pool.len();

        let mut combo = CardBitSet::new();
        for &idx in &self.indices {
            combo.insert(self.pool[idx]);
        }

        let mut i = k;
        while i > 0 {
            i -= 1;
            if self.indices[i] < n - k + i {
                self.indices[i] += 1;
                for j in (i + 1)..k {
                    self.indices[j] = self.indices[j - 1] + 1;
                }
                return Some(combo);
            }
        }

        self.done = true;
        Some(combo)
    }
}
