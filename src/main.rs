use bevy::prelude::*;
use tetris_bevy::components::TileComponent;
use tetris_bevy::constants::{CUTOFF, HEIGHT, TICKSPEED, UNIT, WIDTH};
use tetris_bevy::resources::{GameTick, World};
use tetris_bevy::utils::{
    BlockType, CurrentBlock, check_game_over, compute_grid_coordinate, fill, get_locations,
    hex_color, row_clearing, valid_position,
};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .insert_resource(World::new())
        .insert_resource(GameTick(Timer::from_seconds(
            TICKSPEED,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, tile_update)
        .add_systems(Update, game_loop)
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
                MeshMaterial2d(materials.add(BlockType::No.get_color())),
                Transform::from_xyz(a, b, 0.0),
                TileComponent { x, y },
            ));
        }
    }

    let (a, b) = compute_grid_coordinate((WIDTH / 2.0).round() as usize, CUTOFF + 1);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(WIDTH * UNIT, 1.0))),
        MeshMaterial2d(materials.add(hex_color(0xff0000))),
        Transform::from_xyz(a - UNIT / 2.0, b + UNIT / 2.0, 0.0),
    ));
}

fn tile_update(
    mut query: Query<(&MeshMaterial2d<ColorMaterial>, &TileComponent)>,
    world: Res<World>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    query.iter_mut().for_each(|(mat, tile)| {
        let tile: BlockType = world.grid[tile.y][tile.x];
        let material = materials.get_mut(mat.id()).unwrap();
        material.color = tile.get_color();
    });
}

fn game_loop(
    mut world: ResMut<World>,
    mut ticker: ResMut<GameTick>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if ticker.0.tick(time.delta()).just_finished() {
        let locations = get_locations(world.current.kind);
        let offset = world.current.location;
        fill(&mut world, locations, offset, BlockType::No);

        // Apply gravity
        world.current.location.1 += 1;

        let locations = get_locations(world.current.kind);
        let offset = world.current.location;
        let kind = world.current.kind;
        if valid_position(&world, &locations, offset) {
            // Continue falling
            fill(&mut world, locations, offset, kind);
        } else {
            // Freeze the block
            world.current.location.1 -= 1;

            // Redraw the block
            let locations = get_locations(world.current.kind);
            let offset = world.current.location;
            let kind = world.current.kind;
            fill(&mut world, locations, offset, kind);

            // Check Row Clearing
            row_clearing(&mut world);

            // Check Game Over
            if check_game_over(&world) {
                println!("Game Over!");
                exit.write(AppExit::Success);
            }

            // Remove the object form the falling
            world.current = CurrentBlock::new();
        }
    }

    let locations = get_locations(world.current.kind);
    let offset = world.current.location;
    fill(&mut world, locations, offset, BlockType::No);

    let mut undo = 0;

    if keys.just_pressed(KeyCode::ArrowLeft) {
        world.current.location.0 -= 1;
        undo += 1;
    } else if keys.just_pressed(KeyCode::ArrowRight) {
        world.current.location.0 += 1;
        undo -= 1;
    }

    let locations = get_locations(world.current.kind);
    let offset = world.current.location;
    let kind = world.current.kind;
    if valid_position(&world, &locations, offset) {
        fill(&mut world, locations, offset, kind);
    } else {
        // Undo changes
        world.current.location.0 += undo;

        let locations = get_locations(world.current.kind);
        let offset = world.current.location;
        let kind = world.current.kind;
        fill(&mut world, locations, offset, kind);
    }

    // Smash down
    if keys.just_pressed(KeyCode::Space) {
        // Remove the old position
        let locations = get_locations(world.current.kind);
        let offset = world.current.location;
        fill(&mut world, locations, offset, BlockType::No);

        let mut locations = get_locations(world.current.kind);
        let mut offset = world.current.location;
        while valid_position(&world, &locations, offset) {
            // Go down until you have reached an invalid position
            world.current.location.1 += 1;

            locations = get_locations(world.current.kind);
            offset = world.current.location;
        }

        // Redraw
        world.current.location.1 -= 1;

        // Smashed down
        let locations = get_locations(world.current.kind);
        let offset = world.current.location;
        let kind = world.current.kind;
        fill(&mut world, locations, offset, kind);

        // Check Row Clearing
        row_clearing(&mut world);

        // Check Game Over
        if check_game_over(&world) {
            println!("Game Over!");
            exit.write(AppExit::Success);
        }

        // Replace
        world.current = CurrentBlock::new();
    }
}
