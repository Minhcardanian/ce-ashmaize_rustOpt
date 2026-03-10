#![cfg(target_arch = "wasm32")]

use ashmaize_web::{RomBuilder, RomBuilderError};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const B: usize = 1;
const KB: usize = 1_024 * B;
const MB: usize = 1_024 * KB;

const DEFAULT_KEY: [u8; 32] = [0; 32];

#[wasm_bindgen_test]
fn rom_builder_missing_key() {
    let mut builder = RomBuilder::new();
    builder.size(1 * MB);
    builder.gen_full_random();

    assert!(matches!(builder.build(), Err(RomBuilderError::MissingKey)));
}

#[wasm_bindgen_test]
fn rom_builder_missing_size() {
    let mut builder = RomBuilder::new();
    builder.key(&DEFAULT_KEY);
    builder.gen_full_random();

    assert!(matches!(builder.build(), Err(RomBuilderError::MissingSize)));
}

#[wasm_bindgen_test]
fn rom_builder_missing_gen_type() {
    let mut builder = RomBuilder::new();
    builder.size(1 * MB);
    builder.key(&DEFAULT_KEY);

    assert!(matches!(
        builder.build(),
        Err(RomBuilderError::MissingGenType)
    ));
}

#[wasm_bindgen_test]
fn rom_builder_pre_size_not_power_of_two() {
    let mut builder = RomBuilder::new();
    builder.size(1 * MB);
    builder.key(&DEFAULT_KEY);
    builder.gen_two_steps(17, 8);

    assert!(matches!(
        builder.build(),
        Err(RomBuilderError::PreSizeNotPowerOfTwo)
    ));
}

#[wasm_bindgen_test]
fn rom_build_size_0() {
    let mut builder = RomBuilder::new();
    builder.size(0 * MB);
    builder.key(&DEFAULT_KEY);
    builder.gen_full_random();

    assert!(matches!(builder.build(), Err(RomBuilderError::SizeIsZero)));
}

#[wasm_bindgen_test]
fn rom_build_full_random() {
    let mut builder = RomBuilder::new();
    builder.size(1 * MB);
    builder.key(&DEFAULT_KEY);
    builder.gen_full_random();

    assert!(matches!(builder.build(), Ok(..)));
}

#[wasm_bindgen_test]
fn rom_build_two_steps() {
    let mut builder = RomBuilder::new();
    builder.size(1 * MB);
    builder.key(&DEFAULT_KEY);
    builder.gen_two_steps(256, 8);

    assert!(matches!(builder.build(), Ok(..)));
}

#[wasm_bindgen_test]
fn rom_hash() {
    const PRE_SIZE: usize = 16 * 1024;
    const SIZE: usize = 10 * 1024 * 1024;
    const NB_INSTR: u32 = 256;
    const EXPECTED: [u8; 64] = [
        162, 175, 18, 231, 188, 219, 174, 96, 166, 86, 46, 23, 99, 176, 96, 55, 155, 198, 229, 53,
        1, 212, 219, 197, 164, 176, 34, 105, 205, 116, 105, 157, 21, 180, 90, 106, 48, 149, 144,
        21, 132, 228, 244, 141, 59, 48, 255, 223, 234, 120, 14, 110, 43, 211, 31, 15, 218, 42, 134,
        71, 202, 226, 237, 230,
    ];

    let mut builder = RomBuilder::new();
    builder.size(SIZE);
    builder.key(b"123");
    builder.gen_two_steps(PRE_SIZE, 4);

    let rom = builder.build().unwrap();
    let hash = rom.hash(b"hello", 8, NB_INSTR);

    assert_eq!(hash, EXPECTED);
}
