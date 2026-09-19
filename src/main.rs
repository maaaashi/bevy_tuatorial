use bevy::{prelude::*};

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
        }
    ));
}

fn distance (dash: bool, secs: f32) -> f32 {
    let mut speed = 200.0;

    if dash { speed *= 2.0 }

    return speed * secs;
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>
) {
    let mut direction = Vec2::ZERO;
    if keys.pressed(KeyCode::ArrowRight) { direction.x += 1.0; }
    if keys.pressed(KeyCode::ArrowLeft) { direction.x -= 1.0; }
    if keys.pressed(KeyCode::ArrowUp) { direction.y += 1.0; }
    if keys.pressed(KeyCode::ArrowDown) { direction.y -= 1.0; }

    let Some(direction) = direction.try_normalize() else { return };

    let dash = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    for mut transform in &mut query {
        transform.translation += (direction * distance(dash, time.delta_secs())).extend(0.0)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}