extern crate alloc;
extern crate reqwest;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::string::String;

use skia_safe::{EncodedImageFormat, Image};

use crate::core::builder::petpet_builder::PetpetBuilder;
use crate::core::http::avatar_data_factory::{AvatarDataURL, create_avatar_data};
use crate::core::template::petpet_template::PetpetTemplate;

mod core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let path = "./data1";

    let data = read_file_to_string(&(String::from(path) + "/data.json"));

    let template: PetpetTemplate = serde_json::from_str(&data).unwrap();
    println!("{:#?}", template);

    let builder = PetpetBuilder::new(template, path)?;

    let urls = AvatarDataURL {
        from: None,
        to: Option::from("https://avatars.githubusercontent.com/u/68615161?s=640"),
        bot: None,
        group: None,
        random: vec![],
    };

    let avatar_data = create_avatar_data(&urls).unwrap();
    let f = builder.build(avatar_data).await.unwrap();
    let img = &f[0];
    save_image_to_file(img, "output.png");
    Ok(())
}

fn save_image_to_file(image: &Image, filename: &str) {
    let data = image.encode_to_data(EncodedImageFormat::PNG).unwrap();
    let mut file = std::fs::File::create(filename).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

fn read_file_to_string(file_path: &str) -> String {
    println!("{}", file_path);
    let mut file = File::open(file_path).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content
}