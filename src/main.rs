use bevy::prelude::*;

const UNIT: f32 = 50.0;
const WIDTH: f32 = 10.0;
const HEIGHT: f32 = 20.0;
const EMPTY_COLOR: i32 = 0x2b2b2b;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .insert_resource(World::new())
        .add_systems(Startup, setup)
        .add_systems(Update, tile_update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut window: Single<&mut Window>,
    world: ResMut<World>,
) {
    window.resolution.set(UNIT * 10.0, UNIT * 20.0);

    commands.spawn(Camera2d);

    for x in 0..WIDTH as usize {
        for y in 0..HEIGHT as usize {
            let (a, b) = compute_grid_coordinate(x, y);
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(UNIT, UNIT))),
                MeshMaterial2d(materials.add(hex_color(EMPTY_COLOR))),
                Transform::from_xyz(a, b, 0.0),
                TileComponent { x, y },
            ));
        }
    }
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
    (
        UNIT / 2.0 + UNIT * x as f32 - UNIT * 10.0 / 2.0,
        UNIT / 2.0 - UNIT * y as f32 + UNIT * 20.0 / 2.0 - UNIT,
    )
}

#[derive(Copy, Clone)]
enum TileType {
    I,
    J,
    L,
    O,
    S,
    Z,
    T,
    No,
}

#[derive(Resource)]
struct World {
    grid: [[TileType; 10]; 20],
}

impl World {
    fn new() -> Self {
        Self {
            grid: [[TileType::No; WIDTH as usize]; HEIGHT as usize],
        }
    }
}

#[derive(Component)]
struct TileComponent {
    x: usize,
    y: usize,
}

fn tile_update(
    mut query: Query<(&mut MeshMaterial2d<ColorMaterial>, &TileComponent)>,
    world: Res<World>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    query.iter_mut().for_each(|(mut mat, tile)| {
        let tile: TileType = world.grid[tile.y][tile.x];
        let color = match tile {
            TileType::I => hex_color(0x00ffff),
            TileType::J => hex_color(0x0000ff),
            TileType::L => hex_color(0xff7f00),
            TileType::O => hex_color(0xffff00),
            TileType::S => hex_color(0x00ff00),
            TileType::Z => hex_color(0xff0000),
            TileType::T => hex_color(0x800080),
            TileType::No => hex_color(EMPTY_COLOR),
        };
        let material = materials.get_mut(mat.id()).unwrap();
        material.color = color;
    });
}
