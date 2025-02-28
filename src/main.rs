mod config;
mod handlers;
mod helpers;
mod models;
mod routes;

use bcrypt::{DEFAULT_COST, hash};
use config::config::Config;

use clap::{Arg, Command};
use rand::Rng;
use sqlx::SqlitePool;
use std::{error::Error, sync::OnceLock};
use tokio::fs;
use uuid::Uuid;

static APP_CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Clone)]
pub struct AppState {
    pub db_pool: SqlitePool,
}

fn make_app_args() {
    let matches = Command::new("newsletter")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Chemin vers le fichier de configuration")
                .default_value("./config.cfg"),
        )
        .arg(
            Arg::new("init-db")
                .long("init-db")
                .help("Initialiser la base de données")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let config_path = matches
        .get_one::<String>("config")
        .expect("valeur config invalide")
        .to_owned();

    let conf =
        Config::from_file(&config_path).expect("Erreur lors du chargement de la configuration");
    APP_CONFIG.set(conf).expect("APP_CONFIG déjà initialisé");
}

async fn init_db(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    let migration_sql = fs::read_to_string("migration/init.sql").await?;
    sqlx::query(&migration_sql).execute(pool).await?;

    // Create a new admin user.
    let user_id = Uuid::new_v4().to_string();
    let email = "admin@example.com".to_string();
    // Generate a random 12-character alphanumeric password.
    let password_plain: String = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();
    let hashed_password = hash(&password_plain, DEFAULT_COST)?;
    let role = "admin";

    sqlx::query("INSERT INTO users (id, email, password, role) VALUES (?, ?, ?, ?)")
        .bind(&user_id)
        .bind(&email)
        .bind(&hashed_password)
        .bind(&role)
        .execute(pool)
        .await?;

    println!("Initialized database with new admin user:");
    println!("Email: {}", email);
    println!("Password: {}", password_plain);

    Ok(())
}

#[tokio::main]
async fn main() {
    make_app_args();

    let init_db_flag = std::env::args().any(|arg| arg == "--init-db");

    let config = APP_CONFIG.get().expect("Configuration non initialisée");
    let sqlite_filename = &config.database.sqlite.filename;

    let db_path = std::path::Path::new(sqlite_filename);
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create database directory");
    }
    if !db_path.exists() {
        std::fs::File::create(db_path).expect("Failed to create database file");
    }

    let connection_str = format!("sqlite://{}", sqlite_filename);
    let pool = SqlitePool::connect(&connection_str)
        .await
        .expect("Failed to create SQLite pool");

    if init_db_flag {
        if let Err(e) = init_db(&pool).await {
            eprintln!("Failed to initialize the database: {}", e);
            std::process::exit(1);
        }
        return;
    }

    let state = AppState { db_pool: pool };

    let app = routes::create_routes(&state);

    let listener = tokio::net::TcpListener::bind(config.server.host.clone())
        .await
        .unwrap();
    println!("app running on {:?}", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}
