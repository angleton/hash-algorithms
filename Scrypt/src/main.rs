fn main() {
    let key = scrypt_educational::derive_key(b"password", b"salt", 16, 1, 1, 32);
    println!("{}", hex::encode(key));
}
