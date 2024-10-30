// Import necessary libraries
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

// Define the structure for our contract
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct VotingContract {
    votes: std::collections::HashMap<String, u32>,
    voters: std::collections::HashSet<String>,
}

#[near_bindgen]
impl VotingContract {
    // Function to register a vote
    pub fn vote(&mut self, candidate: String) {
        let voter_id = env::signer_account_id();

        // Check if the voter has already voted
        if self.voters.contains(&voter_id) {
            env::panic_str("You have already voted");
        }

        // Register the vote
        let count = self.votes.entry(candidate).or_insert(0);
        *count += 1;

        // Mark this voter as having voted
        self.voters.insert(voter_id);
    }

    // Function to get the total votes for a candidate
    pub fn get_votes(&self, candidate: String) -> u32 {
        *self.votes.get(&candidate).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vote() {
        let mut contract = VotingContract::default();
        contract.vote("Alice".to_string());
        assert_eq!(contract.get_votes("Alice".to_string()), 1);
    }
}
