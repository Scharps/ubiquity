use bevy::{
    color::palettes::
        tailwind::GREEN_700
    ,
    diagnostic::LogDiagnosticsPlugin,
    prelude::*,
};
mod mesh;

use map_gen::{biome::FbmDescriptor, Chunk, ChunkTransformer, CHUNK_SIZE};
use mesh::generate_quads;

fn main() {
    let mut app = bevy::prelude::App::new();
    app.add_plugins(bevy::DefaultPlugins);
    app.add_plugins(LogDiagnosticsPlugin::default());
    app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
    app.add_systems(Startup, startup);
    app.add_systems(Update, camera_movement);
    app.run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut map = map_gen::ChunkMap::new();

    let descriptor = FbmDescriptor {
        octaves: 4,
        frequency: 1.4,
        lacunarity: 0.5896,
        persistence: 2.342626,
    };
    let generator = map_gen::biome::LandGenerator::new(0, descriptor);
    for x in 0..4 {
        for z in 0..4 {
            let mut chunk = map_gen::Chunk::new((x * Chunk::length(), 0, z * Chunk::length()));
            generator.transform(&mut chunk);
            map.insert_chunk((x, 0, z), chunk);
        }
    }

    for chunk in map.chunks() {
        let mesh: Mesh = generate_quads(&chunk).with_offset(chunk.offset()).into();
        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: GREEN_700.into(),
                metallic: 0.2,
                perceptual_roughness: 0.8,
                ..default()
            })),
        ));
    }

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
    });
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            ..Default::default()
        },
        Transform::default().with_rotation(
            Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)
                * Quat::from_rotation_y(std::f32::consts::FRAC_PI_4)
                * Quat::from_rotation_z(-std::f32::consts::FRAC_PI_4),
        ),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, CHUNK_SIZE as f32, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera3d)>,
) {
    for (mut transform, _) in query.iter_mut() {
        let mut rotation = Quat::IDENTITY;
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction -= Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::X;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::X;
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            direction += Vec3::Y;
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            direction -= Vec3::Y;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            rotation = Quat::from_rotation_y(-0.5 * time.delta_secs());
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            rotation = Quat::from_rotation_y(0.5 * time.delta_secs());
        }

        let r = transform.rotation;
        transform.translation += (r * direction) * time.delta_secs() * 5.0;
        // rotate around Y axis
        transform.rotation = rotation * transform.rotation;
    }
}
