use std::collections::LinkedList;

pub struct Vote {
    // @NOTE might not be needed, since voting will be allowed 1 time only
    uid: u128,
    // Just the candidate number for now
    // @NOTE make a more extensible voting data structure
    vote_data: u16,
    account_id: String,
    signature: Option<String>,
}

pub struct Block {
    votes: Vec<Vote>,
    // Optional because it does not exist starting the blockchain
    prev_hash: Option<String>,
    // Might not be a hash in this case
    hash: String,
}

pub struct Blockchain {
    // The actual blockchain
    blocks: LinkedList<Block>,
    // Will be used for storing unvalidated blocks
    pending_blocks: LinkedList<Block>,
}

impl Block {
    pub fn add_vote(&mut self, vote: Vote) {
        
    }
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: LinkedList::new(),
            pending_blocks: LinkedList::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        
    }
}
