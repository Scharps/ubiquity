use bevy::{
    asset::RenderAssetUsages, color::palettes::css::RED, diagnostic::LogDiagnosticsPlugin, prelude::*, render::mesh::{Indices, MeshVertexAttribute, PrimitiveTopology}
};
use map_gen::{biome::FbmDescriptor, BlockType, Chunky, ChunkPopulate};

fn main() {
    let mut app = bevy::prelude::App::new();
    app.add_plugins(bevy::DefaultPlugins);
    app.add_plugins(LogDiagnosticsPlugin::default());
    app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
    app.add_systems(Startup, startup);
    app.add_systems(Update, moving_light);
    app.run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const WIDTH: usize = 16;
    let mut chunk = map_gen::TestChunk::new();


    let descriptor = FbmDescriptor {
        octaves: 4,
        frequency: 0.2,
        lacunarity: 2.0,
        persistence: 0.5,
    };

    let generator = map_gen::biome::LandGenerator::new(0, descriptor);

    generator.populate(&mut chunk);
    let mesh: Mesh = generate_quads(&chunk).into();

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        })),
    ));
    // for (i, _) in chunk.voxels().iter().enumerate().filter(|(_, block)| **block != BlockType::Air) {
    //     let (x, y, z) = TestChunk::delinearize(i);
    //     commands.spawn((
    //         Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //         MeshMaterial3d(materials.add(Color::srgb_u8(235, 239, 145))),
    //         Transform::from_xyz(x as f32, y as f32, z as f32),
    //     ));
    // }
    commands
        .spawn((
            PointLight {
                intensity: 1000_000.0,
                color: RED.into(),
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(64.0, 4.0, 64.0),
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
        color: Color::WHITE,
        brightness: 100.1,
    });
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(20.0, WIDTH as f32, 20.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn moving_light(time: Res<Time>, mut query: Query<(&mut Transform, &PointLight)>) {
    for (mut transform, _) in query.iter_mut() {
        transform.translation.x = 15.0 * time.elapsed_secs().sin() as f32 + 8.0;
        transform.translation.z = 15.0 * time.elapsed_secs().cos() as f32 + 8.0;
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
    pub groups: [Vec<Quad>; 6]
}

impl From<QuadGroups> for Mesh {
    fn from(value: QuadGroups) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip, RenderAssetUsages::default());
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        // Top 
        value.groups[3].iter().for_each(|quad| {
            let [x, y, z] = quad.voxel;
            let width = quad.width as f32;
            let height = quad.height as f32;

            let start = vertices.len() as u32;
            vertices.push([x as f32, y as f32, z as f32]);
            vertices.push([x as f32 + width, y as f32, z as f32]);
            vertices.push([x as f32 + width, y as f32, z as f32 + height]);
            vertices.push([x as f32, y as f32, z as f32 + height]);

            indices.extend_from_slice(&[start, start + 1, start + 2, start, start + 2, start + 3]);
        });

        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; vertices.len()]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_indices(Indices::U32(indices));
        mesh
    }
}

pub fn generate_quads<C>(chunk: &C) -> QuadGroups
where
    C: Chunky<BlockType>,
{
    let mut buffer = QuadGroups::default();

    for i in 0..C::size() {
        let (x, y, z) = C::delinearize(i);

        if (x > 0 && x < C::X - 1) &&
        (y > 0 && y < C::Y - 1) &&
        (z > 0 && z < C::Z - 1) {
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

    buffer
}