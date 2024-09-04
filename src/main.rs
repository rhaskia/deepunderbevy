use std::io::Write;
use std::time::Duration;
use crossterm::event::KeyCode;
use crossterm::event::{self, poll, Event, KeyEventKind};

fn main() {
    let mut u = vec![vec![vec![0.0; 320]; 320]; 2];
    let mut t: f32 = 0.0;

    loop {
        t += 0.1;
        if t < 6.14 { u[1][50][40] = t.sin() * 5.0 };
        let next = calculate_next_step(&u, 1.0, 0.1, 0.1, 0.02, 0.0);
        u[0] = u[1].clone();
        u[1] = next;
        if t % 1.0 < 0.1 {
            draw_screen(&u[1]);
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn draw_screen(u: &Vec<Vec<f32>>) {
    let mut string = String::new();
    string.push_str("\x1b[H\x1b[2J");
    for y in 0..40 {
        for x in 0..80 {
            let pos1 = (u[x][y*2] * 256.0) as u16;
            let neg1 = (u[x][y*2] * -256.0) as u16;
            let pos2 = (u[x][y*2 + 1] * 256.0) as u16;
            let neg2 = (u[x][y*2 + 1] * -256.0) as u16;
            string.push_str(&format!(
                "\x1b[38;2;{1};{0};0m\x1b[48;2;{3};{2};0m▀",
                pos1, neg1, pos2, neg2,
            ));
        }
        string.push_str("\x1b[m\n");
    }
    string.push_str("\x1b[m");
    print!("{string}{}", u[0][41]);
    std::io::stdout().flush();
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
    let mut c = c;

    // let cfl_condition = c * dt <= f32::sqrt(dx * dx + dy * dy);
    // assert!(cfl_condition, "CFL condition violated");

    for x in 1..n - 1 {
        for y in 1..n - 1 {
            let dudx = (u[t][x + 1][y] - 2.0 * u[t][x][y] + u[t][x - 1][y]) / (dx * dx);
            let dudy = (u[t][x][y + 1] - 2.0 * u[t][x][y] + u[t][x][y - 1]) / (dy * dy);
            let mut friction = f * (u[t][x][y] - u[t - 1][x][y]);

            new_state[x][y] = ((c * c) * (dt * dt) * (dudy + dudx)) + (2.0 * u[t][x][y])
                - u[t - 1][x][y]
                - friction;
        }
    }

    let dtdy = dt / dy;
    let dtdx = dt / dx;

    for y in 1..n-1 {
        for x in n-10..n {
            let dudx = (c * (dt / dy)) * ((u[t][x][y]) - u[t][x - 1][y]);
            new_state[x][y] = u[t][x][y] - dudx;
        }
    }

    for y in 1..n-1 {
        for x in 0..10 {
            let dudx = (c * (dt / dy)) * ((u[t][x][y]) - u[t][x + 1][y]);
            new_state[x][y] = u[t][x][y] - dudx;
        }
    }

    for x in 1..n-1 {
        for y in 70..n {
            let dudy = u[t][x][y] - u[t][x][y - 1];
            new_state[x][y] = u[t][x][y] - (c * dtdy * dudy);
        }
    }

    for x in 1..n-1 {
        for y in 0..10 {
            let dudy = u[t][x][y] - u[t][x][y + 1];
            new_state[x][y] = u[t][x][y] - (c * dtdy * dudy);
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
