use rust_blockchain::cli::cli::Cli;
use rust_blockchain::core::miner::Host;
use rust_blockchain::core::transaction::Transaction;

// TODO rocksDb
fn main() {
    let mut host = Host::new();

    let tx = Transaction::new([2; 32], [3; 32], 3, 1, 0, "".to_string());
    host.mining(&mut vec![tx]);
    let tx = Transaction::new([4; 32], [5; 32], 5, 1, 0, "".to_string());
    host.mining(&mut vec![tx]);

    host.print();
}
