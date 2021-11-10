use crate::config::Config;
use reqwest::{Client, Proxy};
use std::error::Error;
use tokio::time::Instant;

#[derive(Debug)]
pub struct Benchmark {
    config: Config,
    client: Client,
    transfer: usize,
    elapsed: u128,
}

impl Benchmark {
    pub fn new() -> Self {
        let config = Config::new();
        let builder = match config.proxy {
            Some(ref proxy) => Client::builder().proxy(Proxy::all(proxy).unwrap()),
            None => Client::builder().no_proxy(),
        };

        Self {
            config,
            client: builder.build().unwrap(),
            transfer: 0,
            elapsed: 0,
        }
    }

    pub async fn bench(&mut self) -> Result<(), Box<dyn Error>> {
        for _ in 0..self.config.number {
            let now = Instant::now();
            let resp = self.client.get(&self.config.target).send().await?;
            let elapsed = now.elapsed().as_millis();
            let len = resp.bytes().await?.len();
            self.elapsed += elapsed;
            self.transfer += len;
        }
        self.report();
        Ok(())
    }

    pub fn report(&self) {
        println!("Transfer: {:} bytes", self.transfer);
        println!("Time: {:?}ms", self.elapsed);
        println!(
            "Speed: {:.2}MB/s",
            (self.transfer as f64 / ((self.elapsed as f64) / 1000.) / 1024. / 1024.)
        );
    }
}
