#[derive(Debug)]
pub struct HighScores<'a> {
    data: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { data: scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.data
    }

    pub fn latest(&self) -> Option<u32> {
        self.data.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.data.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = self.data.to_vec();
        v.sort();
        v.into_iter().rev().take(3).collect()
    }
}
