mod cli;
mod server;
mod client;
mod integration;

// fn main() -> anyhow::Result<()>{
//     cli::run()
// }

fn main() -> Result<(), &'static str> {
    integration::run()
}