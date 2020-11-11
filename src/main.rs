#![allow(non_snake_case)]
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use std::fs::File;
use std::io::BufReader;
use std::fs;
use std::io::Write;
use actix_multipart::Multipart;
use actix_files::Files;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use rustls::internal::pemfile::{certs, rsa_private_keys, pkcs8_private_keys};
use rustls::NoClientAuth;
use actix_tls::rustls::ServerConfig;
use futures::StreamExt;
use std::fs::OpenOptions;

/// static html/js pages
async fn index(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(&include_bytes!("../rcrdr.html")[..])
}
async fn RecordRTC_js(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().content_type("text/javascript").body(&include_bytes!("../RecordRTC.js")[..])
}
async fn style_css(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(&include_bytes!("../style.css")[..])
}
async fn getHTMLMediaElement_css(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(&include_bytes!("../getHTMLMediaElement.css")[..])
}
async fn getHTMLMediaElement_js(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(&include_bytes!("../getHTMLMediaElement.js")[..])
}
//File upload - multipart
async fn up(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(Ok(mut field)) = payload.next().await {
        if let Some(content_type) = field.content_disposition() {
            if let Some(name_param) = content_type.get_name() {
                if name_param == "video-blob" {
                    if let Some(filename) = content_type.get_filename() {
                        if !filename.contains(".."){
                            let filepath = format!("ups/{}", filename);
                            // Open output file to create or append
                            let mut f = web::block(||
                                OpenOptions::new().append(true).create(true).open(filepath)
                            )
                                .await
                                .unwrap();
                            // Field in turn is stream of *Bytes* object
                            while let Some(chunk) = field.next().await {
                                let data = chunk.unwrap();
                                // filesystem operations are blocking, we have to use threadpool
                                f = web::block(move || f.write_all(&data).map(|_| f)).await?;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(HttpResponse::Ok().into())
}

//List uploaded files
async fn ls(_: HttpRequest) -> HttpResponse {
    let mut rstr = String::new();
    if let Ok(rdir) = fs::read_dir("ups"){
        for ent in rdir{
            if let Ok(entry) = ent{
                if let Ok(md) = entry.metadata(){
                    if let Ok(time) = md.modified() {
                        let datetime: DateTime<Utc> = time.into();
                        let pth1 = entry.path();
                        let pth = pth1.to_string_lossy();
                        rstr.push_str(&format!("{} <a href=\"{}\">{}</a> <br>",
                            datetime.format("%d/%m/%Y %T"), pth, pth));
                    }
                }
            }
        }
    }
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rstr)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    fs::create_dir_all("ups")?; //make uploads folder if not already there
    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).or_else(|_|{
            let key_file2 = &mut BufReader::new(File::open("key.pem").unwrap());
            pkcs8_private_keys(key_file2)
        }).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
            .service(web::resource("/RecordRTC.js").to(RecordRTC_js))
            .service(web::resource("/style.css").to(style_css))
            .service(web::resource("/getHTMLMediaElement.js").to(getHTMLMediaElement_js))
            .service(web::resource("/getHTMLMediaElement.css").to(getHTMLMediaElement_css))
            .service(web::resource("/up").route(web::post().to(up)))
            .service(web::resource("/ls").to(ls))
            .service(Files::new("/ups", "ups"))
    })
    .bind_rustls("0.0.0.0:8443", config)?
    .run()
    .await
}
