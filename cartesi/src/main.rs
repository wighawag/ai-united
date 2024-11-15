use core::str;
use json::{object, JsonValue};
// use lz4_flex::decompress;
use machine::Executor;
use std::{env, slice};

fn execute_module(payload: &str) {
    let s = unsafe {
        // First, we build a &[u8]...
        let slice = slice::from_raw_parts(payload.as_ptr().add(2), payload.len() - 2);

        // ... and then convert that slice into a string slice
        str::from_utf8(slice)
    }
    .expect("failed to slice payload");

    let mut compressed_bytes = hex::decode(s).expect("failed to decode payload");

    // let mut decompressed_bytes =
    //     decompress(&compressed_bytes, 1000000).expect("failed to decompress in to 1,000,000 bytes");
    // let decompressed_bytes: &mut [u8] = &mut decompressed_bytes; //cast to `&mut [u8]`

    // let mut executor = Executor::new();
    // executor.execute(decompressed_bytes);
    let mut executor = Executor::new();
    let compressed_bytes: &mut [u8] = &mut compressed_bytes; //cast to `&mut [u8]`
    executor.execute(compressed_bytes);
}

pub async fn handle_advance(
    _client: &hyper::Client<hyper::client::HttpConnector>,
    _server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received advance request data {}", &request);
    let payload = request["data"]["payload"]
        .as_str()
        .ok_or("Missing payload")?;

    execute_module(payload);

    Ok("accept")
}

pub async fn handle_inspect(
    _client: &hyper::Client<hyper::client::HttpConnector>,
    _server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received inspect request data {}", &request);
    let _payload = request["data"]["payload"]
        .as_str()
        .ok_or("Missing payload")?;
    // TODO: add application logic here
    Ok("accept")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = hyper::Client::new();
    let server_addr = env::var("ROLLUP_HTTP_SERVER_URL")?;

    let mut status = "accept";
    loop {
        println!("Sending finish");
        let response = object! {"status" => status};
        let request = hyper::Request::builder()
            .method(hyper::Method::POST)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .uri(format!("{}/finish", &server_addr))
            .body(hyper::Body::from(response.dump()))?;
        let response = client.request(request).await?;
        println!("Received finish status {}", response.status());

        if response.status() == hyper::StatusCode::ACCEPTED {
            println!("No pending rollup request, trying again");
        } else {
            let body = hyper::body::to_bytes(response).await?;
            let utf = std::str::from_utf8(&body)?;
            let req = json::parse(utf)?;

            let request_type = req["request_type"]
                .as_str()
                .ok_or("request_type is not a string")?;
            status = match request_type {
                "advance_state" => handle_advance(&client, &server_addr[..], req).await?,
                "inspect_state" => handle_inspect(&client, &server_addr[..], req).await?,
                &_ => {
                    eprintln!("Unknown request type");
                    "reject"
                }
            };
        }
    }
}
