mod input;
mod modules;
use std::f32::consts::PI;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{TextureFormat, TextureDimension, Extent3d, TextureId};
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_rapier3d::na::UnitVector3;
use bevy_rapier3d::prelude::*;
use bevy_lunex::prelude::*;
use bevy::input::common_conditions::*;
use input::{InputAxis, InputAxes};

#[derive(Component)]
struct SoundPlane(Vec<Vec<Vec<f32>>>);

#[derive(Component)]
struct SoundImage(Handle<Image>);

#[derive(Component, Default)]
struct Player {
    vertical_velocity: f32,
    velocity: Vec3,
    max_speed: f32,
    acceleration: f32,
    grounded: bool,
}

#[derive(Component)]
struct SoundView;

fn main() {
    App::new() 
        .add_plugins((DefaultPlugins, UiPlugin))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, start)
        .add_systems(Startup, cursor_grab)
        .add_systems(Startup, spawn_ui)
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
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut u = vec![vec![vec![0.0;80];80];2];
    u[1][40][40] = 1.0;
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

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_rotation(Quat::from_rotation_y(PI)),
        ..default()
    });

    let texture = Image::new(
        Extent3d { width: 80, height: 80, ..default() },
        TextureDimension::D2,
        vec![0xFF; 80 * 80 * 4],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD
    );

    let texture_handle = images.add(texture);

    commands.spawn(SoundImage(texture_handle.clone()));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::new(Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 1.0))),
            material: materials.add(
                StandardMaterial {
                    base_color_texture: Some(texture_handle),
                    ..default()
                }
                ),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        SoundView
    ));
    
    commands.insert_resource(InputAxes::default());
}


fn cursor_grab(
    mut q_cursor: Query<&mut Cursor2d>,
) {
    if let Ok(mut cursor) = q_cursor.get_single_mut() {
        cursor.confined = true;
        cursor.visible = true;
    }
}

fn cursor_ungrab(
    mut q_cursor: Query<&mut Cursor2d>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut cursor) = q_cursor.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Escape) {
            cursor.confined = false;
            cursor.visible = true;
        } 
    }
}

fn update(
    mut soundplane: Query<&mut SoundPlane>,
    mut query: Query<&mut SoundImage>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    let mut u = soundplane.get_single_mut().unwrap();
    u.0[1][40][40] = (time.elapsed().as_secs() as f32).sin();
    let next = calculate_next_step(&u.0, 1.0, 0.125, 0.125, 0.025, 0.005);
    u.0[0] = u.0[1].clone();
    u.0[1] = next;
    
    let handle = &mut query.single_mut().0;
    let texture = images.get_mut(handle).unwrap();
    for x in 0..80 {
        for y in 0..80 {
            let value = (u.0[u.0.len() - 1][x][y] * 128.0 + 127.0) as u8;
            texture.data[(x * 80 + y) * 4] = value;
            texture.data[(x * 80 + y) * 4 + 1] = (time.elapsed().as_secs() * 10) as u8;
            texture.data[(x * 80 + y) * 4 + 2] = 254;
        }
    }


    // let ui = ui.single_mut();
    // materials.get(ui.material).unwrap().base_color_texture = Some(handle.clone());

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
    //;if primary_window.cursor.grab_mode == CursorGrabMode::None { return }

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

fn draw_screen(u: &Vec<Vec<f32>>) -> String {
    let mut string = String::new();
    string.push_str("\x1b[H");
    for y in 0..40 {
        for x in 0..80 {
            string.push_str(&format!("\x1b[38;2;{0};{0};{0}\x1b[48;2;{1};{1};{1}m ", 
                (u[x][y*2] * 200.0 + 126.0) as i16,
                (u[x][y*2 + 1] * 200.0 + 126.0) as i16));
        }
        string.push_str("\n");
    }
    string
}

fn calculate_next_step(u: &Vec<Vec<Vec<f32>>>, c: f32, dx: f32, dy: f32, dt: f32, f: f32) -> Vec<Vec<f32>> {
    let n = 80;
    let mut new_state = vec![vec![0.0; n]; n];
    let t = u.len() - 1;

    // let cfl_condition = c * dt <= f32::sqrt(dx * dx + dy * dy);
    // assert!(cfl_condition, "CFL condition violated");

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

fn draw_rect(u: &mut Vec<Vec<f32>>, sx: usize, sy: usize, ex: usize, ey: usize) {
    for x in sx..ex {
        for y in sy..ey {
            u[x][y] /= 2.0;
        }
    }
}

fn spawn_ui(
    mut commands: Commands,
    mut material: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    // Spawn cursor
    commands.spawn(CursorBundle::default());


    // Spawn the floating Ui panel
    commands.spawn((
        UiTreeBundle::<MainUi> {
            transform: Transform::from_xyz(0.4, 0.3, 0.3),
            tree: UiTree::new3d("PanelWidget"),
            ..default()
        },
    )).with_children(|ui| {
        ui.spawn((
            // Link this widget
            UiLink::<MainUi>::path("Menu/Button"),

            // The layout that is used when in base state
            UiLayout::window_full().size((818.0, 965.0)).pack::<Base>(),

            // // Give the mesh an image
            // UiMaterial3dBundle::from_transparent_image(&mut material, texture_handle),
            UiColor::<Base>::new(Color::srgb_from_array([1.0, 0.0, 0.0])),
            UiColor::<Hover>::new(Color::srgb_from_array([1.0, 1.0, 0.0])),

            // Make the panel pickable
            PickableBundle::default(),

            // This is required to control our hover animation
            UiAnimator::<Hover>::new().forward_speed(6.0).backward_speed(5.0),

            // This is required for Layout animation
            UiLayoutController::default(),

            // The layout that is used when in hover state
            UiLayout::window_full().x(100.0).size((818.0, 965.0)).pack::<Hover>(),
            UiClickEmitter::SELF,
        ));
    }); 
}
