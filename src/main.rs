use benchmark::Benchmark;

mod benchmark;
mod config;

#[tokio::main]
async fn main() {
    let mut benchmark = Benchmark::new();
    if let Err(e) = benchmark.bench().await {
        eprintln!("{}", e);
    }
}
