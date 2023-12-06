use std::str::FromStr;

use meval::Expr;
use skia_safe::Point;

use crate::core::builder::background_builder::OriginSize;
use crate::core::errors::Error;
use crate::core::errors::Error::TemplateError;
use crate::core::template::avatar_template::{PosDimension, PosItem};

pub type XYWH = (i32, i32, i32, i32);
pub type P = (i32, i32);
pub type RO = (P, P, P, P, P);

#[derive(Debug, Clone)]
pub enum CompiledNumberPosDimension {
    P2D(Vec<XYWH>),
    P3D(Vec<[Point; 4]>),
}

pub type Expr3DIndex = (usize, usize, usize);

pub type CompiledExpr = (Expr, Expr3DIndex);

pub type CompiledExprVec = Vec<CompiledExpr>;

pub type CompiledPos = (CompiledNumberPosDimension, CompiledExprVec);

pub fn compile_pos<'a>(origin_pos: PosDimension) -> Result<CompiledPos, Error> {
    let mut expr_pos: CompiledExprVec = Vec::new();

    let number_pos: CompiledNumberPosDimension = match origin_pos {
        PosDimension::P1D(_) => return Err(TemplateError("".to_string()))?,
        PosDimension::P2D(p2d) => {
            let mut result: Vec<XYWH> = Vec::with_capacity(p2d.len());
            for (x_index, xywh) in p2d.iter().enumerate() {
                if xywh.len() != 4 {
                    return Err(TemplateError(
                        format!("xywh pos length must == 4 ({:?})", xywh)
                    ))?;
                }
                let pair: Vec<i32> = xywh.iter().enumerate().map(|(y_index, p)|
                    compile_pos_item(p, &mut expr_pos, (x_index, y_index, 0))
                ).collect::<Vec<i32>>();
                result.push((pair[0], pair[1], pair[2], pair[3]))
            }
            CompiledNumberPosDimension::P2D(result)
        }
        PosDimension::P3D(p3d) => {
            let mut result = Vec::with_capacity(p3d.len());
            for (x_index, ro) in p3d.iter().enumerate() {
                if ro.len() != 5 {
                    Err(TemplateError(
                        format!("deform pos length must == 5 ({:?})", ro)
                    ))?
                }
                let mut pair: [(i32, i32); 5] = [(0, 0); 5];
                let mut ri = 0;
                for (y_index, p) in ro.iter().enumerate() {
                    if p.len() != 2 {
                        return Err(TemplateError(
                            format!("deform pos point length must == 2 ({:?})", p)
                        ));
                    }
                    let x = compile_pos_item(
                        &p[0],
                        &mut expr_pos,
                        (x_index, y_index, 0),
                    );
                    let y = compile_pos_item(
                        &p[1],
                        &mut expr_pos,
                        (x_index, y_index, 1),
                    );
                    pair[ri] = (x, y);
                    ri += 1;
                }

                let (
                    (x1, y1),
                    (x2, y2),
                    (x3, y3),
                    (x4, y4),
                ) = relative_to_absolute(
                    (pair[0], pair[1], pair[2], pair[3], pair[4])
                );

                result.push([
                    Point::new(x1 as f32, y1 as f32),
                    Point::new(x2 as f32, y2 as f32),
                    Point::new(x3 as f32, y3 as f32),
                    Point::new(x4 as f32, y4 as f32),
                ])
            }
            CompiledNumberPosDimension::P3D(result)
        }
    };
    Ok((number_pos, expr_pos))
}

pub fn compile_pos_item(
    pos_item: &PosItem,
    expr_vec: &mut CompiledExprVec,
    index3d: Expr3DIndex,
) -> i32 {
    match pos_item {
        PosItem::Num(p_num) => p_num.clone(),
        PosItem::Expr(p_str) => {
            expr_vec.push((
                Expr::from_str(&p_str).unwrap(),
                index3d
            ));
            i32::MIN
        }
    }
}

pub fn eval_size<'a>(
    (num_pos, expr_vec): (&CompiledNumberPosDimension, &CompiledExprVec),
    (width, height): OriginSize,
) -> Result<CompiledNumberPosDimension, Error> {
    let mut ctx = meval::Context::new();
    ctx.var("width", width as f64).var("height", height as f64);

    let num_pos: CompiledNumberPosDimension = num_pos.clone();
    Ok(match num_pos {
        CompiledNumberPosDimension::P2D(mut p2d) => {
            for (expr, (x, y, _)) in expr_vec {
                let num = expr.eval_with_context(&ctx)?.round() as i32;
                match y {
                    0 => p2d[*x].0 = num,
                    1 => p2d[*x].1 = num,
                    2 => p2d[*x].2 = num,
                    3 => p2d[*x].3 = num,
                    _ => return Err(TemplateError("Unknown zoom avatar error".to_string()))
                };
            }
            CompiledNumberPosDimension::P2D(p2d)
        }
        CompiledNumberPosDimension::P3D(p3d) => {
            // for (expr, (x, y, z)) in expr_vec {
            //     let num = expr.eval_with_context(&ctx)?.round() as i32;
            //     let p = match y {
            //         0 => p3d[*x].0,
            //         1 => p3d[*x].1,
            //         2 => p3d[*x].2,
            //         3 => p3d[*x].3,
            //         4 => p3d[*x].4,
            //         _ => return Err(TemplateError("Unknown deform avatar y error"))
            //     };
            //     match z {
            //         0 => p.0,
            //         1 => p.1,
            //         _ => return Err(TemplateError("Unknown deform avatar z error"))
            //     };
            // }
            CompiledNumberPosDimension::P3D(p3d)
        }
    })
}


pub type CompiledSizeExpr = (Expr, usize);

pub type CompiledSize = (OriginSize, Vec<CompiledSizeExpr>);

pub fn compile_size(
    (width, height): &(PosItem, PosItem)
) -> CompiledSize {
    let mut expr_vec: Vec<CompiledSizeExpr> = Vec::new();
    let w = match width {
        PosItem::Num(p_num) => p_num.clone(),
        PosItem::Expr(p_str) => {
            expr_vec.push((
                Expr::from_str(&p_str).unwrap(),
                0
            ));
            i32::MIN
        }
    };
    let h = match height {
        PosItem::Num(p_num) => p_num.clone(),
        PosItem::Expr(p_str) => {
            expr_vec.push((
                Expr::from_str(&p_str).unwrap(),
                1
            ));
            i32::MIN
        }
    };
    ((w, h), expr_vec)
}

pub fn eval_background_size(
    (size, expr_vec): &CompiledSize,
    avatar_size: Vec<OriginSize>,
    text_size: Vec<OriginSize>,
) -> Result<OriginSize, Error> {
    let mut ctx = meval::Context::new();

    for (i, (w, h)) in avatar_size.iter().enumerate() {
        ctx.var(format!("avatar{}Width", i), *w as f64)
            .var(format!("avatar{}Height", i), *h as f64);
    }
    for (i, (w, h)) in text_size.iter().enumerate() {
        ctx.var(format!("text{}Width", i), *w as f64)
            .var(format!("text{}Height", i), *h as f64);
    }

    let mut result = size.clone();
    for (expr, index) in expr_vec {
        match index {
            0 => result.0 = expr.eval_with_context(&ctx)?.round() as i32,
            1 => result.1 = expr.eval_with_context(&ctx)?.round() as i32,
            _ => return Err(TemplateError("Unknown background size error".to_string()))
        }
    }
    Ok(result)
}

pub fn relative_to_absolute(relative_pos: RO) -> (P, P, P, P) {
    let (p0, p1, p2, p3, anchor) = relative_pos;

    let absolute_p0 = (anchor.0 + p0.0, anchor.1 + p0.1);
    let absolute_p1 = (anchor.0 + p1.0, anchor.1 + p1.1);
    let absolute_p2 = (anchor.0 + p2.0, anchor.1 + p2.1);
    let absolute_p3 = (anchor.0 + p3.0, anchor.1 + p3.1);

    (absolute_p0, absolute_p1, absolute_p2, absolute_p3)
}

