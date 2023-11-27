use core::fmt::Formatter;
use std::collections::HashMap;

use paste::paste;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, value, Visitor};
use skia_safe::{Image, RuntimeEffect};

macro_rules! define_filter {
    ($type:ident { $($field:ident: $value:expr),* $(,)? }) => {
        paste!{
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct $type {
                $(
                    #[serde(
                        default = "" [< $type:snake _default_ $field >],
                        deserialize_with = "f32_or_vec"
                    )]
                    pub $field: Vec<f32>,
                )*
            }

            impl AvatarFilterLike for $type {
                fn max_length(&self) -> usize {
                    let numbers = vec![$(
                        self.$field.len(),
                    )*];
                    numbers.iter().copied().fold(0, |max, current| max.max(current))
                }
            }

            impl<'a> From<&'a $type> for UniformsBuilder<'a> {
                fn from(filter: &'a $type) -> Self {
                    UniformsBuilder {
                        uniforms: HashMap::from([
                            $(
                                (stringify!($field), &filter.$field),
                            )*
                        ])
                    }
                }
            }

            $(
                fn [<$type:snake _default_ $field>]() -> Vec<f32> {
                    vec![$value]
                }
            )*
        }
    };
}

trait AvatarFilterLike {
    fn max_length(&self) -> usize;
}

#[derive(Default)]
pub struct UniformsBuilder<'a> {
    uniforms: HashMap<&'a str, &'a Vec<f32>>,
}

impl UniformsBuilder<'_> {
    pub fn build(&self, shader: &RuntimeEffect, image: &Image, index: usize) -> Vec<u8> {
        let mut values = Vec::new();

        for uniform in shader.uniforms().iter() {
            let k = uniform.name();
            let vec = self.uniforms.get(k).unwrap();
            let mut value = vec[index % vec.len()];
            match k {
                "x" => value *= image.width() as f32,
                "y" => value *= image.height() as f32,
                "radius" => if value == 0.0 {
                    value = i32::min(image.width(), image.height()) as f32 / 2.0
                },
                _ => {}
            }
            println!("{}: {}", k, value);
            values.extend(value.to_le_bytes());
        }

        values
    }
}

fn f32_or_vec<'de, D>(deserializer: D) -> Result<Vec<f32>, D::Error>
    where D: Deserializer<'de>
{
    struct F32OrVec;

    impl<'de> Visitor<'de> for F32OrVec {
        type Value = Vec<f32>;

        fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
            formatter.write_str("f32 or list of f32s")
        }

        fn visit_i64<E>(self, i: i64) -> Result<Self::Value, E> where E: de::Error {
            Ok(vec![i as f32])
        }

        fn visit_u64<E>(self, u: u64) -> Result<Self::Value, E> where E: de::Error {
            Ok(vec![u as f32])
        }

        fn visit_f64<E>(self, f: f64) -> Result<Self::Value, E> where E: de::Error {
            Ok(vec![f as f32])
        }

        fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
            where S: SeqAccess<'de> {
            Deserialize::deserialize(value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(F32OrVec)
}

define_filter!(AvatarSwirlFilter {
    radius: 0.0,
    angle: 3.0,
    x: 0.5,
    y: 0.5
});
define_filter!(AvatarBulgeFilter {
    radius: 0.0,
    strength: 0.5, 
    x: 0.5,
    y: 0.5 
});
define_filter!(AvatarSwimFilter {
    scale: 32.0,
    stretch: 1.0,
    angle: 0.0,
    amount: 10.0,
    turbulence: 1.0,
    time: 0.0 
});
define_filter!(AvatarBlurFilter {
    radius: 10.0
});
define_filter!(AvatarContrastFilter {
    brightness: 0.0,
    contrast: 0.0
});
define_filter!(AvatarHSBFilter {
    hue: 0.0,
    saturation: 0.0,
    brightness: 0.0
});
define_filter!(AvatarHalftoneFilter {
    angle: 0.0,
    radius: 4.0,
    x: 0.5,
    y: 0.5
});
define_filter!(AvatarDotScreenFilter {
    angle: 0.0,
    radius: 4.0,
    x: 0.5,
    y: 0.5
});
define_filter!(AvatarNoiseFilter {
    amount: 0.25
});
define_filter!(AvatarDenoiseFilter {
    exponent: 20.0
});
define_filter!(AvatarOilFilter {
    skip: 4.0,
    levels: 8.0,
    range: 12.0
});


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AvatarFilter {
    #[serde(rename = "SWIRL")]
    SWIRL(AvatarSwirlFilter),
    #[serde(rename = "BULGE")]
    BULGE(AvatarBulgeFilter),
    #[serde(rename = "SWIM")]
    SWIM(AvatarSwimFilter),
    #[serde(rename = "BLUR")]
    BLUR(AvatarBlurFilter),
    #[serde(rename = "CONTRAST")]
    CONTRAST(AvatarContrastFilter),
    #[serde(rename = "HSB")]
    HSB(AvatarHSBFilter),
    #[serde(rename = "HALFTONE")]
    HALFTONE(AvatarHalftoneFilter),
    #[serde(rename = "DOT_SCREEN")]
    DOTSCREEN(AvatarDotScreenFilter),
    #[serde(rename = "NOISE")]
    NOISE(AvatarNoiseFilter),
    #[serde(rename = "DENOISE")]
    DENOISE(AvatarDenoiseFilter),
    #[serde(rename = "OIL")]
    OIL(AvatarOilFilter),
    GRAY,
    BINARIZE,
}


impl AvatarFilter {
    pub fn max_length(&self) -> usize {
        match self {
            AvatarFilter::GRAY => 1,
            AvatarFilter::BINARIZE => 1,
            AvatarFilter::SWIRL(t) => t.max_length(),
            AvatarFilter::BULGE(t) => t.max_length(),
            AvatarFilter::SWIM(t) => t.max_length(),
            AvatarFilter::BLUR(t) => t.max_length(),
            AvatarFilter::CONTRAST(t) => t.max_length(),
            AvatarFilter::HSB(t) => t.max_length(),
            AvatarFilter::HALFTONE(t) => t.max_length(),
            AvatarFilter::DOTSCREEN(t) => t.max_length(),
            AvatarFilter::NOISE(t) => t.max_length(),
            AvatarFilter::DENOISE(t) => t.max_length(),
            AvatarFilter::OIL(t) => t.max_length(),
        }
    }
}