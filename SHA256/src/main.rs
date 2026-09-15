fn main() {
    let message = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let message = if message.is_empty() {
        "abc".to_owned()
    } else {
        message
    };
    let (digest, telemetry) = sha256::digest_with_telemetry(message.as_bytes());
    println!("{}", hex::encode(digest));
    eprintln!("\nSHA-256 telemetry");
    eprintln!("{telemetry}");
}