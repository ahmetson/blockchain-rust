use blockchainlib::*;

fn main () {
    let difficulty = 0x000fffffffffffffffffffffffffffff;    
    
    let mut block = Block::new(0, 0, vec![0; 32], 0, "Genesis block".to_owned(), difficulty);

    println!("{:?}", &block);

    let h = block.hash();

    println!("{:?}", &h);

    block.mine();
    
    println!("{:?}", &block);    
}
