use bevy::prelude::*;

#[derive(Component)]
struct MyCameraMarker;

fn main() {
    let mut u = vec![vec![vec![0.0;50];50];2];
    u[0][24][24] = 0.0f64.sin();
    u[1][24][24] = 0.1f64.sin();

    for t in 1..2500 {
        if t < 25 { u[t][24][24] = (t as  f64 /10.0).sin(); }
        u.push(calculate_next_step(&u, 1.0, 0.1, 0.1, 0.1));

        print!("\x1b[H");
        for y in 0..25 {
            for x in 0..50 {
                print!("\x1b[38;2;20;20;{0}m\x1b[48;2;20;20;{1}m▀", 
                    (u[t][x][y*2] * 127.0 + 127.0) as i64, 
                    (u[t][x][(y*2)+1] * 127.0 + 127.0) as i64);
            }
            print!("\r\n");
        }
        //println!("\x1b[m{:?}", u[t]);

        //print!("{}", domain[i].iter().map(|n| format!("\x1b[38;2;{0};{0};{0}m#", *n as i64 + 128)).collect::<Vec<String>>().join(""));
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    // App::new() 
    //     .add_plugins(DefaultPlugins)
    //     .add_systems(Startup, start)
    //     .run();
}

fn calculate_next_step(u: &Vec<Vec<Vec<f64>>>, c: f64, dx: f64, dy: f64, dt: f64) -> Vec<Vec<f64>> {
    let n = 50;
    let mut new_state = vec![vec![0.0; n]; n];
    let t = u.len() - 1;

    for x in 1..n - 1 {
        for y in 1..n-1 {
            let dudx = (u[t][x+1][y] - (2.0 * u[t][x][y]) + u[t][x-1][y]) / (dx * dx);
            let dudy = (u[t][x][y+1] - (2.0 * u[t][x][y]) + u[t][x][y-1]) / (dy * dy);

            new_state[x][y] = (c*c) * (dt*dt) * (dudx + dudy)
                 + (2.0 * u[t][x][y]) - u[t-1][x][y];
            // new_state[x][y] = (c*c) * (dt*dt) * ( ((u[t][x+1][y] - 2.0*u[t][x][y] + u[t][x-1][y])/(dx*dx))
            //     + ((u[t][x][y+1] - 2.0*u[t][x][y] + u[t][x][y-1])/(dy*dy)) ) + 2.0*u[t][x][y] - u[t-1][x][y]
        }

    }

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
