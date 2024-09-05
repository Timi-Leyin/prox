use clap::Parser;
use crate::constants;

#[derive(Parser, Debug)]
#[command(author=constants::AUTHOR ,version=constants::VERSION, about=constants::ABOUT, long_about = constants::INFO)]
pub struct Args{
    #[arg(short, long, default_value_t=constants::DEFAULT_PORT.to_string())]
    pub port:String,

    #[arg(short, long)]
    pub origin:String,

    #[arg(short, long)]
    pub clear:bool
}