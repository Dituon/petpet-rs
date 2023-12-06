use once_cell::sync::Lazy;
use rayon::prelude::*;
use skia_safe::Image;

use crate::core::builder::avatar_builder::{AvatarBuilderList, AvatarData};
use crate::core::builder::background_builder::BackgroundBuilder;
use crate::core::builder::text_builder::TextBuilderList;
use crate::core::errors::Error;
use crate::core::loader::image_loader::has_image;
use crate::core::template::petpet_template::PetpetTemplate;
use crate::core::template::text_template::TextData;

pub static MULTITHREADED_DRAWING: Lazy<bool> = Lazy::new(|| true);

pub struct PetpetBuilder {
    pub template: PetpetTemplate,
    avatar_builders: AvatarBuilderList,
    text_builders: TextBuilderList,
    background_builder: BackgroundBuilder,
}

impl PetpetBuilder {
    pub fn new<'a>(template: PetpetTemplate, background_path: String) -> Result<PetpetBuilder, Error> {
        println!("{}", background_path);

        let background_builder = BackgroundBuilder::new(
            template.background.clone(),
            if has_image(&background_path) { Some(background_path) } else { None },
        )?;

        let avatar_builders = AvatarBuilderList::new(
            template.avatar.clone(),
            background_builder.length
        )?;

        let text_builders = TextBuilderList::new(
            template.text.clone()
        )?;

        Ok(PetpetBuilder {
            template,
            avatar_builders,
            text_builders,
            background_builder,
        })
    }

    pub async fn build<'a>(&'a self, avatar_data: AvatarData<'a>, text_data: TextData) -> Result<(Vec<Image>, u16), Error> {
        let a_count = self.template.avatar.len();
        let mut avatar_size = Vec::with_capacity(a_count);
        let mut top_avatars = Vec::with_capacity(a_count);
        let mut bottom_avatars = Vec::with_capacity(a_count);
        let mut avatar_max_length = 0;

        let avatars = self.avatar_builders.build(avatar_data).await?;
        let texts = self.text_builders.build(&text_data)?;

        for avatar in &avatars {
            if avatar.template.raw.avatar_on_top {
                top_avatars.push(avatar)
            } else {
                bottom_avatars.push(avatar)
            }
            avatar_size.push(avatar.get_size());
            avatar_max_length += avatar.get_length();
        }

        let (mut surface, bgs) = self.background_builder.create_background(
            avatar_size, texts.iter().map(|t| t.get_size()).collect()
        )?;
        let bgs = BackgroundBuilder::repeat_for_avatar_length(bgs, avatar_max_length);

        let t_delay = if self.background_builder.path.is_some() {
            self.template.delay / 10
        } else {
            let mut d = 0;
            for a in &avatars {
                d += a.delay;
            }
            d / a_count as u16 / 10
        };

        if MULTITHREADED_DRAWING.to_owned() {
            let info = surface.image_info();
            let arrs: Vec<Image> = bgs.par_iter().enumerate().map(|(i, bg)| {
                let mut temp_surface = skia_safe::surfaces::raster(
                    &info,
                    0,
                    None,
                ).unwrap();
                let canvas = temp_surface.canvas();
                for ba in &bottom_avatars {
                    ba.draw(canvas, i).unwrap();
                }
                canvas.draw_image(bg, (0, 0), None);
                for ta in &top_avatars {
                    ta.draw(canvas, i).unwrap();
                }
                for text in &texts {
                    text.draw(canvas);
                }
                temp_surface.image_snapshot()
            }).collect();
            Ok((arrs, t_delay))
        } else {
            let mut result = Vec::with_capacity(bgs.len());
            for (i, bg) in bgs.iter().enumerate() {
                let canvas = surface.canvas();
                for ba in &bottom_avatars {
                    ba.draw(canvas, i)?;
                }
                canvas.draw_image(bg, (0, 0), None);
                for ta in &top_avatars {
                    ta.draw(canvas, i)?;
                }
                for text in &texts {
                    text.draw(canvas);
                }
                result.push(surface.image_snapshot());
            }
            Ok((result, t_delay))
        }
    }

    // pub async fn build<'a>(&self, avatar_data: AvatarData<'a>) ->  Result<(), Error> {
    //
    //     let images = join_all(future_vec).await;
    //     Ok(())
    // }
}