use clap::Parser;
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Network address of ONVIF camera, eg. "http://192.0.2.123
    uri: url::Url,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let device = camera_query::onvif::Device::from(args.uri);
    match device {
        Err(error) => {
            eprintln!("Failed to parse uri: {}", error);
            exit(1);
        }
        Ok(device) => {
            println!("Inquiring {}...", device.url);
            let result = camera_query::query::query(&device).await;

            println!();
            println!("{}", result);
        }
    }
}
