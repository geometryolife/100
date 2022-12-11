use chain_core::blockchain;
use std::thread;
use std::time::Duration;

fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();

    println!("Start mining ...");
    thread::sleep(Duration::from_secs(5));
    bc.add_block("alice -> bob: 5 BTC".to_string());
    println!("Produce a block!");

    println!();

    println!("Start mining ...");
    thread::sleep(Duration::from_secs(5));
    bc.add_block(String::from("joe -> tom: 1 BTC"));
    println!("Produce a block!");

    for b in bc.blocks {
        println!("++++++++++++++++++++++++++++++++++++++");
        println!("{:#?}", b);
        println!("");
    }
}
