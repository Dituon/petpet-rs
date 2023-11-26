use skia_safe::RuntimeEffect;

pub fn pinch_shader() -> RuntimeEffect {
    RuntimeEffect::make_for_shader(
        include_str!("./pinch.glsl"),
        None,
    ).unwrap()
}