use skia_safe::RuntimeEffect;

pub fn oil_shader() -> RuntimeEffect {
    RuntimeEffect::make_for_shader(
        include_str!("./oil.glsl"),
        None,
    ).unwrap()
}