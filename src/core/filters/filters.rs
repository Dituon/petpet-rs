use skia_safe::{Data, Image, Paint, SamplingOptions};
use skia_safe::runtime_effect::ChildPtr;

use crate::core::filters::binarize::binarize_shader;
use crate::core::filters::gray::gray_shader;
use crate::core::template::filter_template::AvatarFilter;

pub fn build_style(image: &Image, filters: &Vec<AvatarFilter>) -> Image {
    let shader = image.to_shader(
        None,
        SamplingOptions::default(),
        None,
    ).unwrap();

    let shaders = vec![ChildPtr::Shader(shader)];
    let mut paint = Paint::default();

    for style in filters {
        let eff = match style {
            // AvatarFilter::SWIRL(_) => {}
            // AvatarFilter::BULGE(_) => {}
            // AvatarFilter::SWIM(_) => {}
            // AvatarFilter::BLUR(_) => {}
            // AvatarFilter::CONTRAST(_) => {}
            // AvatarFilter::HSB(_) => {}
            // AvatarFilter::HALFTONE(_) => {}
            // AvatarFilter::DOT_SCREEN(_) => {}
            // AvatarFilter::NOISE(_) => {}
            // AvatarFilter::DENOISE(_) => {}
            // AvatarFilter::OIL(_) => {}
            AvatarFilter::GRAY => gray_shader(),
            AvatarFilter::BINARIZE => binarize_shader(),
            _ => panic!()
        };
        let m_shader = eff.make_shader(
            Data::new_empty(),
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