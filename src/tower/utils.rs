use amethyst::core::{
    math::{Point3, Vector2, Vector3},
    Transform,
};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::Camera;
use amethyst::window::ScreenDimensions;

pub fn in_range(origin: &Transform, radius: f32, point: &Transform) -> bool {
    let t1 = origin.translation();
    let t2 = point.translation();
    let distance = ((t1[0] - t2[0]).powf(2.0) + (t1[1] - t2[1]).powf(2.0)).sqrt();
    distance <= radius
}

pub fn normalize(origin: &Transform, dest: &Transform) -> Vector3<f32> {
    let t1 = origin.translation();
    let t2 = dest.translation();
    let mut norm = Vector3::new(t2[0] - t1[0], t2[1] - t1[1], 0.0);
    let magnitude = ((norm.x * norm.x) + (norm.y * norm.y)).sqrt();
    norm.x = norm.x / magnitude;
    norm.y = norm.y / magnitude;
    norm
}

/// Gets the mouse position after Left-Mouse-Button is pressed in terms of
/// world coordinates.
pub fn mouse_position(
    input: &InputHandler<StringBindings>,
    dim: &ScreenDimensions,
    camera: &Camera,
    camera_trans: &Transform,
) -> Option<Transform> {
    if input.mouse_button_is_down(amethyst::winit::MouseButton::Left) {
        if let Some(m_pos) = input.mouse_position() {
            let screen_dimensions = Vector2::new(dim.width(), dim.height());
            let mouse_pos = Point3::new(m_pos.0, m_pos.1, 0.0);
            let mouse_coords = camera.screen_to_world_point(
                mouse_pos,
                screen_dimensions,
                &camera_trans,
            );
            let mut mouse_trans = Transform::default();
            mouse_trans.set_translation_xyz(mouse_coords.coords[0], mouse_coords.coords[1], 1.0);
            return Some(mouse_trans);
        }
    }
    None
}
