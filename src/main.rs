use std::io::Write;
use macroquad::{prelude::*, miniquad::TextureParams};

#[macroquad::main("Wave Simulation")]
async fn main() {
    let mut x = 320;
    let mut y = 320;
    let mut u = vec![vec![vec![0.0; x]; y]; 2];
    let mut t: f32 = 0.0;
    let mut params = DrawTextureParams::default();

    loop {
        clear_background(DARKGRAY);
        params.dest_size = Some(Vec2::new(screen_height(), screen_height()));

        t += 0.1;
        if t < 6.14 { u[1][x/2][y/2] = t.sin() * 5.0 };
        let next = calculate_next_step(&u, 1.0, 0.1, 0.1, 0.02, 0.0);
        u[0] = u[1].clone();
        u[1] = next;

        let bytes = u[1].iter().map(|row| row.iter().map(|v| [(-255.0 * v) as u8, (255.0 * v) as u8, 0, 255])).flatten().flatten().collect::<Vec<u8>>();
        let image = Texture2D::from_rgba8(x as u16, y as u16, &bytes);

        draw_texture_ex(&image, 0.0, 0.0, WHITE, params.clone());

        draw_text(&get_fps().to_string(), 10.0, 20.0, 16.0, WHITE);

        next_frame().await;
    }
}


fn calculate_next_step(
    u: &Vec<Vec<Vec<f32>>>,
    c: f32,
    dx: f32,
    dy: f32,
    dt: f32,
    f: f32,
) -> Vec<Vec<f32>> {
    let x_len = u[1][0].len();
    let y_len = u[1].len();
    let mut new_state = vec![vec![0.0; x_len]; y_len];
    let t = u.len() - 1;
    let mut c = c;

    // let cfl_condition = c * dt <= f32::sqrt(dx * dx + dy * dy);
    // assert!(cfl_condition, "CFL condition violated");

    for x in 1..x_len - 1 {
        for y in 1..y_len - 1 {
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
    let l = 10;

    for y in 1..y_len-1 {
        for x in x_len-l..x_len {
            let dudx = (c * (dt / dy)) * ((u[t][x][y]) - u[t][x - 1][y]);
            new_state[x][y] = u[t][x][y] - dudx;
        }
    }

    for y in 1..y_len-1 {
        for x in 0..l {
            let dudx = (c * (dt / dy)) * ((u[t][x][y]) - u[t][x + 1][y]);
            new_state[x][y] = u[t][x][y] - dudx;
        }
    }

    for x in 1..x_len-1 {
        for y in y_len-l..y_len {
            let dudy = u[t][x][y] - u[t][x][y - 1];
            new_state[x][y] = u[t][x][y] - (c * dtdy * dudy);
        }
    }

    for x in 1..x_len-1 {
        for y in 0..l {
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
