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

    pub fn all() -> Vec<Dir> {
        vec![Dir::North, Dir::East, Dir::South, Dir::West]
    }
}

#[derive(Component)]
struct Door {
    open: bool,
    position: Vec3,
}

pub fn spawn_module(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(6.0, 0.2, 6.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(0.0, -3.0, 0.0),
        ..default()
    }).insert(Collider::cuboid(3.0, 0.1, 3.0));

    // Roof
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(6.0, 0.2, 6.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        ..default()
    }).insert(Collider::cuboid(3.0, 0.1, 3.0));

    for dir in [0.0] {
        // let dir_v = dir.to_vec();
        // let x = dir_v.z.abs() * 5.8 + 0.2;
        // let z = dir_v.x.abs() * 5.8 + 0.2;
        //
        // commands.spawn(PbrBundle {
        //     mesh: meshes.add(Cuboid::new(x, 6.0, z)),
        //     material: materials.add(Color::WHITE),
        //     transform: Transform::from_xyz(dir_v.x * 3.0, 0.0, dir_v.z * 3.0),
        //     ..default()
        // })
        // .insert(Door { open: false, position: dir.to_vec() })
        // .insert(Collider::cuboid(x / 2.0, 3.0, z / 2.0));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 300_000.0,
            color: Color::srgb_from_array([1.0, 1.0, 0.8]),
            range: 7.0,
            radius: 0.6,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 1.8, 0.0),
        ..default()
    });
}
