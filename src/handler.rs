use crate::feishu;
use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

pub async fn version(_: HttpRequest) -> impl Responder {
    format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

pub async fn cb(
    feishu: web::Data<feishu::FeiShu>,
    req: HttpRequest,
    body: String,
) -> impl Responder {
    let mut dump = String::with_capacity(1024 * 8);
    dump.push_str(&format!("feishu-cb dump: {}\n", req.peer_addr().unwrap()));
    dump.push_str(&format!(
        "{} {} {:?}\n",
        req.method(),
        req.uri(),
        req.version()
    ));
    for (name, value) in req.headers() {
        dump.push_str(&format!(
            "{}: {}\n",
            name,
            String::from_utf8_lossy(value.as_bytes())
        ));
    }

    dump.push_str(&format!("\n{}", body));

    tokio::spawn(async move { feishu.send_text(dump).await.unwrap() });

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json!({}).to_string())
}
