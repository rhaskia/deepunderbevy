use std::f32::consts::PI;

use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct SoundPlane(Vec<Vec<Vec<f64>>>);

#[derive(Component)]
struct SoundImage;

#[derive(Component, Default)]
struct Player {
    vertical_velocity: f32,
    velocity: Vec3,
    max_speed: f32,
    acceleration: f32,
    grounded: bool,
}

fn main() {
    App::new() 
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, start)
        .add_systems(Update, update)
        .add_systems(Update, read_output)
        .add_systems(Update, player_movement)
        .add_systems(Update, camera_movement)
        .run();
}

fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let u = vec![vec![vec![0.0;80];80];2];
    commands.spawn(SoundPlane(u));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    }).insert(Collider::cuboid(4.0, 4.0, 0.1));

    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::capsule_y(1.0, 0.5))
        .insert(PbrBundle{
            mesh: meshes.add(Capsule3d::new(0.5, 2.0)),
            material: materials.add(Color::rgb_linear(1.0, 0.0, 0.0)),
            transform: Transform::from_xyz(1.0, 3.0, 1.0),
            ..default()
        })
        .insert(KinematicCharacterController {
            snap_to_ground: Some(CharacterLength::Absolute(2.0)),
            ..KinematicCharacterController::default()
        })
        .insert(GravityScale(1.0))
        .insert(Player {
            acceleration: 0.8,
            max_speed: 5.0,
            velocity: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_rotation(Quat::from_rotation_y(PI)),
        ..default()
    });

}

fn update(
    mut soundplane: Query<&mut SoundPlane>,
    mut assets: Res<AssetServer>,
    mut query: Query<&mut Handle<Image>, With<SoundImage>>,
) {
    // let mut u = soundplane.get_single_mut().unwrap();
    // let next = calculate_next_step(&u.0, 1.0, 0.125, 0.125, 0.025, 0.01);
    // u.0.push(next);


    //print!("{}", draw_screen(&u.0[t]));

    // std::thread::sleep(std::time::Duration::from_millis(30));
}

fn read_output(mut query: Query<(&KinematicCharacterControllerOutput, &mut Player)>) {
    if let Ok((output, mut player)) = query.get_single_mut() {
        player.grounded = output.grounded;
    }
}

fn camera_movement(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mut player_query: Query<(&mut Transform, &Player), Without<Camera3d>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let mut camera = camera_query.single_mut();
    let mut player = player_query.single_mut().0;
    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.003;
        let pitch = -motion.delta.y * 0.002;
        // Order of rotations is important, see <https://gamedev.stackexchange.com/a/136175/103059>
        player.rotate_y(yaw);
        camera.rotate_local_x(pitch);
    }

}

fn player_movement(
    mut query: Query<(&mut KinematicCharacterController, &mut Player)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut controller, mut player) = query.single_mut();

    let horiz = get_key_axis(&keyboard_input, KeyCode::ArrowLeft, KeyCode::ArrowRight);
    let vert = get_key_axis(&keyboard_input, KeyCode::ArrowDown, KeyCode::ArrowUp);
    let new_velocity = Vec3::new(horiz, 0.0, vert).normalize_or_zero() * player.max_speed;
    println!("{:?}", Vec3::new(horiz, 0.0, vert).normalize_or_zero());
    // TODO seperate accel and decel
    player.velocity = player.velocity.move_towards(new_velocity, player.acceleration);

    if player.grounded {
        player.vertical_velocity = 0.0;
    } else {
        player.vertical_velocity -= 9.8 * time.delta_seconds();
    }

    controller.translation = Some(Vec3::new(player.velocity.x, player.vertical_velocity, player.velocity.z) * time.delta_seconds());
}

fn get_key_axis(input: &Res<ButtonInput<KeyCode>>, neg: KeyCode, pos: KeyCode) -> f32 {
    (input.pressed(pos) as i32 as f32) - (input.pressed(neg) as i32 as f32)
}

fn draw_screen(u: &Vec<Vec<f64>>) -> String {
    let mut string = String::new();
    string.push_str("\x1b[H");
    for y in 0..40 {
        for x in 0..80 {
            string.push_str(&format!("\x1b[38;2;{0};{0};{0}\x1b[48;2;{1};{1};{1}m ", 
                (u[x][y*2] * 200.0 + 126.0) as i64,
                (u[x][y*2 + 1] * 200.0 + 126.0) as i64));
        }
        string.push_str("\n");
    }
    string
}

fn calculate_next_step(u: &Vec<Vec<Vec<f64>>>, c: f64, dx: f64, dy: f64, dt: f64, f: f64) -> Vec<Vec<f64>> {
    let n = 80;
    let mut new_state = vec![vec![0.0; n]; n];
    let t = u.len() - 1;

    let cfl_condition = c * dt <= f64::sqrt(dx * dx + dy * dy);
    assert!(cfl_condition, "CFL condition violated");

    for x in 1..n - 1 {
        for y in 1..n-1 {
            let dudx = (u[t][x+1][y] - 2.0*u[t][x][y] + u[t][x-1][y])/(dx*dx);
            let dudy = (u[t][x][y+1] - 2.0*u[t][x][y] + u[t][x][y-1])/(dy*dy);
            let friction = f * (u[t][x][y] - u[t-1][x][y]);

            new_state[x][y] = ((c*c) * (dt*dt) * (dudy + dudx))
                            + (2.0*u[t][x][y]) - u[t-1][x][y]
                            - friction;
        }
    }

    new_state
}

fn draw_rect(u: &mut Vec<Vec<f64>>, sx: usize, sy: usize, ex: usize, ey: usize) {
    for x in sx..ex {
        for y in sy..ey {
            u[x][y] /= 2.0;
        }
    }
}
