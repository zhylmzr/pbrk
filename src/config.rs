use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "zhylmzr <zhylmzr@gmail.com>")]
pub struct Config {
    #[clap(short = 'x', about = "http or socks5 support")]
    pub proxy: Option<String>,
    #[clap(about = "http/https url")]
    pub target: String,
    #[clap(short = 'n', about = "request numbers", default_value = "1")]
    pub number: u64,
}

impl Config {
    pub fn new() -> Self {
        Config::parse()
    }
}
