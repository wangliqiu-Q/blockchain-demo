use blockchain_demo::cli::cli::Cli;
use blockchain_demo::core::miner::Host;
use blockchain_demo::core::transaction::Transaction;

// TODO rocksDb
fn main() {
    let mut host = Host::new();

    let tx = Transaction::new([2; 32], [3; 32], 3, 1, 0, "".to_string());
    host.mining(&mut vec![tx]);
    let tx = Transaction::new([4; 32], [5; 32], 5, 1, 0, "".to_string());
    host.mining(&mut vec![tx]);

    host.print();
}
