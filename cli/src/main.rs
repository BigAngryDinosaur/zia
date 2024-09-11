use clap::Parser;
use shared::CompletionAPI;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    prompt: String,

    #[arg(short, long)]
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let api = CompletionAPI::new();
    let response = api.fetch_completion(args.prompt, args.text).await?;

    println!("{}", response);

    Ok(())
}
