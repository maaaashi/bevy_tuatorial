use bevy::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 50.0);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed {
    base: f32,
    dash_multiplier: f32
}

impl Speed {
    fn current(&self, dash: bool) -> f32 {
        if dash { self.base * self.dash_multiplier } else { self.base }
    }
}

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider {
    half_size: Vec2,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Player,
        Speed { base: 200.0, dash_multiplier: 2.0 },
        Velocity::default(),
        Collider { half_size: PLAYER_SIZE / 2.0 },
        Sprite {
            custom_size: Some(PLAYER_SIZE),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        }
    ));
}

fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Speed, &mut Velocity), With<Player>>
) {
    let mut direction = Vec2::ZERO;
    if keys.pressed(KeyCode::ArrowRight) { direction.x += 1.0; }
    if keys.pressed(KeyCode::ArrowLeft) { direction.x -= 1.0; }
    if keys.pressed(KeyCode::ArrowUp) { direction.y += 1.0; }
    if keys.pressed(KeyCode::ArrowDown) { direction.y -= 1.0; }

    let direction = direction.normalize_or_zero();
    let dash = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    for (speed, mut velocity) in &mut query {
        velocity.0 = direction * speed.current(dash);
    }
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    let dt = time.delta_secs();
    for (mut transform, velocity) in &mut query {
        transform.translation += (velocity.0 * dt).extend(0.0);
    }
}

fn clamp_to_screen(
    camera: Single<(&Projection, &GlobalTransform), With<Camera2d>>,
    mut query: Query<(&mut Transform, &Collider)>,
) {
    let (projection, camera_transform) = *camera;
    let Projection::Orthographic(ortho) = projection else {
        return;
    };

    let center = camera_transform.translation().truncate();
    let view_min = center + ortho.area.min;
    let view_max = center + ortho.area.max;

    for (mut transform, collider) in &mut query {
        let min = (view_min + collider.half_size).min(center);
        let max = (view_max - collider.half_size).max(center);

        transform.translation.x = transform.translation.x.clamp(min.x, max.x);
        transform.translation.y = transform.translation.y.clamp(min.y, max.y);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_input, apply_velocity, clamp_to_screen).chain())
        .run();
}
