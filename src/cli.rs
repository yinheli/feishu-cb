use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Subcommands
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run server to receive webhook
    Serve(ServeArg),
}

#[derive(clap::Args)]
pub struct ServeArg {
    /// bind
    #[arg(env = "FEISHU_CB_BIND", default_value = "0.0.0.0:8080")]
    pub bind: String,

    /// feishu webhook
    #[arg(env = "FEISHU_CB_WEBHOOK", short, long)]
    pub webhook: String,
    /// webhook secret
    #[arg(env = "FEISHU_CB_SECRET", short, long, requires = "webhook")]
    pub secret: Option<String>,
}
