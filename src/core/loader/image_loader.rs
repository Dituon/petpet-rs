use std::path::Path;

use once_cell::sync::Lazy;
use schnellru::{ByLength, LruMap};
use skia_safe::{AlphaType, Codec, ColorType, Data, Image, ImageInfo};

use crate::core::errors::Error::{self, FileError, ImageDecodeError};

static MAX_CACHE_LENGTH: Lazy<u32> = Lazy::new(|| 32);

static mut IMAGE_CACHE: Lazy<LruMap<String, Vec<Image>, ByLength>> = Lazy::new(|| {
    LruMap::new(ByLength::new(*MAX_CACHE_LENGTH))
});

pub fn has_image(path: &str) -> bool {
    let image_path_str = format!("{}/0.png", path);
    let image_path = Path::new(&image_path_str);
    image_path.exists()
}

pub fn image_count(path: &str) -> usize {
    let dir = std::fs::read_dir(path);
    if dir.is_err() {
        return 0;
    }

    dir.unwrap().filter_map(|entry| {
        entry.ok().and_then(|entry| {
            let binding = entry.file_name();
            let file_name = binding.to_string_lossy();
            let file_number = file_name.trim_end_matches(".png").parse::<usize>();
            file_number.map(|number| (number, entry)).ok()
        })
    })
        .enumerate()
        .take_while(|(index, (number, _))| *index == *number)
        .count()
}

pub fn load_image(path: String) -> Result<Image, Error> {
    if let Ok(blob) = std::fs::read(&path) {
        let data = Data::new_copy(blob.as_ref());
        let mut codec = Codec::from_data(data).unwrap();
        let info = ImageInfo::new(
            codec.dimensions(),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );
        codec.get_image(info, None).map_err(|_| ImageDecodeError(path))
    } else {
        Err(FileError(path))
    }
}

fn load_background(path: &str) -> Result<Vec<Image>, Error> {
    let mut images: Vec<Image> = Vec::new();
    for i in 0.. {
        let image_path_str = format!("{}/{}.png", path, i);
        let image_path = Path::new(&image_path_str);

        if !image_path.exists() {
            break;
        }

        let image = load_image(image_path_str)?;
        images.push(image);
    }
    Ok(images)
}

pub fn load_cached_background(path: &str) -> Result<&Vec<Image>, Error> {
    unsafe {
        Ok(match IMAGE_CACHE.get(path) {
            None => {
                let img = load_background(path)?;
                IMAGE_CACHE.insert(path.parse().unwrap(), img);
                IMAGE_CACHE.get(path).unwrap()
            }
            Some(img) => img
        })
    }
}
