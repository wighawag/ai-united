use core::str;
use std::slice;

use clap::Parser;
use lz4_flex::decompress;
use machine::Battle;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct WasmData {
    wasm: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file to compress and convert
    #[arg(short, long)]
    bot: String,
}

fn main() {
    let cli = Cli::parse();

    let payload: &str = &cli.bot.as_str();

    println!("payload: {}", payload);

    let compressed_bytes = if payload.starts_with("0x") {
        let s = unsafe {
            // First, we build a &[u8]...
            let slice = slice::from_raw_parts(payload.as_ptr().add(2), payload.len() - 2);

            // ... and then convert that slice into a string slice
            str::from_utf8(slice)
        }
        .expect("failed to slice payload");

        hex::decode(s).expect("failed to decode payload")
    } else {
        // Open the file
        let file = File::open(payload).expect("failed to read file");
        let reader = BufReader::new(file);
        // Parse the JSON
        let wasm_data: WasmData = serde_json::from_reader(reader).expect("fauled to parse json");

        let s = unsafe {
            // First, we build a &[u8]...
            let slice =
                slice::from_raw_parts(wasm_data.wasm.as_ptr().add(2), wasm_data.wasm.len() - 2);

            // ... and then convert that slice into a string slice
            str::from_utf8(slice)
        }
        .expect("failed to slice payload");

        hex::decode(s).expect("failed to decode payload")
    };

    let mut wasm_bytes_1 = if let Ok(b) = decompress(&compressed_bytes, 1000000) {
        b
    } else {
        println!("could not decompress, assume it is not compressed");
        compressed_bytes.clone()
    };
    let wasm_bytes_1: &mut [u8] = &mut wasm_bytes_1; //cast to `&mut [u8]`

    let mut wasm_bytes_2 = if let Ok(b) = decompress(&compressed_bytes, 1000000) {
        b
    } else {
        println!("could not decompress, assume it is not compressed");
        compressed_bytes.clone()
    };
    let wasm_bytes_2: &mut [u8] = &mut wasm_bytes_2; //cast to `&mut [u8]`

    let mut battle = Battle::new();
    println!("addding bot 1 ...");
    battle.add_bot(wasm_bytes_1);
    println!("addding bot 2 ...");
    battle.add_bot(wasm_bytes_2);

    println!("battle!");
    battle.execute();
}
