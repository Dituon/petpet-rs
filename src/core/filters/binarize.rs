use skia_safe::RuntimeEffect;

pub fn binarize_shader() -> RuntimeEffect {
    RuntimeEffect::make_for_shader(
        include_str!("./binarize.glsl"),
        None,
    ).unwrap()
}