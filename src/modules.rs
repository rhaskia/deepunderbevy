use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
enum Dir {
    North, South, East, West
}

impl Dir {
    pub fn to_vec(&self) -> Vec3 {
        match self {
            Dir::North => Vec3::Z,
            Dir::South => Vec3::Z * -1.0,
            Dir::East => Vec3::X * -1.0,
            Dir::West => Vec3::X,
        }
    }
}

pub fn spawn_module(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(6.0, 0.2, 6.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(0.0, -3.0, 0.0),
        ..default()
    }).insert(Collider::cuboid(3.0, 0.1, 3.0));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.2, 6.0, 6.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(3.0, 0.0, 0.0),
        ..default()
    }).insert(Dir::East)
    .insert(Collider::cuboid(0.1, 3.0, 3.0));
}
