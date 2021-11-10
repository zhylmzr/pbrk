use std::error::Error;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Error params\n");
        print_help();
        std::process::exit(-1);
    }

    let proxy = &args[2];
    let target = &args[3];

    if let Err(e) = bench(proxy, target).await {
        eprintln!("{}", e);
    }
}

async fn bench(proxy: &str, target: &str) -> Result<(), Box<dyn Error>> {
    let now = tokio::time::Instant::now();
    let builder = reqwest::Client::builder();
    let client = builder.proxy(reqwest::Proxy::http(proxy)?).build()?;
    let resp = client.get(target).send().await?;
    let elapsed = now.elapsed().as_millis();
    let len = resp.bytes().await?.len();
    println!("Transfer: {:.2} bytes", len);
    println!("Time: {:?}ms", elapsed);
    println!(
        "Speed: {:.2}MB/s",
        (len as f64 / ((elapsed as f64) / 1000.) / 1024. / 1024.)
    );
    Ok(())
}

fn print_help() {
    println!("Usage: pbrk config target");
    println!("    config:");
    println!("        -x proxy, http or socks5 support");
    println!();
    println!("    example:");
    println!("        pbrk -x socks5://127.0.0.1:1080 http://127.0.0.1:8080");
}
