use clap::Parser;
use lz4_flex::compress;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct BotWasmData {
    wasm: String,
    uncompressed: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file to compress and convert
    #[arg(short, long)]
    input: PathBuf,

    /// Output file to save the hex string
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    // Read input file
    let mut input_file = File::open(&cli.input).expect("failed to open file");
    let mut input_data = Vec::new();
    input_file
        .read_to_end(&mut input_data)
        .expect("failed to read file");

    let uncompressed_hex = {
        // Convert compressed data to hex string
        let mut hex_string = hex::encode(&input_data);
        hex_string.insert_str(0, "0x");
        hex_string
    };
    println!("{}", uncompressed_hex);

    // Compress the data
    // let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    // encoder.write_all(&input_data)?;
    // let compressed_data = encoder.finish()?;
    let compressed_data = compress(&input_data);

    let bot_module = {
        // Convert compressed data to hex string
        let mut hex_string = hex::encode(&compressed_data);
        hex_string.insert_str(0, "0x");
        BotWasmData {
            wasm: hex_string,
            uncompressed: uncompressed_hex,
        }
    };

    // Serialize it to a JSON string.
    let json = serde_json::to_string(&bot_module).expect("failed to serialize");

    // Write hex string to output file
    let mut output_file = File::create(&cli.output).expect("failed to create file");
    output_file
        .write_all(json.as_bytes())
        .expect("faile dto write file");

    println!("File compressed and converted to hex successfully!");
    println!("{}", bot_module.wasm);
}
