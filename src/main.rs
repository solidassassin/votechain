/* BLOCKCHAIN BASED VOTING MECHANISM
* @TODO
* Implement proper block validation system!
* Create API to interact with the blockchain?
* Create a voting data structure
* Perhaps make it based account ids entirely?
*/


mod blockchain;
use blockchain::{Block, Blockchain, Vote};


fn main() {
    let blockchain = Blockchain::new();
    // blockchain.add_block();
}

