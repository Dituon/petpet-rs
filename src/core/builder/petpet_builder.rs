use alloc::rc::Rc;

use skia_safe::Image;

use crate::core::builder::avatar_builder::{AvatarBuilder, AvatarData, by_type};
use crate::core::builder::background_builder::BackgroundBuilder;
use crate::core::errors::Error;
use crate::core::loader::image_loader::has_image;
use crate::core::template::avatar_template::AvatarType;
use crate::core::template::petpet_template::PetpetTemplate;

pub struct PetpetBuilder<'a> {
    pub template: PetpetTemplate,
    pub types: usize,
    top_avatar_builders: Vec<AvatarBuilder>,
    bottom_avatar_builders: Vec<AvatarBuilder>,
    background_builder: BackgroundBuilder<'a>,
}

impl PetpetBuilder<'_>{
    pub fn new<'a>(template: PetpetTemplate, background_path: &'a str) -> Result<PetpetBuilder<'a>, Error> {
        let mut types = 0;
        let mut top_avatar_builders: Vec<AvatarBuilder> = Vec::with_capacity(template.avatar.len());
        let mut bottom_avatar_builders: Vec<AvatarBuilder> = Vec::with_capacity(template.avatar.len());
        for avatar in &template.avatar {
            types = types | by_type(&avatar._type);
            if avatar.avatar_on_top {
                top_avatar_builders.push(AvatarBuilder::new(avatar.clone())?)
            } else {
                bottom_avatar_builders.push(AvatarBuilder::new(avatar.clone())?)
            }
        };

        let background_builder = BackgroundBuilder::new(
            template.background.clone(),
            if has_image(background_path) { Some(background_path) } else { None }
        )?;

        Ok(PetpetBuilder {
            template,
            types,
            top_avatar_builders,
            bottom_avatar_builders,
            background_builder,
        })
    }

    pub async fn build<'a>(&'a self, avatar_data: AvatarData<'a>) ->  Result<Vec<Image>, Error> {
        let rc_data = Rc::new(avatar_data);

        let mut avatar_size = Vec::with_capacity(self.template.avatar.len());
        let mut top_avatar = Vec::with_capacity(self.top_avatar_builders.len());
        let mut bottom_avatar = Vec::with_capacity(self.bottom_avatar_builders.len());
        for top_avatar_builder in &self.top_avatar_builders {
            let a = top_avatar_builder.build(Rc::clone(&rc_data)).await?;
            avatar_size.push(a.get_size());
            top_avatar.push(a);
        }
        for bottom_avatar_builder in &self.bottom_avatar_builders {
            let a = bottom_avatar_builder.build(Rc::clone(&rc_data)).await?;
            avatar_size.push(a.get_size());
            bottom_avatar.push(a);
        }

        let (mut surface, bgs) = self.background_builder.create_background(avatar_size)?;
        let mut result = Vec::with_capacity(bgs.len());

        for bg in bgs {
            let canvas = surface.canvas();
            for (i, ba) in bottom_avatar.iter().enumerate() {
                ba.draw(canvas, i)?;
            }
            canvas.draw_image(bg, (0, 0), None);
            for (i, ta) in top_avatar.iter().enumerate() {
                ta.draw(canvas, i)?;
            }
            result.push(surface.image_snapshot());
        }

        Ok(result)
    }

    // pub async fn build<'a>(&self, avatar_data: AvatarData<'a>) ->  Result<(), Error> {
    //     let mut future_vec = Vec::with_capacity(5);
    //     if self.types & FROM != 0 {
    //         if let Some(imf) = avatar_data.from {
    //             let f = Box::pin(async move {
    //                 let img = imf.await?;
    //             });
    //             future_vec.insert(0, f)
    //         } else {
    //             return Err(MissingDataError("Missing FROM data"))
    //         }
    //     } else if self.types & TO != 0 {
    //         if let Some(f) = avatar_data.to {
    //             future_vec.insert(1, f)
    //         } else {
    //             return Err(MissingDataError("Missing TO data"))
    //         }
    //     } else if self.types & GROUP != 0 {
    //         if let Some(f) = avatar_data.group {
    //             future_vec.insert(2, f)
    //         } else {
    //             return Err(MissingDataError("Missing GROUP data"))
    //         }
    //     } else if self.types & BOT != 0 {
    //         if let Some(f) = avatar_data.bot {
    //             future_vec.insert(3, f)
    //         } else {
    //             return Err(MissingDataError("Missing BOT data"))
    //         }
    //     } else if self.types & RANDOM != 0 {
    //         let mut vec = avatar_data.random;
    //         let index = rand::thread_rng().gen_range(0..vec.len());
    //         if let Some(f) = vec.remove(index) {
    //             future_vec.insert(4, f)
    //         } else {
    //             return Err(MissingDataError("Missing RANDOM data"))
    //         }
    //     }
    //     let images = join_all(future_vec).await;
    //     Ok(())
    // }
}