use std::ops::Sub;

use anyhow::Error;
use clap::{Parser, Subcommand};
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use scraper::{db, scrape_utils, web_server_utils};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<(), Error> {
    connection.run_pending_migrations(MIGRATIONS).unwrap();

    let mut conn = db::helpers::establish_connection()?;
    db::helpers::delete_all_past_streams(&mut conn)?;

    Ok(())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Parse the website for updated games")]
    Data {
        #[command(subcommand)]
        data_command: Option<DataCommands>,
    },
    #[clap(about = "Run the web server")]
    Serve {
        /// port to run the server on
        /// (default: 3000)
        /// usage: sportshub serve -p 5173
        #[clap(short, long, default_value = "3000")]
        port: u16,

        /// Whether to run the server in silent mode
        /// (default: false)
        /// usage: sportshub serve -s
        #[clap(short, long)]
        silent: bool,
    },
}

#[derive(Subcommand, Clone)]
enum DataCommands {
    #[clap(about = "Scrape the website for updated games")]
    Scrape {
        /// Whether to run the browser in headless mode
        /// (default: true)
        /// usage: sportshub scrape -H false
        #[clap(short = 'H', long = "headless", default_value = "true")]
        headless: bool,
    },
    #[clap(about = "Update the database with latest games links")]
    Update {
        /// How many tabs to use for scraping
        /// (default: 10)
        /// usage: sportshub update -t 20
        #[clap(short = 't', long = "tabs", default_value = "10")]
        tabs: u8,

        /// Whether to run the browser in headless mode
        /// (default: true)
        /// usage: sportshub update -H false
        /// usage: sportshub update -H false -t 20
        #[clap(short = 'H', long = "headless", default_value = "true")]
        headless: bool,
    },
    #[clap(about = "Get the info about the current database")]
    Info {},
    #[clap(about = "Delete all past streams")]
    Clear {},
}

#[rocket::main]
async fn main() {
    let mut conn = db::helpers::establish_connection().unwrap();
    run_migrations(&mut conn).unwrap();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Data { data_command }) => {
            match data_command {
                Some(DataCommands::Scrape { headless }) => {
                    scrape_utils::scrape_events(headless).unwrap();
                }
                Some(DataCommands::Update { tabs, headless }) => {
                    scrape_utils::update_streams(tabs as usize, headless).unwrap();
                }
                Some(DataCommands::Info {}) => {
                    let streams = db::helpers::get_streams(&mut conn).unwrap();
                    println!("Total events: {}", streams.len());
                    let empty_streams = db::helpers::get_empty_streams(&mut conn).unwrap();
                    println!("Empty events: {}", empty_streams.len());
                    let linked_streams = db::helpers::get_linked_streams(&mut conn).unwrap();
                    println!("Linked events: {}", linked_streams.len());

                    let mut total_links = 0;

                    for stream in linked_streams {
                        if stream.stream_link != "" {
                            total_links += stream.stream_link.split(",").count();
                        }
                    }

                    println!("Total streams: {}", total_links);
                }
                Some(DataCommands::Clear {}) => {
                    db::helpers::delete_all_streams(&mut conn).unwrap();
                }
                None => {
                    println!("use sportshub data -h for help");
                }
            }
        }
        Some(Commands::Serve { port, silent }) => {
            web_server_utils::run(port, silent).await.unwrap();
        }
        None => {
            println!("use sportshub -h for help");
        }
    }
}
