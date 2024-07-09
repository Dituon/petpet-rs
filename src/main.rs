extern crate alloc;
extern crate reqwest;

use std::fs::File;
use std::io::Read;
use std::io::Write;

use gif::Repeat;
use skia_safe::{EncodedImageFormat, Image};

#[cfg(feature = "server")]
use crate::server::{config::ServerConfig, server::PetpetServer};
// use crate::server::service::petpet_service::PetpetService;

mod core;
mod server;

#[tokio::main]
#[cfg(feature = "server")]
async fn main() {
    let config = ServerConfig::read_or_save("./config.json");
    // let service = PetpetService::with_paths(&config.data_path).unwrap();
    // &service.generate_all().await;
    // let f = FontMgr::new();
    // let _: Vec<String> = dbg!(f.family_names().collect());

    let server = PetpetServer::new(config).unwrap();
    server.run().await;
}

#[cfg(not(feature = "server"))]
fn main() {
}

pub fn save_image_to_file(image: &Image, filename: &str) {
    let data = image.encode_to_data(EncodedImageFormat::PNG).unwrap();
    let mut file = std::fs::File::create(filename).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

pub fn save_images_to_file(images: &Vec<Image>, filename: &str) {
    let mut image = File::create(filename).unwrap();
    let mut encoder = gif::Encoder::new(
        &mut image,
        images[0].width() as u16,
        images[0].height() as u16,
        &[]
    ).unwrap();
    let _ = encoder.set_repeat(Repeat::Infinite);
    for img in images {
        let mut ps = img.peek_pixels().unwrap().bytes().unwrap().to_owned();
        let mut frame = gif::Frame::from_rgba_speed(
            img.width() as u16,
            img.height() as u16,
            &mut ps,
            10
        );
        frame.delay = 65;
        let _ = encoder.write_frame(&frame);
    }
}