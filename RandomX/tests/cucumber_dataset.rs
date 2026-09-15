//! BDD harness for the `dataset` register-seeding step: `features/dataset/`.
//! Run with `cargo test --test cucumber_dataset`.

use cucumber::{World, given, then, when};

#[derive(Debug, Default, World)]
pub struct DatasetWorld {
    item_number: u64,
    registers: [u64; 8],
}

#[given(expr = "the dataset item number {int}")]
fn given_item_number(world: &mut DatasetWorld, item_number: u64) {
    world.item_number = item_number;
}

#[when("I seed the dataset item registers")]
fn when_seed(world: &mut DatasetWorld) {
    world.registers = randomx::dataset::seed_registers(world.item_number);
}

#[then(regex = r#"^register r(\d) should equal "(.+)"$"#)]
fn then_register(world: &mut DatasetWorld, index: usize, expected_hex: String) {
    let expected = u64::from_str_radix(expected_hex.trim_start_matches("0x"), 16)
        .expect("scenario hex value must be valid hex");
    assert_eq!(world.registers[index], expected, "register r{index}");
}

#[tokio::test]
async fn cucumber_dataset() {
    DatasetWorld::cucumber()
        .with_default_cli()
        .run("features/dataset")
        .await;
}
