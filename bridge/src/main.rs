mod infra;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::from_filename("./.env.prod").expect("Failed to read .env");
    env_logger::init();
    println!("Hello, world!");
}
