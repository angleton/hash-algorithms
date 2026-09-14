fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap_or_else(|| "test key 000".to_owned());
    let input = args.next().unwrap_or_else(|| "This is a test".to_owned());
    if args.next().is_some() {
        eprintln!("usage: randomx-miner [key] [input]");
        std::process::exit(2);
    }

    let (hash, telemetry) =
        randomx_miner::calculate_hash_with_telemetry(key.as_bytes(), input.as_bytes());
    println!("{}", hex::encode(hash));
    eprintln!("\nRandomX light-mode telemetry");
    eprintln!("{telemetry}");
}
