mod cli;
mod server;
mod client;

fn main() -> anyhow::Result<()>{
    cli::run()
}