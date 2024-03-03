use bevy::prelude::{Query, Transform, With};
use crate::game::component::material::Material;

/// 재료를 아래로 떨어뜨린다
pub fn drop_material(mut transform: Query<(&mut Transform), With<Material>>) {
    let mut transform = transform.single_mut();

    let next_y = transform.translation.y - 1.0;

    // 별 기능이 없으니 어느 정도 아래로 떨어지면 다시 복원
    let next_y = if next_y < -250.0 {
        0.0
    } else {
        next_y
    };

    transform.translation.y = next_y;
}
