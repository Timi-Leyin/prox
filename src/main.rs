use clap::Parser;
mod constants;
mod cli;
mod server;
mod utils;
#[actix_web::main]
async  fn main() {
    let args = cli::Args::parse();
    server::server(args.port.parse::<u16>().ok()).await.unwrap();
}
