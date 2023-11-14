extern crate alloc;
extern crate reqwest;

use std::error::Error;
use std::io::Write;

use skia_safe::{EncodedImageFormat, Image};

use crate::core::builder::petpet_builder::PetpetBuilder;
use crate::core::http::avatar_data_factory::{AvatarDataURL, create_avatar_data};
use crate::core::template::petpet_template::PetpetTemplate;

mod core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = r#"
    {
    "type": "IMG",
    "avatar": [{
      "type": "TO",
      "posType": "DEFORM",
      "pos": [
        [3, 79], [0, 533], [336, 938], [330, 1], [272, -84]
      ],
      "avatarOnTop": false
    }]
    }"#;

    let template: PetpetTemplate = match serde_json::from_str(data){
        Ok(t) => t,
        Err(e) => panic!("{}", e.to_string())
    };
    println!("{:#?}", template);

    let builder = PetpetBuilder::new(template, "./data")?;

    let urls = AvatarDataURL {
        from: None,
        to: Option::from("https://avatars.githubusercontent.com/u/68615161?s=200"),
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