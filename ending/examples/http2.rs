use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let url = "https://www.rust-lang.org/";

    let _body = reqwest::blocking::get(url).unwrap().text().unwrap();
    info!("Fetching url: {}", url);
}
