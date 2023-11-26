use skia_safe::RuntimeEffect;

pub fn contrast_shader() -> RuntimeEffect {
    RuntimeEffect::make_for_shader(
        include_str!("./contrast.glsl"),
        None,
    ).unwrap()
}