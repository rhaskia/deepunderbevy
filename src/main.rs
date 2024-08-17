mod input;
mod modules;
use std::f32::consts::PI;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_rapier3d::prelude::*;
use bevy::input::common_conditions::*;
use input::{InputAxis, InputAxes};

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
        .add_systems(Startup, cursor_grab)
        .add_systems(Update, cursor_ungrab)
        .add_systems(Update, cursor_grab.run_if(input_just_pressed(MouseButton::Left)))
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


    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::capsule_y(1.0, 0.5))
        .insert(PbrBundle{
            mesh: meshes.add(Capsule3d::new(0.5, 2.0)),
            material: materials.add(Color::srgb_from_array([1.0, 0.0, 0.0])),
            transform: Transform::from_xyz(1.0, 3.0, 1.0),
            ..default()
        })
        .insert(KinematicCharacterController {
            snap_to_ground: Some(CharacterLength::Absolute(2.0)),
            ..KinematicCharacterController::default()
        })
        .insert(Player {
            acceleration: 0.8,
            max_speed: 5.0,
            velocity: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        });

    modules::spawn_module(&mut commands, &mut meshes, &mut materials);

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
    
    commands.insert_resource(InputAxes::default());
}


fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}

fn cursor_ungrab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        let mut primary_window = q_windows.single_mut();
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
    } 
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
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let primary_window = q_windows.single_mut();

    let mut camera = camera_query.single_mut();
    let mut player = player_query.single_mut().0;

    camera.translation = player.translation + Vec3::new(0.0, 1.0, 0.0);

    // Only rotate the camera when cursor is grabbed
    if primary_window.cursor.grab_mode == CursorGrabMode::None { return }

    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.003;
        let pitch = -motion.delta.y * 0.002;
        // Order of rotations is important, see <https://gamedev.stackexchange.com/a/136175/103059>
        player.rotate_y(yaw);
        camera.rotate_y(yaw);
        camera.rotate_local_x(pitch);
    }
}

fn player_movement(
    mut query: Query<(&mut KinematicCharacterController, &mut Player, &Transform)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    axes: Res<InputAxes>,
    time: Res<Time>,
) {
    let (mut controller, mut player, transform) = query.single_mut();

    let horiz = get_key_axis(&keyboard_input, axes.get("Horizontal"));
    let forward = get_key_axis(&keyboard_input, axes.get("Vertical"));
    let move_vector = forward * transform.forward() + horiz * transform.left();
    let new_velocity = move_vector.normalize_or_zero() * player.max_speed;
    // TODO seperate accel and decel
    player.velocity = player.velocity.move_towards(new_velocity, player.acceleration);

    if player.grounded {
        player.vertical_velocity = 0.0;
    } else {
        player.vertical_velocity -= 9.8 * time.delta_seconds();
    }

    controller.translation = Some(Vec3::new(player.velocity.x, player.vertical_velocity, player.velocity.z) * time.delta_seconds());
}

fn get_key_axis(input: &Res<ButtonInput<KeyCode>>, axis: &InputAxis) -> f32 {
    let pos = axis.pos.iter().map(|key| input.pressed(*key)).any(|p| p);
    let neg = axis.neg.iter().map(|key| input.pressed(*key)).any(|p| p);
    (neg as i32 as f32) - (pos as i32 as f32)
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
