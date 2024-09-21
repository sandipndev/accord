pub mod config;
mod db;

use anyhow::Context;
use clap::Parser;
use std::{fs, path::PathBuf};

use self::config::{Config, EnvOverride};

#[derive(Parser)]
#[clap(version, long_about = None)]
struct Cli {
    #[clap(short, long, env = "ACCORDE_CONFIG", value_name = "FILE")]
    config: Option<PathBuf>,
    #[clap(
        long,
        env = "ACCORDE_HOME",
        default_value = ".accorde",
        value_name = "DIRECTORY"
    )]
    accorde_home: String,
    #[clap(env = "PG_CON")]
    pg_con: String,
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = Config::load_config(cli.config, EnvOverride { db_con: cli.pg_con })?;

    run_cmd(&cli.accorde_home, config).await?;

    Ok(())
}

async fn run_cmd(accorde_home: &str, config: Config) -> anyhow::Result<()> {
    store_server_pid(accorde_home, std::process::id())?;
    let pool = db::init_pool(&config.db).await?;
    let app = crate::app::AccordeApp::run(pool, config.app).await?;
    crate::server::run(config.server, app).await?;
    Ok(())
}

pub fn store_server_pid(accorde_home: &str, pid: u32) -> anyhow::Result<()> {
    create_accorde_dir(accorde_home)?;
    let _ = fs::remove_file(format!("{accorde_home}/server-pid"));
    fs::write(format!("{accorde_home}/server-pid"), pid.to_string()).context("Writing PID file")?;
    Ok(())
}

fn create_accorde_dir(accorde_home: &str) -> anyhow::Result<()> {
    let _ = fs::create_dir(accorde_home);
    Ok(())
}
