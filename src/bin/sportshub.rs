use clap::{Parser, Subcommand};
use scraper::{
    db::{self, helpers::run_migrations},
    scrape,
    web_server_routes,
};


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

        /// Do a full data refresh before hosting server?  
        /// (default: false)
        /// usage: sportshub serve -F
        /// usage: sportshub serve -F -s
        #[clap(short = 'F', long = "full-refresh")]
        full_refresh: bool,
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
        /// usage: sportshub update -H
        /// usage: sportshub update -H -t 20
        #[clap(short = 'H', long = "headless")]
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

    run_migrations(&mut db::helpers::establish_connection().unwrap(), &mut conn).unwrap();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Data { data_command }) => {
            match data_command {
                Some(DataCommands::Scrape { headless }) => {
                    scrape::scrape_events(headless).unwrap();
                }
                Some(DataCommands::Update { tabs, headless }) => {
                    scrape::update_streams(tabs as usize, headless).unwrap();
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
        Some(Commands::Serve {
            port,
            silent,
            full_refresh,
        }) => {
            if full_refresh {
                scrape::start_scraping(10, true).unwrap();
            }
            web_server_routes::run(port, silent).await.unwrap();
        }
        None => {
            println!("use sportshub -h for help");
        }
    }
}
