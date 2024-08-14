use bevy::prelude::*;

#[derive(Component)]
struct MyCameraMarker;

fn main() {
    let mut u = vec![vec![vec![0.0;80];80];2];
    u[0][40][40] = 0.0f64.sin();
    u[1][40][40] = 0.1f64.sin();

    for t in 1..4000 {
        if t < 100 { u[t][40][40] = (t as f64 /10.0).sin(); }
        draw_rect(&mut u[t], 20, 20, 35, 35);
        u.push(calculate_next_step(&u, 1.0, 0.125, 0.125, 0.025));

        print!("\x1b[H");
        for y in 0..40 {
            for x in 0..80 {
                print!("\x1b[38;2;{0};{0};{0}\x1b[48;2;{1};{1};{1}m ", 
                    (u[t][x][y*2] * 200.0 + 126.0) as i64,
                    (u[t][x][y*2 + 1] * 200.0 + 126.0) as i64);
            }
            print!("\r\n");
        }

        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}

fn calculate_next_step(u: &Vec<Vec<Vec<f64>>>, c: f64, dx: f64, dy: f64, dt: f64) -> Vec<Vec<f64>> {
    let n = 80;
    let mut new_state = vec![vec![0.0; n]; n];
    let t = u.len() - 1;

    let cfl_condition = c * dt <= f64::sqrt(dx * dx + dy * dy);
    assert!(cfl_condition, "CFL condition violated");

    for x in 1..n - 1 {
        for y in 1..n-1 {
            let dudx = (u[t][x+1][y] - 2.0*u[t][x][y] + u[t][x-1][y])/(dx*dx);
            let dudy = (u[t][x][y+1] - 2.0*u[t][x][y] + u[t][x][y-1])/(dy*dy);

            new_state[x][y] = ((c*c) * (dt*dt) * (dudy + dudx))
                            + (2.0*u[t][x][y]) - u[t-1][x][y];
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

fn start(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        MyCameraMarker
    ));
}
