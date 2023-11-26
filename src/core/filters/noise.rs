use skia_safe::RuntimeEffect;

pub fn noise_shader() -> RuntimeEffect {
    RuntimeEffect::make_for_shader(
        include_str!("./noise.glsl"),
        None,
    ).unwrap()
}