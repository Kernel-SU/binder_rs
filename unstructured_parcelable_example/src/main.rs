#![allow(non_snake_case)]
mod cli;
mod server;
mod client;
mod IMySimpleParcelableService;
mod my_simple_parcelable;

fn main() -> anyhow::Result<()>{
    cli::run()
}