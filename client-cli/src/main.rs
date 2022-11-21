use std::io::Write;

use clap::Parser;
use log::info;

mod config;

#[derive(Debug, clap::Parser)]
struct Cli {
    /// Lite server config url or path to the file
    #[clap(
        short = 'c',
        long,
        default_value = "https://ton-blockchain.github.io/global.config.json"
    )]
    lite_client_config: String,

    #[clap(short, long, default_value = "/tmp/ton/keys")]
    key_store_dir: std::path::PathBuf,

    #[clap(short, long, default_value = "10")]
    request_timeout_seconds: u64,

    #[clap(short, long, default_value = "1")]
    log_level: i8,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    AccountState {
        address: String,
    },
    Transactions {
        address: String,

        #[clap(long)]
        from_tx_lt: Option<String>,

        #[clap(long)]
        from_tx_hash: Option<String>,

        #[clap(long)]
        to_tx_lt: Option<String>,

        #[clap(short, long)]
        limit: Option<usize>,
    },
}

fn main() {
    env_logger::builder()
        .format_timestamp(Some(env_logger::fmt::TimestampPrecision::Millis))
        .init();

    let cli = Cli::parse();
    info!("cli settings: {cli:?}");

    let lite_server_config = config::get_lite_server_config(&cli.lite_client_config);
    let mut client = ton_rs_client::client::Client::new(&ton_rs_client::client::Config {
        lite_server_config,
        keystore_dir: cli.key_store_dir.to_str().unwrap().to_string(),
        request_timeout: std::time::Duration::from_secs(cli.request_timeout_seconds),
        log_level: cli.log_level,
    });

    match &cli.command {
        Some(Commands::AccountState { address }) => {
            let fut = client.get_account_state(address);
            let state = futures::executor::block_on(fut);
            info!("account state:");
            std::io::stdout()
                .write_all(serde_json::to_string_pretty(&state).unwrap().as_bytes())
                .unwrap();
            print!("\n");
        }

        Some(Commands::Transactions {
            address,
            from_tx_lt,
            from_tx_hash,
            to_tx_lt,
            limit,
        }) => {
            let fut = client.get_transactions(
                address,
                from_tx_lt.clone(),
                from_tx_hash.clone(),
                to_tx_lt.clone(),
                limit.clone(),
            );
            let state = futures::executor::block_on(fut);
            info!("transactions:");
            std::io::stdout()
                .write_all(serde_json::to_string_pretty(&state).unwrap().as_bytes())
                .unwrap();
            std::io::stdout().flush().unwrap();
            print!("\n");
        }

        None => {
            println!("sub command not found")
        }
    }
}
