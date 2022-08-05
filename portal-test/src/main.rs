use bevy::{input::mouse::MouseMotion, prelude::*, window::close_on_esc};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_scene)
        .add_system(close_on_esc)
        .add_system(player_movement)
        .add_system(player_rotation)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    window.set_cursor_lock_mode(true);

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

    // Player
    commands
        .spawn()
        .insert(Player::default())
        .insert_bundle(SpatialBundle {
            transform: Transform::from_xyz(-2.0, 1.0, 5.0),
            ..default()
        })
        .with_children(|children| {
            children.spawn_bundle(Camera3dBundle { ..default() });
        });
}

#[derive(Component, Default)]
struct Player {
    pitch: f32,
    yaw: f32,
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player = query.get_single_mut().unwrap();

    let mut movement = Vec3::default();
    let moves = [
        (KeyCode::W, Vec3::Z),
        (KeyCode::A, -Vec3::X),
        (KeyCode::S, -Vec3::Z),
        (KeyCode::D, Vec3::X),
    ];

    for (key, direction) in moves {
        if keyboard_input.pressed(key) {
            movement += direction;
        }
    }
    let speed = 0.1;
    let movement = movement.normalize_or_zero() * speed;

    let forward = player.forward();
    let right = player.right();
    player.translation += movement.z * forward;
    player.translation += movement.x * right;
}

fn player_rotation(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut player: Query<(&mut Transform, &mut Player)>,
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let (mut transform, mut player) = player.get_single_mut().unwrap();
    let mut camera = camera.get_single_mut().unwrap();

    let mut delta = Vec2::default();
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }

    let sensitivity = 0.1;
    player.yaw -= delta.x * sensitivity;
    player.pitch += delta.y * sensitivity;

    player.pitch = player.pitch.clamp(-89.0, 89.9);

    camera.rotation = Quat::from_axis_angle(-Vec3::X, player.pitch.to_radians());
    transform.rotation = Quat::from_axis_angle(Vec3::Y, player.yaw.to_radians());
}
