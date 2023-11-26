use skia_safe::RuntimeEffect;

pub fn color_halftone_shader() -> RuntimeEffect {
    RuntimeEffect::make_for_shader(
        include_str!("./color_halftone.glsl"),
        None,
    ).unwrap()
}