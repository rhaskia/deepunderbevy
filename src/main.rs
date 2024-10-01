mod skinbuilder;

use skinbuilder::build_skin;
use noise::{
    core::perlin::{perlin_2d, perlin_3d, perlin_4d},
    permutationtable::PermutationTable,
    utils::*,
};

use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets::Group},
};

struct Speaker {
    position: Vec2,
    icon_pos: Vec2,
    grab_offset: Vec2,
    is_dragging: bool,
    volume: f32,
    offset: f32,
    speed: f32,
}

struct Block {
    position: Vec2,
    icon_pos: Vec2,
    grab_offset: Vec2,
    size: Vec2,
    is_dragging: bool,
    speed: f32,
}

impl Speaker {
    fn new(position: Vec2) -> Self {
        Speaker {
            position,
            icon_pos: position,
            grab_offset: vec2(0.0, 0.0),
            is_dragging: false,
            volume: 1.0,
            offset: 0.0,
            speed: 1.0,
        }
    }
}

#[macroquad::main("Wave Simulation")]
async fn main() {
    let mut x: f32 = 160.0;
    let mut y: f32 = 160.0;
    let mut u = vec![vec![vec![0.0; x as usize]; y as usize]; 2];

    let hasher = PermutationTable::new(0);
    let perlin = PlaneMapBuilder::new_fn(|point| perlin_2d(point.into(), &hasher))
            .set_size(x as usize, y as usize)
            .set_x_bounds(-10.0, 10.0)
            .set_y_bounds(-10.0, 10.0)
            .build();

    for x in 0..x as usize {
        for y in 0..y as usize {
            u[0][x][y] = perlin.get_value(x, y) as f32;
            u[1][x][y] = perlin.get_value(x, y) as f32;
        }
    }

    let mut t: f32 = 0.0;
    let mut params = DrawTextureParams::default();
    let mut wave_speed = 1.0;
    let mut friction = 0.0f32;
    let mut resolution = 1.0;
    let mut speakers = vec![Speaker::new(vec2(50., 50.)), Speaker::new(vec2(100., 100.0))];
    //let mut blocks = vec![];
    let skin = build_skin().await;

    root_ui().push_skin(&skin);

    loop {
        clear_background(LIGHTGRAY);
        params.dest_size = Some(Vec2::new(screen_height(), screen_height()));

        t += 0.1;

        for speaker in &speakers {
            let y = ((speaker.position.y + 10.0) / screen_height() * x) as usize;
            let x = ((speaker.position.x + 10.0) / screen_height() * x) as usize;
            u[1][y][x] = (t * speaker.speed + speaker.offset).sin() * 5.0 * speaker.volume;
        }

        // for block in &blocks {
        //     draw_rect_sim(&mut u[0], 0.0, block.position, sy, ex, ey)
        // }

        draw_double_slit(&mut u[0]);
        draw_double_slit(&mut u[1]);

        let next = calculate_next_step(
            &u,
            wave_speed,
            0.1,
            0.1,
            get_frame_time().min(0.02),
            friction / 10.0,
        );
        u[0] = u[1].clone();
        u[1] = next;

        let bytes = u[1]
            .iter()
            .map(|row| row.iter().map(|v| [(-255.0 * v) as u8, (255.0 * v) as u8, 0, 255]))
            .flatten()
            .flatten()
            .collect::<Vec<u8>>();
        let image = Texture2D::from_rgba8(x as u16, y as u16, &bytes);

        draw_texture_ex(&image, 0.0, 0.0, WHITE, params.clone());

        draw_text(&get_fps().to_string(), 10.0, 20.0, 16.0, WHITE);

        root_ui().window(
            hash!(),
            vec2(screen_height(), 0.0),
            vec2(screen_width() - screen_height(), screen_height()),
            |ui| {
                ui.slider(hash!(), "Wave Speed", 0.0..10.0, &mut wave_speed);
                ui.slider(hash!(), "Friction", 0.0..2.0, &mut friction);
                ui.slider(hash!(), "Resolution", 0.0..5.0, &mut resolution);

                if ui.button(None, "Clear") {
                    u = vec![vec![vec![0.0; x as usize]; y as usize]; 2];
                }

                if ui.button(None, "Add Speaker") {
                    speakers.push(Speaker::new(vec2(20.0, 20.0)));
                }
                ui.same_line(0.0);

                if ui.button(None, "Remove All") {
                    speakers.clear();
                }

                // if ui.button(None, "Add Block") {
                //     blocks.push(Block::new(vec2(100.0, 100.0), vec2(20.0, 20.0)));
                // }
                //ui.same_line(0.0);

                // if ui.button(None, "Remove All") {
                //     speakers.clear();
                // }

                for (n, speaker) in speakers.iter_mut().enumerate() {
                    ui.group(hash!("speakergroup", n), vec2(200.0, 70.0), |ui| {
                        ui.slider(
                            hash!("speakervolume", n),
                            "Volume",
                            0.0..5.0,
                            &mut speaker.volume,
                        );
                        ui.slider(
                            hash!("speakeroffset", n),
                            "Offset",
                            0.0..6.28,
                            &mut speaker.offset,
                        );
                        ui.slider(hash!("speakerspeed", n), "Speed", 0.0..5.0, &mut speaker.speed);
                    });
                }

                // for (n, block) in blocks.iter_mut().enumerate() {
                //     ui.group(hash!("blockgroup", n), vec2(200.0, 70.0), |ui| {
                //         ui.slider(
                //             hash!("blockspeed", n),
                //             "Wave Speed",
                //             0.0..5.0,
                //             &mut speaker.speed,
                //         );
                //     });
                // }
            },
        );

        for (n, speaker) in speakers.iter_mut().enumerate() {
            let drag = Group::new(hash!("speaker", n), Vec2::new(20., 20.))
                .position(speaker.icon_pos)
                .draggable(true)
                .ui(&mut root_ui(), |ui| {
                    ui.label(None, "");
                });

            match drag {
                macroquad::ui::Drag::No => {}
                macroquad::ui::Drag::Dragging(v, _) => {
                    if !speaker.is_dragging {
                        speaker.is_dragging = true;
                        speaker.grab_offset = speaker.position - v;
                    }
                    speaker.position = v + speaker.grab_offset;
                }
                macroquad::ui::Drag::Dropped(v, _) => {
                    speaker.icon_pos = v + speaker.grab_offset;
                    speaker.is_dragging = false;
                }
                _ => {}
            }
        }

        next_frame().await;
    }
}

fn draw_double_slit(u: &mut Vec<Vec<f32>>) {
    let height = u.len();
    let width = u[0].len();
    draw_rect_sim(u, 0.0, 0, height/2, width / 3, height/2 + 5);
    draw_rect_sim(u, 0.0, (width / 3) * 2, height/2, width, height/2 + 5);
    draw_rect_sim(u, 0.0, (width / 10) * 4, height/2, (width / 10) * 6, height/2 + 5);
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

    for x in 1..x_len - 1 {
        for y in 1..y_len - 1 {
            let dudx = (u[t][x + 1][y] - 2.0 * u[t][x][y] + u[t][x - 1][y]) / (dx * dx);
            let dudy = (u[t][x][y + 1] - 2.0 * u[t][x][y] + u[t][x][y - 1]) / (dy * dy);
            let friction = f * (u[t][x][y] - u[t - 1][x][y]);

            new_state[x][y] = ((c * c) * (dt * dt) * (dudy + dudx)) + (2.0 * u[t][x][y])
                - u[t - 1][x][y]
                - friction;
        }
    }

    let dtdy = dt / dy;
    let dtdx = dt / dx;
    let l = 10;

    for y in 1..y_len - 1 {
        for x in x_len - l..x_len {
            let dudx = (c * (dt / dy)) * ((u[t][x][y]) - u[t][x - 1][y]);
            new_state[x][y] = u[t][x][y] - dudx;
        }
    }

    for y in 1..y_len - 1 {
        for x in 0..l {
            let dudx = (c * (dt / dy)) * ((u[t][x][y]) - u[t][x + 1][y]);
            new_state[x][y] = u[t][x][y] - dudx;
        }
    }

    for x in 1..x_len - 1 {
        for y in y_len - l..y_len {
            let dudy = u[t][x][y] - u[t][x][y - 1];
            new_state[x][y] = u[t][x][y] - (c * dtdy * dudy);
        }
    }

    for x in 1..x_len - 1 {
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
            u[y][x] = value;
        }
    }
}
