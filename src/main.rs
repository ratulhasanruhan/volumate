mod volume;

use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use actix_cors::Cors;
use actix_files::Files;
use serde::{Deserialize, Serialize};
use local_ip_address::local_ip;
use colored::*;

#[derive(Serialize, Deserialize)]
struct VolumeRequest {
    level: f32,
}

#[derive(Serialize, Deserialize)]
struct MuteRequest {
    muted: bool,
}

#[derive(Serialize)]
struct VolumeResponse {
    success: bool,
    volume: Option<f32>,
    muted: Option<bool>,
    message: Option<String>,
}

// API Handlers
async fn get_volume() -> HttpResponse {
    match volume::get_master_volume() {
        Ok(vol) => {
            match volume::get_mute() {
                Ok(muted) => HttpResponse::Ok().json(VolumeResponse {
                    success: true,
                    volume: Some(vol),
                    muted: Some(muted),
                    message: None,
                }),
                Err(_) => HttpResponse::Ok().json(VolumeResponse {
                    success: true,
                    volume: Some(vol),
                    muted: Some(false),
                    message: None,
                }),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(VolumeResponse {
            success: false,
            volume: None,
            muted: None,
            message: Some(format!("Error: {}", e)),
        }),
    }
}

async fn set_volume(req: web::Json<VolumeRequest>) -> HttpResponse {
    match volume::set_master_volume(req.level) {
        Ok(_) => HttpResponse::Ok().json(VolumeResponse {
            success: true,
            volume: Some(req.level),
            muted: None,
            message: Some(format!("Volume set to {:.0}%", req.level * 100.0)),
        }),
        Err(e) => HttpResponse::InternalServerError().json(VolumeResponse {
            success: false,
            volume: None,
            muted: None,
            message: Some(format!("Error: {}", e)),
        }),
    }
}

async fn set_mute(req: web::Json<MuteRequest>) -> HttpResponse {
    match volume::set_mute(req.muted) {
        Ok(_) => HttpResponse::Ok().json(VolumeResponse {
            success: true,
            volume: None,
            muted: Some(req.muted),
            message: Some(format!("Mute set to {}", req.muted)),
        }),
        Err(e) => HttpResponse::InternalServerError().json(VolumeResponse {
            success: false,
            volume: None,
            muted: None,
            message: Some(format!("Error: {}", e)),
        }),
    }
}

fn print_banner(ip: &str) {
    println!("\n{}", "═══════════════════════════════════════════════════════════".bright_cyan().bold());
    println!("{}", "          🎚️  VOLUME CONTROLLER SERVER".bright_white().bold());
    println!("{}", "═══════════════════════════════════════════════════════════".bright_cyan().bold());
    println!();
    println!("  {}  {}", "Status:".bright_yellow().bold(), "Running ✓".bright_green().bold());
    println!();
    println!("  {} {}", "Local IP:".bright_yellow().bold(), ip.bright_white().bold());
    println!();
    println!("  {} {}", "Web Interface:".bright_yellow().bold(),
             format!("http://{}:8080", ip).bright_cyan().bold().underline());
    println!();
    println!("{}", "═══════════════════════════════════════════════════════════".bright_cyan().bold());
    println!();
    println!("  {}", "📱 Access from any device on your network:".bright_white());
    println!("     1. Connect to the same WiFi network");
    println!("     2. Open a browser");
    println!("     3. Enter: {}", format!("http://{}:8080", ip).bright_cyan().bold());
    println!();
    println!("  {}", "Press Ctrl+C to stop the server".bright_black());
    println!();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get local IP
    let local_ip_addr = local_ip().unwrap_or_else(|_| {
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))
    });

    let ip_str = local_ip_addr.to_string();

    // Clear screen (Windows command)
    print!("\x1B[2J\x1B[1;1H");

    // Print beautiful banner
    print_banner(&ip_str);

    // Start HTTP server
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .route("/api/volume", web::get().to(get_volume))
            .route("/api/volume", web::post().to(set_volume))
            .route("/api/mute", web::post().to(set_mute))
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
