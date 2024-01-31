use anyhow::Error;
use clap::{Parser, Subcommand};
use diesel::{sqlite::Sqlite};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use scraper::{db, scrape_utils, web_server_utils};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<(), Error> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS).unwrap();

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
    #[clap(about = "Parse the website for updated data")]
    Parse {
        /// The number of tabs to open for checking stream links
        tabs: usize,
    },
    #[clap(about = "Run the web server")]
    Server,
}

#[tokio::main]
async fn main() {
    let mut conn = db::establish_connection();
    run_migrations(&mut conn).unwrap();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Parse { tabs }) => {
            scrape_utils::start_scraping(tabs);
        }
        Some(Commands::Server) => {
            web_server_utils::run().await;
        }
        None => {
            println!("use sportshub -h for help");
        }
    }
}
