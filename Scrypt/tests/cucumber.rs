//! BDD harness for the full, end-to-end Scrypt feature.

use cucumber::{World, given, then, when};

#[derive(Debug, Default, World)]
pub struct ScryptWorld {
    password: Vec<u8>,
    salt: Vec<u8>,
    n: u32,
    r: u32,
    p: u32,
    output_len: usize,
    key: Vec<u8>,
}

#[given(expr = "the password {string} and salt {string}")]
fn given_password_and_salt(world: &mut ScryptWorld, password: String, salt: String) {
    world.password = password.into_bytes();
    world.salt = salt.into_bytes();
}

#[given(expr = "the Scrypt parameters N {int}, r {int}, p {int}, and output length {int}")]
fn given_parameters(world: &mut ScryptWorld, n: i32, r: i32, p: i32, output_len: i32) {
    world.n = n as u32;
    world.r = r as u32;
    world.p = p as u32;
    world.output_len = output_len as usize;
}

#[when("I derive the Scrypt key")]
fn when_derive(world: &mut ScryptWorld) {
    world.key = scrypt_educational::derive_key(
        &world.password,
        &world.salt,
        world.n,
        world.r,
        world.p,
        world.output_len,
    );
}

#[then(expr = "the resulting key should equal {string}")]
fn then_key_equals(world: &mut ScryptWorld, expected_hex: String) {
    let expected = hex::decode(expected_hex).expect("scenario key must be valid hex");
    assert_eq!(world.key, expected);
}

#[tokio::main]
async fn main() {
    ScryptWorld::run("features/full_hash").await;
}
