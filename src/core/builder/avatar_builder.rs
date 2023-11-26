use std::sync::Arc;

use futures::future::{BoxFuture, join_all};
use rand::Rng;
use skia_safe::{Image, Matrix};

use crate::core::builder::pos_builder::{compile_pos, CompiledPos};
use crate::core::errors::Error;
use crate::core::errors::Error::{MissingDataError, TemplateError};
use crate::core::model::avatar_model::AvatarModel;
use crate::core::template::avatar_template::{AvatarCropType, AvatarPosType, AvatarStyle, AvatarTemplate, AvatarType, CropPos, PosDimension};
use crate::core::template::filter_template::AvatarFilter;

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

pub struct AvatarBuiltTemplate {
    pub raw: AvatarTemplate,
    pub pos: CompiledPos,
    pub matrix: Matrix,
}

pub struct AvatarBuilder {
    built_template: AvatarBuiltTemplate,
}

pub type AvatarDataItem<'a> = BoxFuture<'a, Result<Arc<Vec<Image>>, Error>>;

pub struct AvatarData<'a> {
    pub from: Option<AvatarDataItem<'a>>,
    pub to: Option<AvatarDataItem<'a>>,
    pub bot: Option<AvatarDataItem<'a>>,
    pub group: Option<AvatarDataItem<'a>>,
    pub random: Vec<AvatarDataItem<'a>>,
}

impl AvatarBuilder {
    pub fn new<'a>(mut template: AvatarTemplate) -> Result<AvatarBuilder, Error> {
        let pos: PosDimension = match &template.pos_type {
            AvatarPosType::ZOOM => match &template.pos {
                PosDimension::P1D(pos) => PosDimension::P2D(vec![pos.clone()]),
                PosDimension::P2D(_) => template.pos.clone(),
                _ => Err(TemplateError(format!("{:?}", template)))?
            },
            AvatarPosType::DEFORM => match &template.pos {
                PosDimension::P2D(pos) => PosDimension::P3D(vec![pos.clone()]),
                PosDimension::P3D(_) => template.pos.clone(),
                _ => Err(TemplateError(format!("{:?}", template)))?
            }
        };

        let pos = compile_pos(pos)?;

        template.crop = match &template.crop_type {
            AvatarCropType::NONE => None,
            _ => {
                if let CropPos::WH(wh) = template.crop.as_ref().ok_or(TemplateError("Can not find crop pos".to_string()))? {
                    Some(CropPos::XYXY((0.0, 0.0, wh.0, wh.1)))
                } else { template.crop }
            }
        };

        for style in &template.style {
            match style {
                AvatarStyle::GRAY => {
                    template.filter.push(AvatarFilter::GRAY);
                },
                AvatarStyle::BINARIZATION => {
                    template.filter.push(AvatarFilter::BINARIZE);
                },
                _ => {}
            }
        }

        let matrix = Self::compile_matrix();

        Ok(AvatarBuilder {
            built_template: AvatarBuiltTemplate {
                raw: template,
                pos,
                matrix,
            },
        })
    }

    fn compile_matrix() -> Matrix {
        Matrix::default()
    }

    pub fn build(&self, images: Arc<Vec<Image>>) -> Result<AvatarModel, Error> {
        AvatarModel::new(&self.built_template, images)
    }
}

pub struct AvatarBuilderList {
    types: usize,
    pub builders: Vec<(usize, bool, AvatarBuilder)>,
}

impl AvatarBuilderList {
    pub fn new<'a>(templates: Vec<AvatarTemplate>) -> Result<AvatarBuilderList, Error> {
        let mut types = 0;
        let mut items = Vec::with_capacity(templates.len());
        for avatar in templates {
            types = types | by_type(&avatar._type);
            items.push((
                by_type(&avatar._type),
                avatar.avatar_on_top,
                AvatarBuilder::new(avatar.clone())?,
            ));
        };
        Ok(AvatarBuilderList {
            types,
            builders: items,
        })
    }

    pub async fn build<'a>(&'a self, data: AvatarData<'a>) -> Result<Vec<AvatarModel<'a>>, Error> {
        // if self.types & if data.from.is_none() { 0 } else { FROM } == 0
        //     && self.types & if data.to.is_none() { 0 } else { TO } == 0
        //     && self.types & if data.bot.is_none() { 0 } else { BOT } == 0
        //     && self.types & if data.group.is_none() { 0 } else { GROUP } == 0
        //     && self.types & if data.random.is_none() { 0 } else { RANDOM } == 0 {
        //     return Err(MissingDataError(""));
        // }
        let (futures, types) = build_future(self.types, data)?;
        let mut res = join_all(futures).await;
        let mut avatars = Vec::with_capacity(res.len());
        for i in 0..res.len() {
            let t = types[i];
            let imgs = &res.remove(i)?;
            for (_, _, builder) in &self.builders {
                if by_type(&builder.built_template.raw._type) != t {
                    continue;
                }
                avatars.push(builder.build(Arc::clone(imgs))?);
            }
        }

        Ok(avatars)
    }
}

fn build_future(types: usize, data: AvatarData)
                -> Result<(Vec<BoxFuture<Result<Arc<Vec<Image>>, Error>>>, Vec<usize>), Error> {
    let mut future_vec = Vec::with_capacity(5);
    let mut extra_vec: Vec<usize> = Vec::with_capacity(5);
    if types & FROM != 0 {
        future_vec.push(data.from.ok_or_else(|| MissingDataError("Missing FROM data".to_string()))?);
        extra_vec.push(FROM);
    } else if types & TO != 0 {
        future_vec.push(data.to.ok_or_else(|| MissingDataError("Missing TO data".to_string()))?);
        extra_vec.push(TO);
    } else if types & GROUP != 0 {
        future_vec.push(data.group.ok_or_else(|| MissingDataError("Missing GROUP data".to_string()))?);
        extra_vec.push(GROUP);
    } else if types & BOT != 0 {
        future_vec.push(data.bot.ok_or_else(|| MissingDataError("Missing BOT data".to_string()))?);
        extra_vec.push(BOT);
    } else if types & RANDOM != 0 {
        let mut vec = data.random;
        if vec.is_empty() {
            return Err(MissingDataError("Missing RANDOM data".to_string()));
        }
        let index = rand::thread_rng().gen_range(0..vec.len());
        future_vec.push(vec.remove(index));
        extra_vec.push(RANDOM);
    }
    Ok((future_vec, extra_vec))
}
