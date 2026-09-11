fn main() {
    // The real "run this as fast as possible" miner loop comes later.
    // For now, this crate is developed and verified through the BDD
    // scenarios in features/randomx_hash.feature (run via `cargo test`).
    let key = b"test key 000";
    let input = b"This is a test";
    let (hash, telemetry) = randomx_miner::calculate_hash_with_telemetry(key, input);
    println!("{}", hex::encode(hash));
    eprintln!("{telemetry}");
}
