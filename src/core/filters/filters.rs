use skia_safe::{Data, HighContrastConfig, Image, Paint, Point, SamplingOptions};
use skia_safe::runtime_effect::ChildPtr;

use crate::core::filters::binarize::binarize_shader;
use crate::core::filters::bulge::bulge_shader;
use crate::core::filters::color_halftone::color_halftone_shader;
use crate::core::filters::contrast::contrast_shader;
use crate::core::filters::denoise::denoise_shader;
use crate::core::filters::dot_screen::dot_screen_shader;
use crate::core::filters::gray::gray_shader;
use crate::core::filters::hsb::hsb_shader;
use crate::core::filters::noise::noise_shader;
use crate::core::filters::oil::oil_shader;
use crate::core::filters::pinch::pinch_shader;
use crate::core::filters::swim::swim_shader;
use crate::core::filters::swirl::swirl_shader;
use crate::core::template::filter_template::{AvatarFilter, UniformsBuilder};

pub fn build_style(image: &Image, filters: &Vec<AvatarFilter>, index: usize) -> Image {
    let mut surface = skia_safe::surfaces::raster_n32_premul(
        (image.width(), image.height())
    ).unwrap();
    let canvas = surface.canvas();
    let mut paint = Paint::default();

    for style in filters {
        let (eff, uniforms) = match style {
            AvatarFilter::SWIRL(t) => (
                swirl_shader(),
                Some(UniformsBuilder::from(t))
            ),
            AvatarFilter::BULGE(t) => (
                if t.strength[index % t.strength.len()] > 0.0 {
                    bulge_shader()
                } else {
                    pinch_shader()
                },
                Some(UniformsBuilder::from(t))
            ),
            AvatarFilter::SWIM(t) => (
                swim_shader(),
                Some(UniformsBuilder::from(t))
            ),
            AvatarFilter::BLUR(t) => {
                let radius = &t.radius[index % t.radius.len()];
                paint.set_image_filter(skia_safe::image_filters::blur(
                    (*radius, *radius),
                    None, None, None
                ));
                canvas.draw_image(image, Point::from((0.0, 0.0)), Some(&paint));
                return surface.image_snapshot()
            },
            AvatarFilter::CONTRAST(t) => (
                contrast_shader(),
                Some(UniformsBuilder::from(t))
            ),
            AvatarFilter::HSB(t) => (
                hsb_shader(),
                Some(UniformsBuilder::from(t))
            ),
            AvatarFilter::HALFTONE(t) => (
                color_halftone_shader(),
                Some(UniformsBuilder::from(t))
            ),
            AvatarFilter::DOTSCREEN(t) => (
                dot_screen_shader(),
                Some(UniformsBuilder::from(t))
            ),
            AvatarFilter::NOISE(t) => (
                noise_shader(),
                Some(UniformsBuilder::from(t))
            ),
            AvatarFilter::DENOISE(t) => (
                denoise_shader(),
                Some(UniformsBuilder::from(t))
            ),
            AvatarFilter::OIL(t) => (
                oil_shader(),
                Some(UniformsBuilder::from(t))
            ),
            AvatarFilter::GRAY => (gray_shader(), None),
            AvatarFilter::BINARIZE => (binarize_shader(), None),
            _ => panic!()
        };

        let shader = image.to_shader(
            None,
            SamplingOptions::default(),
            None,
        ).unwrap();
        let shaders = vec![ChildPtr::Shader(shader)];
        let m_shader = eff.make_shader(
            if uniforms.is_none() {
                Data::new_empty()
            } else {
                Data::new_copy(&uniforms.unwrap().build(&eff, image, index))
            },
            &shaders,
            None,
        ).unwrap();
        paint.set_shader(m_shader);
    }

    canvas.draw_paint(&paint);
    surface.image_snapshot()
}