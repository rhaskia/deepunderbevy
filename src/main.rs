mod input;
mod modules;
mod render;
use std::io::Write;
use std::time::Duration;

use bevy::prelude::*;
use bevy::window::ExitCondition;
use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use crossterm::event::KeyCode;
use crossterm::event::{self, poll, Event, KeyEventKind};
use modules::Module;
use render::Rect;

fn main() {
    let wait_duration = std::time::Duration::from_secs_f64(1. / 60.); // 60 FPS

    App::new()
        //.add_plugins(ScheduleRunnerPlugin::run_loop(wait_duration))
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, render::enter_screen)
        .add_systems(Startup, start)
        .add_systems(Update, draw_scene_system)
        .add_systems(Update, update)
        .add_systems(Update, player_movement)
        .run();
}

#[derive(Resource)]
struct ScreenSize {
    width: i32,
    height: i32,
}

#[derive(Resource)]
struct ScreenDirty(bool);

fn draw_scene_system(
    mut size: ResMut<ScreenSize>,
    player: Query<&Player>,
    modules: Query<&Module>,
    mut dirty: ResMut<ScreenDirty>,
) {
    if !dirty.0 { return; } 
    dirty.0 = false;
    render::clear();

    let player = player.single();

    let width = size.width / 2;
    let height = size.height / 2;

    for module in &modules {
        let x = (module.position.x * 7) - player.position.x + (width as i32);
        let y = (module.position.y * 3) - player.position.y + (height as i32);
        let rect = Rect::new(x, y, 8, 4);
        render::draw_module(rect);
    }

    let x = size.width / 2;
    let y = size.height / 2;
    render::draw_char('@', x, y);

    std::io::stdout().flush().unwrap();
}

#[derive(Component)]
struct SoundPlane(Vec<Vec<Vec<f32>>>);

#[derive(Component, Default)]
struct Player {
    position: IVec2,
}

#[derive(Component)]
struct SoundView;

fn start(mut commands: Commands) {
    let mut u = vec![vec![vec![0.0; 80]; 80]; 2];
    u[1][40][40] = 1.0;
    commands.spawn(SoundPlane(u));

    commands.spawn(Player::default());

    commands.spawn(Module::new(0, 0));
    commands.spawn(Module::new(1, 0));

    commands.insert_resource(ScreenSize {
        width: 80,
        height: 24,
    });

    commands.insert_resource(ScreenDirty(true));

    render::hide_cursor();
}

fn update(
    mut soundplane: Query<&mut SoundPlane>,
    mut commands: Commands,
    player_q: Query<&mut Player>,
    time: Res<Time>,
) {
    let mut u = soundplane.single_mut();
    let mut player = player_q.single();
    let next = calculate_next_step(&u.0, 1.0, 0.125, 0.125, 0.025, 0.01);
    u.0[0] = u.0[1].clone();
    u.0[1] = next;
}

fn player_movement(mut query: Query<&mut Player>, mut exit: EventWriter<AppExit>, mut size: ResMut<ScreenSize>, mut dirty: ResMut<ScreenDirty>) {
    let mut player = query.single_mut();

    loop {
        if poll(Duration::from_millis(10)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key) => {
                    use KeyCode::*;
                    match key.code {
                        Char('w') => player.position.y -= 1,
                        Char('a') => player.position.x -= 1,
                        Char('s') => player.position.y += 1,
                        Char('d') => player.position.x += 1,
                        Char('q') => {
                            render::leave_screen();
                            exit.send_default();
                        }
                        _ => {}
                    }
                    dirty.0 = true;
                }
                Event::Resize(width, height) => {
                    size.width = width as i32;
                    size.height = height as i32;
                },
                _ => {}
            }
        } else {
            break;
        }
    }
}

fn draw_screen(u: &Vec<Vec<f32>>) -> String {
    let mut string = String::new();
    string.push_str("\x1b[H");
    for y in 0..40 {
        for x in 0..80 {
            string.push_str(&format!(
                "\x1b[38;2;{0};{0};{0}\x1b[48;2;{1};{1};{1}m ",
                (u[x][y * 2] * 200.0 + 126.0) as i16,
                (u[x][y * 2 + 1] * 200.0 + 126.0) as i16
            ));
        }
        string.push_str("\n");
    }
    string
}

fn calculate_next_step(
    u: &Vec<Vec<Vec<f32>>>,
    c: f32,
    dx: f32,
    dy: f32,
    dt: f32,
    f: f32,
) -> Vec<Vec<f32>> {
    let n = 80;
    let mut new_state = vec![vec![0.0; n]; n];
    let t = u.len() - 1;

    // let cfl_condition = c * dt <= f32::sqrt(dx * dx + dy * dy);
    // assert!(cfl_condition, "CFL condition violated");

    for x in 1..n - 1 {
        for y in 1..n - 1 {
            let dudx = (u[t][x + 1][y] - 2.0 * u[t][x][y] + u[t][x - 1][y]) / (dx * dx);
            let dudy = (u[t][x][y + 1] - 2.0 * u[t][x][y] + u[t][x][y - 1]) / (dy * dy);
            let friction = f * (u[t][x][y] - u[t - 1][x][y]);

            new_state[x][y] = ((c * c) * (dt * dt) * (dudy + dudx)) + (2.0 * u[t][x][y])
                - u[t - 1][x][y]
                - friction;

            if x < 5 || x > 76 || y < 5 || y > 76 {
                new_state[x][y] *= 0.99;
            }
        }
    }

    new_state
}

fn draw_rect_sim(u: &mut Vec<Vec<f32>>, value: f32, sx: usize, sy: usize, ex: usize, ey: usize) {
    for x in sx..ex {
        for y in sy..ey {
            u[x][y] = value;
        }
    }
}
