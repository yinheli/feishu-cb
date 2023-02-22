use actix_web::{middleware, web, App, HttpServer};
use clap::Parser;

mod cli;
mod feishu;
mod handler;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cli = cli::Cli::parse();
    match cli.command {
        cli::Command::Serve(args) => serve(args).await,
    };
}

async fn serve(args: cli::ServeArg) {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(feishu::FeiShu::new(
                args.webhook.clone(),
                args.secret.clone(),
            )))
            .wrap(middleware::Logger::default())
            .route("/", web::to(handler::version))
            .route("/cb/{tail}*", web::to(handler::cb))
    })
    .bind(args.bind)
    .unwrap()
    .run()
    .await
    .unwrap();
}
