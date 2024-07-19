use clap::{CommandFactory, Parser, Subcommand};
use crate::cli::{commands};
use crate::cli::parser::Cli;

#[derive(Subcommand)]
pub enum Commands {
    /// 生成 UUID
    #[command(about = "生成 UUID", long_about = "生成指定数量的 UUID")]
    Uuid {
        /// 指定生成数量
        #[arg(help = "指定要生成的数量", default_value_t = 1)]
        num: u32,
    },
    /// 查询是否为 UUID
    #[command(about = "查询是否为 UUID", long_about = "检查提供的 UUID 是否有效")]
    Query {
        /// 要查询的 UUID
        #[arg(help = "要验证的 UUID 字符串")]
        uuid: String,
    },
    /// 查询工具版本
    #[command(about = "查询 timestamp", long_about = "查询当前 timestamp 单位：秒（s）")]
    Ts {
        #[arg(help = "要转换为时间戳的时间", default_value = "")]
        datetime: String,
    },
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Some(command) => match command {
            Commands::Uuid { num } => {
                commands::silo_uuid::handle(*num);
            },
            Commands::Query { uuid } => {
                commands::query_uuid::handle(&uuid);
            },
            Commands::Ts { datetime } => {
                commands::query_timestamp::handle(datetime);
            },
        },
        None => {
            let mut app = Cli::command();
            app.print_long_help().unwrap();
        }
    }
}
