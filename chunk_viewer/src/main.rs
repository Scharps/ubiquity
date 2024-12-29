use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::{BLUE, RED},
    diagnostic::LogDiagnosticsPlugin,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use map_gen::{biome::FbmDescriptor, Chunk, ChunkTransformer};

fn main() {
    let mut app = bevy::prelude::App::new();
    app.add_plugins(bevy::DefaultPlugins);
    app.add_plugins(LogDiagnosticsPlugin::default());
    app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
    app.add_systems(Startup, startup);
    app.add_systems(Update, (camera_movement));
    app.run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const WIDTH: usize = 16;
    let mut chunk = map_gen::Chunk::new((0, 0, 0));

    let descriptor = FbmDescriptor {
        octaves: 4,
        frequency: 1.0,
        lacunarity: 2.0,
        persistence: 0.5,
    };

    let generator = map_gen::biome::LandGenerator::new(0, descriptor);

    generator.transform(&mut chunk);
    let mesh: Mesh = generate_quads(&chunk).into();

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: BLUE.into(),
            ..default()
        })),
    ));
    commands
        .spawn((
            PointLight {
                intensity: 1000_000.0,
                color: RED.into(),
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(8.0, 4.0, 8.0),
        ))
        .with_children(|builder| {
            builder.spawn((
                Mesh3d(meshes.add(Sphere::new(0.3).mesh().uv(32, 18))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: RED.into(),
                    emissive: LinearRgba::new(1.0, 0.0, 0.0, 0.6),
                    emissive_exposure_weight: 1.0,
                    ..default()
                })),
            ));
        });

    commands.insert_resource(AmbientLight {
        color: Color::srgb(1.0, 1.0, 1.0),
        brightness: 1_000.1,
    });
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, WIDTH as f32, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn moving_light(time: Res<Time>, mut query: Query<(&mut Transform, &PointLight)>) {
    for (mut transform, _) in query.iter_mut() {
        transform.translation.x = 15.0 * time.elapsed_secs().sin() as f32 + 8.0;
        transform.translation.z = 15.0 * time.elapsed_secs().cos() as f32 + 8.0;
    }
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

#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub voxel: [usize; 3],
    pub width: u32,
    pub height: u32,
}

#[derive(Default)]
pub struct QuadGroups {
    pub groups: [Vec<Quad>; 6],
}

impl QuadGroups {
    const LEFT: usize = 0;
    const RIGHT: usize = 1;
    const BOTTOM: usize = 2;
    const TOP: usize = 3;
    const FRONT: usize = 4;
    const BACK: usize = 5;
}

impl From<QuadGroups> for Mesh {
    fn from(value: QuadGroups) -> Self {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();

        // Top
        value.groups[QuadGroups::TOP].iter().for_each(|quad| {
            let [x, y, z] = quad.voxel;
            let width = quad.width as f32;
            let height = quad.height as f32;

            let start = vertices.len() as u32;
            vertices.extend_from_slice(&vec![
                [x as f32, y as f32 + 1.0, z as f32],
                [x as f32 + width, y as f32 + 1.0, z as f32],
                [x as f32 + width, y as f32 + 1.0, z as f32 + height],
                [x as f32, y as f32 + 1.0, z as f32 + height],
            ]);
            indices.extend_from_slice(&[
                start, start + 2, start + 1, 
                start, start + 3, start + 2
            ]);
            uvs.extend_from_slice(&vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
            normals.extend_from_slice(&vec![[0.0, 1.0, 0.0]; 4]);
        });

        // LEFT
        value.groups[QuadGroups::LEFT].iter().for_each(|quad| {
            let [x, y, z] = quad.voxel;
            let width = quad.width as f32;
            let height = quad.height as f32;

            let start = vertices.len() as u32;
            vertices.extend_from_slice(&vec![
                [x as f32, y as f32, z as f32],
                [x as f32, y as f32 + height, z as f32],
                [x as f32, y as f32 + height, z as f32 + width],
                [x as f32, y as f32, z as f32 + width ],
            ]);
            indices.extend_from_slice(&[
                start, start + 2, start + 1, 
                start, start + 3, start + 2
            ]);
            uvs.extend_from_slice(&vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
            normals.extend_from_slice(&vec![[-1.0, 0.0, 0.0]; 4]);
        });

        // RIGHT
        value.groups[QuadGroups::RIGHT].iter().for_each(|quad| {
            let [x, y, z] = quad.voxel;
            let width = quad.width as f32;
            let height = quad.height as f32;

            let start = vertices.len() as u32;
            vertices.extend_from_slice(&vec![
                [x as f32 + 1.0, y as f32, z as f32],
                [x as f32 + 1.0, y as f32, z as f32 + width],
                [x as f32 + 1.0, y as f32 + height, z as f32 + width],
                [x as f32 + 1.0, y as f32 + height, z as f32],
            ]);
            indices.extend_from_slice(&[
                start, start + 2, start + 1, 
                start, start + 3, start + 2
            ]);
            uvs.extend_from_slice(&vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
            normals.extend_from_slice(&vec![[1.0, 0.0, 0.0]; 4]);
        });

        // FRONT
        value.groups[QuadGroups::FRONT].iter().for_each(|quad| {
            let [x, y, z] = quad.voxel;
            let width = quad.width as f32;
            let height = quad.height as f32;

            let start = vertices.len() as u32;
            vertices.extend_from_slice(&vec![
                [x as f32, y as f32, z as f32],
                [x as f32 + width, y as f32, z as f32],
                [x as f32 + width, y as f32 + height, z as f32],
                [x as f32, y as f32 + height, z as f32],
            ]);
            indices.extend_from_slice(&[
                start, start + 2, start + 1, 
                start, start + 3, start + 2
            ]);
            uvs.extend_from_slice(&vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
            normals.extend_from_slice(&vec![[0.0, 0.0, -1.0]; 4]);
        });

        // BACK
        value.groups[QuadGroups::BACK].iter().for_each(|quad| {
            let [x, y, z] = quad.voxel;
            let width = quad.width as f32;
            let height = quad.height as f32;

            let start = vertices.len() as u32;
            vertices.extend_from_slice(&vec![
                [x as f32, y as f32, z as f32 + 1.0],
                [x as f32, y as f32 + height, z as f32 + 1.0],
                [x as f32 + width, y as f32 + height, z as f32 + 1.0],
                [x as f32 + width, y as f32, z as f32 + 1.0],
            ]);
            indices.extend_from_slice(&[
                start, start + 2, start + 1, 
                start, start + 3, start + 2
            ]);
            uvs.extend_from_slice(&vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
            normals.extend_from_slice(&vec![[0.0, 0.0, 1.0]; 4]);
        });

        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_indices(Indices::U32(indices));
        mesh
    }
}

pub fn generate_quads(chunk: &Chunk) -> QuadGroups {
    let mut buffer = QuadGroups::default();

    for x in 1..Chunk::length() - 1 {
        for y in 1..Chunk::length() - 1 {
            for z in 1..Chunk::length() - 1 {
                let voxel = chunk.get(x, y, z);

                let neighbors = [
                    chunk.get(x - 1, y, z),
                    chunk.get(x + 1, y, z),
                    chunk.get(x, y - 1, z),
                    chunk.get(x, y + 1, z),
                    chunk.get(x, y, z - 1),
                    chunk.get(x, y, z + 1),
                ];

                for (i, neighbor) in neighbors.iter().enumerate() {
                    if *neighbor != voxel {
                        buffer.groups[i].push(Quad {
                            voxel: [x, y, z],
                            width: 1,
                            height: 1,
                        });
                    }
                }
            }
        }
    }

    buffer
}
