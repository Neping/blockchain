use core::blockchain;

fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();
    bc.add_block(String::from("a -> b 1 BTC"));
    bc.add_block(String::from("c -> d 1 BTC"));
    for b in bc.blocks  {
        println!("{:#?}", b);
    }
}
