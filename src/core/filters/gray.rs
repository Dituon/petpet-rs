use skia_safe::RuntimeEffect;

pub fn gray_shader() -> RuntimeEffect {
    RuntimeEffect::make_for_shader(
        include_str!("./gray.glsl"),
        None,
    ).unwrap()
}