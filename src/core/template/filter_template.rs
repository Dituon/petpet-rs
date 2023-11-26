use core::fmt::{Formatter};

use paste::paste;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, value, Visitor};

macro_rules! define_default {
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
            $(
                fn [<$type:snake _default_ $field>]() -> Vec<f32> {
                    vec![$value]
                }
            )*
        }
    };
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

define_default!(AvatarSwirlFilter {
    radius: 0.0,
    angle: 3.0,
    x: 0.5,
    y: 0.5
});
define_default!(AvatarBulgeFilter { 
    radius: 0.0,
    strength: 0.5, 
    x: 0.5,
    y: 0.5 
});
define_default!(AvatarSwimFilter { 
    scale: 32.0,
    stretch: 1.0,
    angle: 0.0,
    amount: 10.0,
    turbulence: 1.0,
    time: 0.0 
});
define_default!(AvatarBlurFilter {
    radius: 10.0
});
define_default!(AvatarContrastFilter {
    brightness: 0.0,
    contrast: 0.0
});
define_default!(AvatarHSBFilter {
    hue: 0.0,
    saturation: 0.0,
    brightness: 0.0
});
define_default!(AvatarHalftoneFilter {
    angle: 0.0,
    radius: 4.0,
    x: 0.5,
    y: 0.5
});
define_default!(AvatarDotScreenFilter {
    angle: 0.0,
    radius: 4.0,
    x: 0.5,
    y: 0.5
});
define_default!(AvatarNoiseFilter {
    amount: 0.25
});
define_default!(AvatarDenoiseFilter {
    exponent: 20.0
});
define_default!(AvatarOilFilter {
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
