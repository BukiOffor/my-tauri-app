use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct UpdateResponse {
    version: String,
    notes: String,
    pub_date: String,
    platforms: Platforms,
}


#[derive(Serialize)]
struct V2Response {
    version: String,
    notes: String,
    pub_date: String,
    url: String,
}


#[derive(Serialize)]
struct Platforms {
    darwin: Option<PlatformMetadata>,
    windows: Option<PlatformMetadata>,
    linux: Option<PlatformMetadata>,
}

#[derive(Serialize)]
struct PlatformMetadata {
    signature: String,
    url: String,
}

const LATEST_VERSION: &str = "v1.2.0";

#[get("/v1/updates/{platform}/{version}")]
async fn tauri_update(path: web::Path<(String, String)>) -> impl Responder {
    let (platform, version) = path.into_inner();

    if version == LATEST_VERSION {
        return HttpResponse::NoContent().finish();
    }

    let metadata = match platform.as_str() {
        "darwin" => Some(PlatformMetadata {
            signature: "FAKE_SIGNATURE_DARWIN".to_string(),
            url: "https://example.com/releases/app-macos.dmg".to_string(),
        }),
        "windows" => Some(PlatformMetadata {
            signature: "FAKE_SIGNATURE_WINDOWS".to_string(),
            url: "https://example.com/releases/app-windows.exe".to_string(),
        }),
        "linux" => Some(PlatformMetadata {
            signature: "FAKE_SIGNATURE_LINUX".to_string(),
            url: "https://example.com/releases/app-linux.tar.gz".to_string(),
        }),
        _ => None,
    };

    if metadata.is_none() {
        return HttpResponse::NotFound().body("Unsupported platform");
    }

    let mut platforms = Platforms {
        darwin: None,
        windows: None,
        linux: None,
    };

    match platform.as_str() {
        "darwin" => platforms.darwin = metadata,
        "windows" => platforms.windows = metadata,
        "linux" => platforms.linux = metadata,
        _ => {}
    }

    let update_info = UpdateResponse {
        version: LATEST_VERSION.to_string(),
        notes: "Bug fixes and performance improvements.".to_string(),
        pub_date: "2025-06-19T12:00:00Z".to_string(),
        platforms,
    };

    HttpResponse::Ok().json(update_info)
}


#[get("/v2/updates/{platform}/{version}")]
async fn tauri_update_handler(path: web::Path<(String, String)>) -> impl Responder {
   HttpResponse::Ok().json(V2Response{
        url: "https://mycompany.example.com/myapp/releases/myrelease.tar.gz".into(),
        version: "v1.0.1".into(),
        notes: "Theses are some release notes".into(),
        pub_date: "2020-09-18T12:29:53+01:00".into(),
        
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Tauri Update server running on http://127.0.0.1:8000");
    HttpServer::new(|| {
        App::new()
            .service(tauri_update)
    })
    .bind(("0.0.0.0", 8088))?
    .run()
    .await
}
