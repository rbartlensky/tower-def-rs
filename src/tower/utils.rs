use amethyst::core::{math::Vector3, Transform};

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
