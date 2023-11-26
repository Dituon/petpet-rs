use std::collections::HashMap;
use std::path::Path;

use crate::core::builder::petpet_builder::PetpetBuilder;
use crate::core::errors::Error;
use crate::core::errors::Error::{FileError, TemplateError};
use crate::core::template::petpet_template::PetpetTemplate;

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

    pub fn with_paths<'a>(paths: &Vec<&str>) -> Result<Self, Error> {
        let mut s = PetpetService::new();
        for &path in paths {
            s.join_path(path)?;
        }
        Ok(s)
    }
}