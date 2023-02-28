#![allow(non_snake_case)]
mod cli;
mod server;
mod client;
mod IRemoteService;

fn main() -> anyhow::Result<()>{
    cli::run()
}