use bevy::prelude::*;

const UNIT: f32 = 50.0;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .insert_resource(GameTick {time_elapsed: 0})
        .insert_resource(World {grid: [[BlockType::No; 10]; 20]})
        .add_systems(Startup, setup)
        .add_systems(Update, test)
        .add_systems(Update, grid_renderer)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut window: Single<&mut Window>,
    mut world: ResMut<World>,
) {
    window.resolution.set(UNIT * 10.0, UNIT * 20.0);

    // Debug Borders
    world.grid[0][0] = BlockType::I;
    world.grid[0][1] = BlockType::T;
    world.grid[19][0] = BlockType::J;
    world.grid[19][9] = BlockType::T;
    world.grid[0][9] = BlockType::L;

    commands.spawn(Camera2d);

    for (i, y) in world.grid.into_iter().enumerate() {
        for (j, x) in y.into_iter().enumerate() {
            let color = match x {
                BlockType::I => hex_color(0x00ffff),
                BlockType::J => hex_color(0x0000ff),
                BlockType::L => hex_color(0xff7f00),
                BlockType::O => hex_color(0xffff00),
                BlockType::S => hex_color(0x00ff00),
                BlockType::Z => hex_color(0xff0000),
                BlockType::T => hex_color(0x800080),
                BlockType::No => hex_color(0x2b2b2b),
            };

            let (a, b) = compute_grid_coordinate(j, i);
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(UNIT, UNIT))),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(a, b, 0.0),
            ));
        }
    }

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(1.0))),
        MeshMaterial2d(materials.add(hex_color(0xff0000))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn hex_color(hex: i32) -> Color {
    let mut hex = hex;
    let blue = (hex % 256) as f32 / 256.0;
    hex = hex / 256;
    let green = (hex % 256) as f32 / 256.0;
    hex = hex / 256;
    let red = (hex % 256) as f32 / 256.0;

    Color::srgb(red, green, blue)
}

fn compute_grid_coordinate(x: usize, y: usize) -> (f32, f32) {
   (UNIT / 2.0 + UNIT * x as f32 - UNIT * 10.0 / 2.0, UNIT / 2.0 + UNIT * y as f32 - UNIT * 20.0 / 2.0)
}

#[derive(Resource)]
struct GameTick {
    time_elapsed: usize,
}

#[derive(Resource)]
struct World {
    grid: [[BlockType; 10]; 20]
}

#[derive(Copy, Clone)]
enum BlockType {
    I,
    J,
    L,
    O,
    S,
    Z,
    T,
    No,
}

fn test(mut ticker: ResMut<GameTick>, time: Res<Time>) {
    let delta: usize = time.delta().as_millis() as usize;
    ticker.time_elapsed = ticker.time_elapsed + delta;
    println!("{}", ticker.time_elapsed);
}

fn grid_renderer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world: Res<World>,
) {
}
