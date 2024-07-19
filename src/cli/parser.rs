use clap::{Parser};
use crate::cli::command::Commands;

#[derive(Parser)]
#[command(name = "silo", version, about = "实用命令行工具", long_about = None)]
pub struct Cli {
    /// 子命令
    #[command(subcommand)]
    pub command: Option<Commands>,
}
