use clap::Parser;
use std::error::Error;
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};
use warp::Filter;

use tokio::sync::Mutex;

mod utils;

use auth_service_api::client::AuthService;

// response and request
mod request;
mod response;

// db web stuff
mod tournament_data_service;
mod tournament_year_service;
mod tournament_year_demand_service;
mod tournament_service;
mod tournament_submission_service;
mod tournament_membership_service;

mod api;
mod db_types;
mod handlers;

static SERVICE_NAME: &str = "production-minigame-service";

#[derive(Parser, Clone)]
struct Opts {
    #[clap(long)]
    site_external_url: String,
    #[clap(long)]
    database_url: String,
    #[clap(long)]
    auth_service_url: String,
    #[clap(long)]
    port: u16,
}

#[derive(Clone)]
pub struct Config {
    pub site_external_url: String,
}

pub type Db = Arc<Mutex<Client>>;

#[tokio::main]
async fn main() {
    let Opts {
        database_url,
        site_external_url,
        auth_service_url,
        port,
    } = Opts::parse();

    let (client, connection) = loop {
        match tokio_postgres::connect(&database_url, NoTls).await {
            Ok(v) => break v,
            Err(e) => utils::log(utils::Event {
                msg: e.to_string(),
                source: e.source().map(|x| x.to_string()),
                severity: utils::SeverityKind::Error,
            }),
        }

        // sleep for 5 seconds
        std::thread::sleep(std::time::Duration::from_secs(5));
    };

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let db: Db = Arc::new(Mutex::new(client));

    // open connection to auth service
    let auth_service = AuthService::new(&auth_service_url).await;

    let log = warp::log::custom(|info| {
        // Use a log macro, or slog, or println, or whatever!
        utils::log(utils::Event {
            msg: info.method().to_string(),
            source: Some(info.path().to_string()),
            severity: utils::SeverityKind::Info,
        });
    });

    let api = api::api(
        Config {
            site_external_url,
        },
        db,
        auth_service,
    );

    warp::serve(api.with(log)).run(([0, 0, 0, 0], port)).await;
}
