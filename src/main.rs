extern crate alloc;
extern crate reqwest;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::string::String;

use gif::Repeat;
use skia_safe::{EncodedImageFormat, Image};
use crate::server::server::PetpetServer;

mod core;
mod server;

#[tokio::main]
async fn main() {
    let server = PetpetServer::new().unwrap();
    server.run().await;
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
    encoder.set_repeat(Repeat::Infinite);
    for img in images {
        let mut ps = img.peek_pixels().unwrap().bytes().unwrap().to_owned();
        let mut frame = gif::Frame::from_rgba_speed(
            img.width() as u16,
            img.height() as u16,
            &mut ps,
            10
        );
        frame.delay = 65;
        encoder.write_frame(&frame);
    }
}

fn read_file_to_string(file_path: &str) -> String {
    println!("{}", file_path);
    let mut file = File::open(file_path).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content
}