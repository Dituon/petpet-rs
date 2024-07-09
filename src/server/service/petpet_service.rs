use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

use crate::core::builder::petpet_builder::PetpetBuilder;
use crate::core::encoder::encoder::IMAGE_ENCODER;
use crate::core::errors::Error;
use crate::core::errors::Error::{FileError, TemplateError};
use crate::core::http::avatar_data_factory::create_avatar_data;
use crate::core::http::template_data::AvatarDataURL;
use crate::core::template::petpet_template::PetpetTemplate;
use crate::core::template::text_template::TextData;

pub struct PetpetService {
    builder_map: HashMap<String, PetpetBuilder>,
}

impl PetpetService {
    pub fn new() -> PetpetService {
        PetpetService {
            builder_map: HashMap::with_capacity(32)
        }
    }

    pub fn get_builder(&self, key: &str) -> Option<&PetpetBuilder> {
        self.builder_map.get(key)
    }

    pub fn join_path<'a>(&'a mut self, path: &'a str) -> Result<&Self, Error> {
        if let Ok(paths) = std::fs::read_dir(path) {
            for entry in paths {
                let path_buf = entry?.path();
                if !&path_buf.is_dir() {
                    continue
                }
                let root_path_str = (&path_buf).to_str().ok_or(
                    FileError(format!("Can not get path name: {:?}", path_buf))
                )?.to_string();
                let data_path_str = format!("{}/data.json", root_path_str);
                let template: PetpetTemplate = if Path::new(&data_path_str).exists() {
                    let str = std::fs::read_to_string(data_path_str)?;
                    let jd = &mut serde_json::Deserializer::from_str(&str);
                    serde_path_to_error::deserialize(jd).map_err(|err|
                        TemplateError(format!(
                            "Can not decode {} in {}/data.json: {}",
                            err.path(), root_path_str, err.inner().to_string()
                        ))
                    )?
                } else {
                    continue
                };
                self.builder_map.insert(
                    (&path_buf).file_name().ok_or(
                        FileError(format!("Can not get path name: {:?}", &path_buf))
                    )?.to_str().ok_or(
                        FileError(format!("Can not filename convert to str: {:?}", &path_buf))
                    )?.to_string(),
                    PetpetBuilder::new(template, root_path_str)?
                );
            }
            Ok(self)
        } else {
            Err(FileError(format!("Can not read file: {}", path)))
        }
    }

    pub fn with_paths<'a>(paths: &Vec<String>) -> Result<Self, Error> {
        let mut s = PetpetService::new();
        for path in paths {
            s.join_path(&path)?;
        }
        Ok(s)
    }

    pub async fn generate_all(&self) {
        let avatar = Some("https://avatars.githubusercontent.com/u/68615161?v=4".to_string());
        for (k, v) in &self.builder_map {
            println!("{}", k);
            let b = &AvatarDataURL {
                from: avatar.clone(),
                to: avatar.clone(),
                bot: avatar.clone(),
                group: avatar.clone(),
                random: None,
            };
            let t = TextData::default();
            let data = create_avatar_data(b).unwrap();
            let start_time0 = Instant::now();
            let (images, delay) = v.build(data, t).await.unwrap();
            println!("download & draw: {:?}", start_time0.elapsed());
            let start_time1 = Instant::now();
            let (blob, format) = IMAGE_ENCODER.encode(&images, delay).unwrap();
            println!("encode: {:?}", start_time1.elapsed());
            let mut file = File::create(format!("./output/{}.{}", k, format.get_str())).unwrap();
            file.write_all(&blob).expect("TODO: panic message");
        }
    }
}