use bevy::{prelude::*, window::close_on_esc};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_scene)
        .add_system(close_on_esc)
        .add_system(player_movement)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0),
            ..default()
        })
        .insert(Player);
}

#[derive(Component)]
struct Player;

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player = query.get_single_mut().unwrap();

    let mut movement = Vec3::default();
    let moves = [
        (KeyCode::W, -Vec3::Z),
        (KeyCode::A, -Vec3::X),
        (KeyCode::S, Vec3::Z),
        (KeyCode::D, Vec3::X),
    ];

    for (key, direction) in moves {
        if keyboard_input.pressed(key) {
            movement += direction;
        }
    }

    let speed = 0.1;
    player.translation += movement.normalize_or_zero() * speed;
}
