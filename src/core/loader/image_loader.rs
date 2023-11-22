use std::path::Path;

use once_cell::sync::Lazy;
use schnellru::{ByMemoryUsage, LruMap};
use skia_safe::{AlphaType, Codec, ColorType, Data, Image, ImageInfo};

use crate::core::errors::Error::{self, FileError, ImageDecodeError};

static MAX_MEMORY: Lazy<usize> = Lazy::new(|| 32_000_000);

static mut IMAGE_CACHE: Lazy<LruMap<String, Vec<Image>, ByMemoryUsage>> = Lazy::new(|| {
    LruMap::with_memory_budget(*MAX_MEMORY)
});

pub fn has_image(path: &str) -> bool {
    let image_path_str = format!("{}/0.png", path);
    let image_path = Path::new(&image_path_str);
    image_path.exists()
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
