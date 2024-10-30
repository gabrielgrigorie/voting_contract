// src/lib.rs
pub struct VotingContract {
    votes: Vec<i32>,
}

impl VotingContract {
    pub fn new() -> Self {
        VotingContract { votes: Vec::new() }
    }

    pub fn cast_vote(&mut self, vote: i32) {
        self.votes.push(vote);
    }

    pub fn count_votes(&self) -> usize {
        self.votes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cast_vote() {
        let mut contract = VotingContract::new();
        contract.cast_vote(1);
        assert_eq!(contract.count_votes(), 1);
    }
}
