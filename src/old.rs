use bevy::prelude::*;

#[derive(Component)]
struct MyCameraMarker;

fn main() {
    let mut domain = vec![guass();2];

    println!("{:?}", domain[0]);
    for i in 1..2500 {
        domain.push(calculate_next_step(&domain[i-1], &domain[i], 1.0, 0.01, 0.01));

        print!("\x1b[H\x1b[24B");
        for j in 0..24 {
            print!("\r");
            for n in &domain[i] {
                if *n >= j as f64 - 12.0 {
                    print!("#");
                } else {
                    print!(" ")
                }
            }
            print!("\x1b[A");
        }
        print!("{}", domain[i].iter().map(|n| format!("\x1b[38;2;{0};{0};{0}m#", *n as i64 + 128)).collect::<Vec<String>>().join(""));
        std::thread::sleep(std::time::Duration::from_millis(30));
    }

    // App::new() 
    //     .add_plugins(DefaultPlugins)
    //     .add_systems(Startup, start)
    //     .run();
}

fn calculate_next_step(previous_state: &[f64], current_state: &[f64], c: f64, dx: f64, dt: f64) -> Vec<f64> {
    let n = current_state.len();
    let mut new_state = vec![0.0; n];

    for i in 1..n - 1 {
        new_state[i] = 2.0 * current_state[i] - previous_state[i] +
            c * c * dt * dt * (current_state[i + 1] - 2.0 * current_state[i] + current_state[i - 1]) / (dx * dx);
    }

    //new_state[50] = 10.0;
    // Handle boundary conditions here
    // new_state[0] = 0.0; // Example: fixed boundary
    // new_state[n - 1] = 0.0; // Example: fixed boundary

    new_state
}

fn start(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        MyCameraMarker
    ));
}
