use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .insert_resource(GameTick {time_elapsed: 0})
        .add_systems(Startup, setup)
        .add_systems(Update, test)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let col = 0xff00ff;

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 100.0))),
        MeshMaterial2d(materials.add(hex_color(col))),
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

#[derive(Resource)]
struct GameTick {
    time_elapsed: usize,
}

fn test(mut ticker: ResMut<GameTick>, time: Res<Time>) {
    let delta: usize = time.delta().as_millis() as usize;
    ticker.time_elapsed = ticker.time_elapsed + delta;
    println!("{}", ticker.time_elapsed);
}
