use clap::Parser;
use std::process;
use zwl_parser::{ywallet::YWallet, zwl::ZecWalletLite, Wallet};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Convert ZecWallet Lite data to the YWallet format.",
    long_about = None
)]
#[command(group = clap::ArgGroup::new("source").required(true).args(["from_seed", "from_path"]))]
struct Cli {
    /// Seed phrase used to generate a new wallet
    #[arg(long)]
    from_seed: Option<String>,
    /// Path to an existing ZecWallet Lite wallet file
    #[arg(long)]
    from_path: Option<String>,
    /// Number of addresses to derive when using --from-seed
    #[arg(long)]
    num_addresses: Option<u32>,
    /// Output file for the generated YWallet database
    #[arg(long, default_value = "generated_from_zwl.db")]
    output_file: String,
}

fn main() {
    let cli = Cli::parse();

    let wallet_result = if let Some(seed) = cli.from_seed.as_deref() {
        let num_addresses = cli.num_addresses.unwrap_or(1);
        if num_addresses == 0 {
            eprintln!("--num-addresses must be greater than zero when using --from-seed.");
            process::exit(1);
        }
        ZecWalletLite::from_seed_phrase(seed, num_addresses)
    } else if let Some(path) = cli.from_path.as_deref() {
        Wallet::parse::<ZecWalletLite>(path)
    } else {
        unreachable!("clap enforces one of --from-seed or --from-path");
    };

    let wallet = match wallet_result {
        Ok(wallet) => wallet,
        Err(err) => {
            eprintln!("Failed to create wallet: {}", err);
            process::exit(1);
        }
    };
    
    if let Err(err) = wallet.write::<YWallet>(&cli.output_file) {
        eprintln!("Failed to write wallet: {}", err);
        process::exit(1);
    }

    println!("Wallet exported to {}", cli.output_file);
}
