mod args;
mod config;
mod handlers;
mod helpers;
mod models;
mod routes;
mod telemetry;

use args::Args;
use bcrypt::{DEFAULT_COST, hash};
use clap::Parser;
use config::config::Config;
use helpers::email::Email;
use rand::Rng;
use sqlx::SqlitePool;
use std::{error::Error, sync::OnceLock};
use uuid::Uuid;

static APP_CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Clone)]
pub struct AppState {
    pub db_pool: SqlitePool,
}

async fn init_db(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    let migration_sql = include_str!("../migration/init.sql");
    sqlx::query(migration_sql).execute(pool).await?;

    let user_id = Uuid::new_v4().to_string();
    let email = "admin@example.com";
    let password_plain: String = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();
    let hashed_password = hash(&password_plain, DEFAULT_COST)?;
    let role = "admin";

    sqlx::query("insert into users (id, email, password, role) values (?, ?, ?, ?)")
        .bind(&user_id)
        .bind(email)
        .bind(&hashed_password)
        .bind(role)
        .execute(pool)
        .await?;

    println!(
        "Database initialized with admin user:\nEmail: {}\nPassword: {}",
        email, password_plain
    );

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let conf = Config::from_file(&args.file_path).expect("Error loading configuration");

    APP_CONFIG
        .set(conf)
        .expect("APP_CONFIG already initialized");

    let config = APP_CONFIG.get().expect("Configuration not initialized");

    telemetry::init_telemetry();

    Email::init(&config.email);

    let sqlite_db_file_path = &config.database.sqlite.file_path;

    if let Some(parent) = sqlite_db_file_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create database directory");
    }
    if !sqlite_db_file_path.exists() {
        std::fs::File::create(sqlite_db_file_path).expect("Failed to create database file");
    }

    let connection_str = format!(
        "sqlite://{}",
        sqlite_db_file_path
            .to_str()
            .expect("Invalid sqlite database file path")
    );
    let pool = SqlitePool::connect(&connection_str)
        .await
        .expect("Failed to create SQLite pool");

    if args.init_db {
        if let Err(e) = init_db(&pool).await {
            eprintln!("Failed to initialize database: {}", e);
            std::process::exit(1);
        }
        return;
    }

    let state = AppState { db_pool: pool };

    let app = routes::create_routes(&state);

    let listener = tokio::net::TcpListener::bind(config.server.host.clone())
        .await
        .expect("Failed to bind server address");
    println!("App running on {:?}", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}
