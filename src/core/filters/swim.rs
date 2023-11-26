use skia_safe::RuntimeEffect;

pub fn swim_shader() -> RuntimeEffect {
    RuntimeEffect::make_for_shader(
        include_str!("./swim.glsl"),
        None,
    ).unwrap()
}