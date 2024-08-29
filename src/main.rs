mod input;
mod modules;
use std::f32::consts::PI;
use bevy::input::common_conditions::*;
use input::{InputAxis, InputAxes};
use bevy::prelude::*;
use std::time::Duration;
use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use bevy_ratatui::{
    error::exit_on_error,
    event::KeyEvent,
    input_forwarding::{Capability, Detected, Emulate, EmulationPolicy, ReleaseKey},
    kitty::KittyEnabled,
    terminal::RatatuiContext,
    RatatuiPlugins,
};
use crossterm::event::KeyEventKind;
use ratatui::text::Text;
use ratatui::prelude::Rect;
use ratatui::widgets::Paragraph;
use crossterm::event::KeyCode;
use modules::Module;

fn main() {
    let wait_duration = std::time::Duration::from_secs_f64(1. / 60.); // 60 FPS
    App::new()
        .add_plugins(ScheduleRunnerPlugin::run_loop(wait_duration))
        .add_systems(Startup, start)
        .add_systems(Update, draw_scene_system.pipe(exit_on_error))
        .add_systems(Update, player_movement)
        .run();
}

#[derive(Resource, Deref, DerefMut)]
struct LastKeypress(pub KeyEvent);

#[derive(Resource, Deref, DerefMut)]
struct LastBevyKeypress(pub KeyboardInput);

#[derive(Resource, Deref, DerefMut)]
struct BevyKeypresses(pub Vec<KeyCode>);

fn draw_scene_system(
    mut context: ResMut<RatatuiContext>,
    player: Query<&Player>,
    modules: Query<&Module>,
) -> color_eyre::Result<()> {
    let player = player.single();

    context.draw(|frame| {
        let width = frame.size().width / 2;
        let height = frame.size().height / 2;

        for module in &modules {
            let x = (module.position.x * 6) - player.position.x + (width as i32);
            let y = (module.position.y * 4) - player.position.y + (height as i32);
            let rect = Rect::new(x as u16, y as u16, 7, 6);
            let sprite = Paragraph::new("#-----#\n|     |\n|     |\n|     |\n#-----#");
            frame.render_widget(sprite, rect);
        }

        let sprite = Paragraph::new("@");
        let x = frame.size().width / 2;
        let y = frame.size().height / 2;
        frame.render_widget(sprite, Rect::new(x, y, 1, 1));
    })?;

    Ok(())
}

#[derive(Component)]
struct SoundPlane(Vec<Vec<Vec<f32>>>);

#[derive(Component)]
struct SoundImage(Handle<Image>);

#[derive(Component, Default)]
struct Player {
    position: IVec2,
}

#[derive(Component)]
struct SoundView;

#[derive(Component)]
struct SoundMat(Handle<StandardMaterial>);

fn start(
    mut commands: Commands,
) {
    let mut u = vec![vec![vec![0.0;80];80];2];
    u[1][40][40] = 1.0;
    commands.spawn(SoundPlane(u));

    commands.spawn(Player::default());

    commands.spawn(Module::new(0,0));
    commands.spawn(Module::new(1,0));

    commands.insert_resource(InputAxes::default());
}

fn update(
    mut soundplane: Query<&mut SoundPlane>,
    mut query: Query<&mut SoundImage>,
    mut q_mat: Query<&mut SoundMat>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_q: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut u = soundplane.single_mut();
    let mut player = player_q.single();
    let pos = player.translation * 16.0;
    u.0[1][(pos.z + 40.0) as usize][(pos.x + 40.0) as usize] = (time.elapsed().as_secs() as f32 * 2.0).sin();
    let next = calculate_next_step(&u.0, 1.0, 0.125, 0.125, 0.025, 0.01);
    u.0[0] = u.0[1].clone();
    u.0[1] = next;
}

fn player_movement(
    mut query: Query<&mut Player>,
    mut events: EventReader<KeyEvent>,
    mut exit: EventWriter<AppExit>,
    axes: Res<InputAxes>,
    time: Res<Time>,
) {
    let mut player = query.single_mut();

    for event in events.read() {
        use KeyCode::*;
        if event.kind != KeyEventKind::Press { continue; }
        match event.code {
            Char('w') => player.position.y -= 1, 
            Char('a') => player.position.x -= 1,
            Char('s') => player.position.y += 1, 
            Char('d') => player.position.x += 1,
            Char('q') => { exit.send_default(); },
            _ => {}
        }
    }
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
