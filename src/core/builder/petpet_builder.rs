use skia_safe::{Canvas, Image, SurfaceProps};

use crate::core::builder::avatar_builder::{AvatarBuilderList, AvatarData};
use crate::core::builder::background_builder::BackgroundBuilder;
use crate::core::errors::Error;
use crate::core::loader::image_loader::has_image;
use crate::core::template::petpet_template::PetpetTemplate;

pub struct PetpetBuilder {
    pub template: PetpetTemplate,
    avatar_builders: AvatarBuilderList,
    background_builder: BackgroundBuilder,
}

impl PetpetBuilder{
    pub fn new<'a>(template: PetpetTemplate, background_path: String) -> Result<PetpetBuilder, Error> {
        let avatar_builders = AvatarBuilderList::new(template.avatar.clone())?;

        let background_builder = BackgroundBuilder::new(
            template.background.clone(),
            if has_image(&background_path) { Some(background_path) } else { None }
        )?;

        Ok(PetpetBuilder {
            template,
            avatar_builders,
            background_builder,
        })
    }

    pub async fn build<'a>(&'a self, avatar_data: AvatarData<'a>) ->  Result<Vec<Image>, Error> {
        let mut avatar_size = Vec::with_capacity(self.template.avatar.len());
        let mut top_avatars = Vec::with_capacity(self.template.avatar.len());
        let mut bottom_avatars = Vec::with_capacity(self.template.avatar.len());

        let avatars = self.avatar_builders.build(avatar_data).await?;
        for avatar in &avatars {
            if avatar.template.avatar_on_top {
                top_avatars.push(avatar)
            } else {
                bottom_avatars.push(avatar)
            }
            avatar_size.push(avatar.get_size());
        }

        let (mut surface, bgs) = self.background_builder.create_background(avatar_size)?;
        let mut result = Vec::with_capacity(bgs.len());

        for (i, bg) in bgs.iter().enumerate() {
            let mut canvas = surface.canvas();
            // println!("canvas {:?}", canvas.image_info());
            for ba in &bottom_avatars {
                ba.draw(canvas, i)?;
            }
            canvas.draw_image(bg, (0, 0), None);
            for ta in &top_avatars {
                ta.draw(canvas, i)?;
            }
            result.push(surface.image_snapshot());
        }

        Ok(result)
    }

    // pub async fn build<'a>(&self, avatar_data: AvatarData<'a>) ->  Result<(), Error> {
    //
    //     let images = join_all(future_vec).await;
    //     Ok(())
    // }
}