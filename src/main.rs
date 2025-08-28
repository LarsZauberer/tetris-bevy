use bevy::prelude::*;
use tetris_bevy::components::TileComponent;
use tetris_bevy::constants::{HEIGHT, UNIT, WIDTH};
use tetris_bevy::resources::World;
use tetris_bevy::utils::{TileType, compute_grid_coordinate};

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
) {
    window.resolution.set(UNIT * WIDTH, UNIT * HEIGHT);

    commands.spawn(Camera2d);

    for x in 0..WIDTH as usize {
        for y in 0..HEIGHT as usize {
            let (a, b) = compute_grid_coordinate(x, y);
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(UNIT, UNIT))),
                MeshMaterial2d(materials.add(TileType::No.get_color())),
                Transform::from_xyz(a, b, 0.0),
                TileComponent { x, y },
            ));
        }
    }
}

fn tile_update(
    mut query: Query<(&MeshMaterial2d<ColorMaterial>, &TileComponent)>,
    world: Res<World>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    query.iter_mut().for_each(|(mat, tile)| {
        let tile: TileType = world.grid[tile.y][tile.x];
        let material = materials.get_mut(mat.id()).unwrap();
        material.color = tile.get_color();
    });
}
