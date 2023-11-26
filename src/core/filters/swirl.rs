use skia_safe::RuntimeEffect;

pub fn swirl_shader() -> RuntimeEffect {
    RuntimeEffect::make_for_shader(
        include_str!("./swirl.glsl"),
        None,
    ).unwrap()
}