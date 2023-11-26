use skia_safe::RuntimeEffect;

pub fn hsb_shader() -> RuntimeEffect {
    RuntimeEffect::make_for_shader(
        include_str!("./hsb.glsl"),
        None,
    ).unwrap()
}