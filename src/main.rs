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

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>
) {
    for mut transform in &mut query {
        if keys.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 200.0 * time.delta_secs()
        }

        if keys.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 200.0 * time.delta_secs()
        }

        if keys.pressed(KeyCode::ArrowUp) {
            transform.translation.y += 200.0 * time.delta_secs()
        }

        if keys.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= 200.0 * time.delta_secs()
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