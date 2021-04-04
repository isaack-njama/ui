use bitcoin_hashes::hex::ToHex;
use config::{load_from_file, ClientConfig};
use mint_api::{Amount, Coins};
use mint_client::{MintClient, SpendableCoin};
use std::path::PathBuf;
use structopt::StructOpt;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[derive(StructOpt)]
struct Options {
    workdir: PathBuf,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(
        about = "Issue tokens in exchange for a peg-in proof (not yet implemented, just creates coins)"
    )]
    PegIn { amount: Amount },
    #[structopt(about = "Reissue tokens received from a third party to avoid double spends")]
    Reissue {
        #[structopt(parse(from_str = parse_coins))]
        coins: Coins<SpendableCoin>,
    },
    #[structopt(about = "Prepare coins to send to a third party as a payment")]
    Spend { amount: Amount },
    #[structopt(about = "Fetch (re-)issued coins and finalize issuance process")]
    Fetch,
    #[structopt(about = "Display wallet info (holdings, tiers)")]
    Info,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let opts: Options = StructOpt::from_args();
    let cfg_path = opts.workdir.join("client.json");
    let db_path = opts.workdir.join("client.db");
    let cfg: ClientConfig = load_from_file(&cfg_path);
    let db = sled::open(&db_path)
        .unwrap()
        .open_tree("mint-client")
        .unwrap();

    let mut rng = rand::rngs::OsRng::new().unwrap();

    let client = MintClient::new(cfg, db);

    match opts.command {
        Command::PegIn { amount } => {
            info!("Starting peg-in transaction for {}", amount);
            let id = client.peg_in(amount, &mut rng).await.unwrap();
            info!(
                "Started peg-in {}, please fetch the result later",
                id.to_hex()
            );
        }
        Command::Reissue { coins } => {
            info!("Starting reissuance transaction for {}", coins.amount());
            let id = client.reissue(coins, &mut rng).await.unwrap();
            info!(
                "Started reissuance {}, please fetch the result later",
                id.to_hex()
            );
        }
        Command::Spend { amount } => {
            match client.coins().select_coins(amount) {
                Some(outgoing_coins) => {
                    client.spend_coins(&outgoing_coins);
                    println!("{}", serialize_coins(&outgoing_coins));
                }
                None => {
                    error!("Not enough funds or in wrong denominations!")
                }
            };
        }
        Command::Fetch => {
            for id in client.fetch_all(&mut rng).await.unwrap() {
                info!("Fetched coins from issuance {}", id.to_hex());
            }
        }
        Command::Info => {
            let coins = client.coins();
            info!(
                "We own {} coins with a total value of {}",
                coins.coin_count(),
                coins.amount()
            );
            for (amount, coins) in coins.coins {
                info!("We own {} coins of denomination {}", coins.len(), amount);
            }
        }
    }
}

fn parse_coins(s: &str) -> Coins<SpendableCoin> {
    let bytes = base64::decode(s).unwrap();
    bincode::deserialize(&bytes).unwrap()
}

fn serialize_coins(c: &Coins<SpendableCoin>) -> String {
    let bytes = bincode::serialize(&c).unwrap();
    base64::encode(&bytes)
}