use nalgebra::{Matrix4, Vector2};

#[derive(Clone, Copy)]
pub struct Transform {
    pub translation: Vector2<f32>,
    pub scale: Vector2<f32>,
}

impl From<&Transform> for Matrix4<f32> {
    fn from(transform: &Transform) -> Self {
        let translation = Matrix4::new_translation(&transform.translation.insert_row(2, 0.0));
        let scale = Matrix4::new_nonuniform_scaling(&transform.scale.insert_row(2, 1.0));
        translation * scale
    }
}

impl From<&Transform> for [[f32; 4]; 4] {
    fn from(transform: &Transform) -> Self {
        Into::<Matrix4<f32>>::into(transform).into()
    }
}
