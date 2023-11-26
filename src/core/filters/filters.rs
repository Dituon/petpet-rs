use skia_safe::{Data, Image, Paint, SamplingOptions};
use skia_safe::runtime_effect::ChildPtr;

use crate::core::filters::binarize::binarize_shader;
use crate::core::filters::bulge::bulge_shader;
use crate::core::filters::gray::gray_shader;
use crate::core::filters::pinch::pinch_shader;
use crate::core::filters::swim::swim_shader;
use crate::core::filters::swirl::swirl_shader;
use crate::core::template::filter_template::{AvatarFilter, UniformsBuilder};

pub fn build_style(image: &Image, filters: &Vec<AvatarFilter>, index: usize) -> Image {
    let shader = image.to_shader(
        None,
        SamplingOptions::default(),
        None,
    ).unwrap();

    let shaders = vec![ChildPtr::Shader(shader)];
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
            // AvatarFilter::BLUR(_) => {}
            // AvatarFilter::CONTRAST(_) => {}
            // AvatarFilter::HSB(_) => {}
            // AvatarFilter::HALFTONE(_) => {}
            // AvatarFilter::DOT_SCREEN(_) => {}
            // AvatarFilter::NOISE(_) => {}
            // AvatarFilter::DENOISE(_) => {}
            // AvatarFilter::OIL(_) => {}
            AvatarFilter::GRAY => (gray_shader(), None),
            AvatarFilter::BINARIZE => (binarize_shader(), None),
            _ => panic!()
        };
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

    let mut surface = skia_safe::surfaces::raster_n32_premul(
        (image.width(), image.height())
    ).unwrap();
    let canvas = surface.canvas();
    canvas.draw_paint(&paint);
    surface.image_snapshot()
}