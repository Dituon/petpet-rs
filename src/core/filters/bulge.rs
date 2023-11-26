use skia_safe::RuntimeEffect;

pub fn bulge_shader() -> RuntimeEffect {
    RuntimeEffect::make_for_shader(
        include_str!("./bulge.glsl"),
        None,
    ).unwrap()
}