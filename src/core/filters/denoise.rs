use skia_safe::RuntimeEffect;

pub fn denoise_shader() -> RuntimeEffect {
    RuntimeEffect::make_for_shader(
        include_str!("./denoise.glsl"),
        None,
    ).unwrap()
}