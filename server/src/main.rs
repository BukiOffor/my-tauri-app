use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};
use actix_web::http::header;
use reqwest::Client;
use server::get_latest_release;
use tokio::io::BufReader;
use std::path::Path;
use tokio_util::io::ReaderStream;
use serde::Serialize;
use tokio::fs as async_fs;


#[derive(Serialize)]
struct V2Response {
    version: String,
    notes: String,
    pub_date: String,
    url: String,
    signature: String
}



const LATEST_VERSION: &str = "v1.0.0";


#[get("/v2/updates/{platform}/{version}")]
async fn tauri_update_v2(path: web::Path<(String, String)>) -> impl Responder {
    let (platform, version) = path.into_inner();
    println!("Request recived");
    println!("platform: {}, version: {}", platform, version);

    if version == LATEST_VERSION {
        return HttpResponse::NoContent().finish();
    }
   HttpResponse::Ok().json(V2Response{
        url: "https://my-tauri-app.onrender.com/download/test-tauri-build_aarch64.app.tar.gz".into(),
        //url: "http://127.0.0.1:8088/download/test-tauri-build_aarch64.app.tar.gz".into(), 
        version: "v1.0.0".into(),
        notes: "Theses are some release notes".into(),
        pub_date: "2020-09-18T12:29:53+01:00".into(),
        signature : "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVSam1rOWg0YjhHaWtKQ1d3ajgxazBwV0lQbXJ3RzZCWkpLeU1jQjJhTUhiQWFQcnVNVGpqNUhQRVBKcHBXdjk1UlFSVXdGZEIxM2JMd2FvR3ZSbzI5WENZMjZYaWE1RkFrPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzUwNDIwNzkyCWZpbGU6dGVzdC10YXVyaS1idWlsZC5hcHAudGFyLmd6Ck50VjdwUFZBZkhZL0JCRDQ4RUk5bzRMZDFXbmNNYUpZU1BUMWxWZVF4ZE9MNGdVM3ZtdFgyRDZ2WTk1ZlNWSXJiUUVMUTluQ2svZG1paEJxTE80Z0FBPT0K".into()    
    })
}



#[get("/v1/updates/{platform}/{version}")]
async fn tauri_update_v1(path: web::Path<(String, String)>) -> impl Responder {
    let (platform, version) = path.into_inner();
    println!("Request recived");
    println!("platform: {}, version: {}", platform, version);
    if version == LATEST_VERSION {
        return HttpResponse::NoContent().finish();
    }
    let client = Client::builder().user_agent("reqwest").build().expect("reqwest client could not be built");
    let repo = "BukiOffor/my-tauri-app";
    let value = get_latest_release(&client, repo).await.expect("msg");
    println!("Tauri Response: {}", value);
    HttpResponse::Ok().json(value)
}

#[get("/download/{filename}")]
async fn stream_zip_file(path: web::Path<String>) -> impl Responder {
    let filename = path.into_inner();
    let path = Path::new(&filename);
    if async_fs::metadata(&path).await.is_err() {
        return HttpResponse::NotFound().body("Zip file not found");
    }
    let file = match async_fs::File::open(&path).await {
        Ok(f) => f,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to open file: {e}")),
    };
    let filename = path.file_name().unwrap().to_string_lossy();
    let stream = ReaderStream::new(BufReader::new(file));
    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "application/zip"))
        .insert_header((
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{filename}\""),
        ))
        .streaming(stream)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Tauri Update server running on http://127.0.0.1:8088");
    HttpServer::new(|| {
        App::new()
            .service(tauri_update_v1)
            .service(tauri_update_v2)
            .service(stream_zip_file)
    })
    .bind(("0.0.0.0", 8088))?
    .run()
    .await
}
