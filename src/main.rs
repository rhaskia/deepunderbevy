use bevy::prelude::*;

#[derive(Component)]
struct SoundPlane(Vec<Vec<Vec<f64>>>);
#[derive(Component)]
struct SoundImage;

fn main() {
    // u[0][40][40] = 0.0f64.sin();
    // u[1][40][40] = 0.1f64.sin();


    App::new() 
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, start)
        .add_systems(Update, update)
        .run();
}

fn start(
    mut commands: Commands,
    mut textures: ResMut<Assets<Texture>>,
) {
    let u = vec![vec![vec![0.0;80];80];2];

    let (width, height) = (256, 256);
    let mut bytes = Vec::with_capacity(80 * 80 * 4);
    for _y in 0..height {
        for _x in 0..width {
            bytes.push(0xff);
            bytes.push(0x00);
            bytes.push(0x00);
            bytes.push(0xff);
        }
    }

    let texture = Texture::new(
        Extent3d::new(width as u32, height as u32, 1),
        TextureDimension::D2,
        bytes,
    );

    let texture_handle = textures.add(texture);

    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: texture_handle.clone(),
        ..Default::default()
    });
}

fn update(
    mut soundplane: Query<&mut SoundPlane>,
    mut assets: Res<AssetServer>,
    mut query: Query<&mut Handle<Image>, With<SoundImage>>,
) {
    let mut u = soundplane.get_single_mut().unwrap();
    // if t < 1000 { u[t][40][60] = (t as f64 /10.0).sin(); }
    // if t < 320 { u[t][t/4][20] = (t as f64 / 2.0).sin(); }
    // draw_rect(&mut u[t], 20, 20, 35, 35);
    let next = calculate_next_step(&u.0, 1.0, 0.125, 0.125, 0.025, 0.01);
    u.0.push(next);

    let image = query.single_mut();
    (*image)[0] = 1.0;
    *image.dirty();

    //print!("{}", draw_screen(&u.0[t]));

    // std::thread::sleep(std::time::Duration::from_millis(30));
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
