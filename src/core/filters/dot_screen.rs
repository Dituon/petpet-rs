use skia_safe::RuntimeEffect;

pub fn dot_screen_shader() -> RuntimeEffect {
    RuntimeEffect::make_for_shader(
        include_str!("./dot_screen.glsl"),
        None,
    ).unwrap()
}