use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Player,
        Sprite {
            custom_size: Some(Vec2::new(50.0, 50.0)),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Transform::default()
    ));
}

fn displacement (dash: bool, secs: f32) -> f32 {
    let mut speed = 200.0;

    if dash { speed *= 2.0 }

    return speed * secs;
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>
) {
    let dash = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let secs = time.delta_secs();

    for mut transform in &mut query {
        if keys.pressed(KeyCode::ArrowRight) {
            transform.translation.x += displacement(dash, secs);
        }

        if keys.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= displacement(dash, secs);
        }

        if keys.pressed(KeyCode::ArrowUp) {
            transform.translation.y += displacement(dash, secs);
        }

        if keys.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= displacement(dash, secs);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}