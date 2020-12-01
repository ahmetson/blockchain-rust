use blockchainlib::*;

fn main () {
    let difficulty = 0x000fffffffffffffffffffffffffffff;    
    
    let mut block = Block::new(0, now(), vec![0; 32], 0, "Genesis block".to_owned(), difficulty);
    block.mine();

    let mut last_hash = block.hash.clone();
    
    println!("Minted genesis block {:?}", &block);

    let mut blockchain = Blockchain {
	blocks: vec![block],
    };

    println!("Verify: {}", &blockchain.verify());

    for i in 1..=10 {
	let mut block = Block::new(i, now(), last_hash, 0, "Normal block".to_owned(), difficulty);	
	block.mine();	
    
	println!("Minted normal block {:?}", &block);

	last_hash = block.hash.clone();

	blockchain.blocks.push(block);
	println!("Verify: {}", &blockchain.verify());
    }
}
