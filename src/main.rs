use clap::{command, CommandFactory, Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "silo", version, about = "实用命令行工具", long_about = None)]
struct Cli {
    /// 子命令
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 生成 UUID
    #[command(about = "生成 UUID", long_about = "生成指定版本 4 的 UUID")]
    Uuid {
        /// 指定生成数量
        #[arg(default_value_t = 1, help = "指定要生成的数量）")]
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
    #[command(about = "查询工具版本", long_about = "显示此 CLI 工具的版本")]
    Version,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(command) => match command {
            Commands::Uuid { num } => {
                println!("生成的 UUIDv4:");
                for _i in 0..*num {
                    let my_uuid = Uuid::new_v4();
                    println!("{}", my_uuid);
                }
            },
            Commands::Query { uuid } => match Uuid::parse_str(uuid) {
                Ok(parsed_uuid) => println!("有效的 UUID: {}", parsed_uuid),
                Err(_) => eprintln!("无效的 UUID: {}", uuid),
            },
            Commands::Version => {
                let version = env!("CARGO_PKG_VERSION");
                println!("UUID 工具版本 {}", version);
            }
        },
        None => {
            let mut app = Cli::command();
            // app = app.override_help("显示此消息或指定子命令的帮助信息");
            app.print_long_help().unwrap();
        }
    }
}
