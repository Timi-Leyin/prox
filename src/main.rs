use clap::Parser;
mod constants;
mod cli;
mod server;
fn main() {
    let args = cli::Args::parse();
    server::server(args.port.parse::<u16>().ok());
}
