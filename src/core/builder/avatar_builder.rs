use alloc::rc::Rc;
use std::sync::{Arc, Mutex};

use futures::future::BoxFuture;
use skia_safe::Image;

use crate::core::builder::pos_builder::{compile_pos, CompiledPos};
use crate::core::errors::Error;
use crate::core::errors::Error::TemplateError;
use crate::core::model::avatar_model::AvatarModel;
use crate::core::template::avatar_template::{AvatarPosType, AvatarTemplate, AvatarType, PosDimension};

pub static FROM: usize = 0b00001;
pub static TO: usize = 0b00010;
pub static GROUP: usize = 0b00100;
pub static BOT: usize = 0b01000;
pub static RANDOM: usize = 0b10000;

pub fn by_type(t: &AvatarType) -> usize {
    match t {
        AvatarType::FROM => FROM,
        AvatarType::TO => TO,
        AvatarType::GROUP => GROUP,
        AvatarType::BOT => BOT,
        AvatarType::RANDOM => RANDOM
    }
}

pub struct AvatarBuilder {
    template: AvatarTemplate,
    pos: CompiledPos,
}

pub type AvatarDataItem<'a> = Option<Arc<Mutex<BoxFuture<'a, Result<Arc<Vec<Image>>, Error<'a>>>>>>;

pub struct AvatarData<'a> {
    pub from: AvatarDataItem<'a>,
    pub to: AvatarDataItem<'a>,
    pub bot: AvatarDataItem<'a>,
    pub group: AvatarDataItem<'a>,
    pub random: Vec<AvatarDataItem<'a>>,
}


impl AvatarBuilder {
    pub fn new<'a>(template: AvatarTemplate) -> Result<AvatarBuilder, Error<'a>> {
        let pos: PosDimension = match &template.pos_type {
            AvatarPosType::ZOOM => match &template.pos {
                PosDimension::P1D(pos) => PosDimension::P2D(vec![pos.clone()]),
                PosDimension::P2D(_) => template.pos.clone(),
                _ => Err(TemplateError(""))?
            },
            AvatarPosType::DEFORM => match &template.pos {
                PosDimension::P2D(pos) => PosDimension::P3D(vec![pos.clone()]),
                PosDimension::P3D(_) => template.pos.clone(),
                _ => Err(TemplateError(""))?
            }
        };

        let pos = compile_pos(pos)?;

        Ok(AvatarBuilder {
            template,
            pos
        })
    }
    pub async fn build<'a>(&'a self, data: Rc<AvatarData<'a>>) -> Result<AvatarModel<'a>, Error> {
        AvatarModel::new(&self.template, data, &self.pos).await
    }
}
