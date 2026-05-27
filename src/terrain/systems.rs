use bevy::{
    asset::RenderAssetUsages,
    mesh::Indices,
    prelude::*,
    render::render_resource::PrimitiveTopology,
};
use noise::{NoiseFn, Perlin};

use super::components::{TerrainChunk, TerrainConfig};

fn generate_terrain_mesh(config: &TerrainConfig) -> Mesh {
    let perlin = Perlin::new(1);
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    for z in 0..=config.size {
        for x in 0..=config.size {
            let px = x as f32;
            let pz = z as f32;

            let noise_val = perlin.get([px as f64 * config.scale, pz as f64 * config.scale]);
            let py = noise_val as f32 * config.height_multiplier;

            positions.push([px, py, pz]);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([px / config.size as f32, pz / config.size as f32]);
        }
    }

    let mut indices = Vec::new();
    for z in 0..config.size {
        for x in 0..config.size {
            let bottom_left = z * (config.size + 1) + x;
            let bottom_right = bottom_left + 1;
            let top_left = (z + 1) * (config.size + 1) + x;
            let top_right = top_left + 1;

            indices.push(bottom_left);
            indices.push(top_left);
            indices.push(bottom_right);

            indices.push(bottom_right);
            indices.push(top_left);
            indices.push(top_right);
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh.compute_smooth_normals();
    mesh
}

pub fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<TerrainConfig>, // Inject the resource here
) {
    let terrain_mesh = generate_terrain_mesh(&config);
    let half_size = (config.size as f32) / 2.0;

    commands.spawn((
        TerrainChunk, // Attach our marker component
        Mesh3d(meshes.add(terrain_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.2, 0.5, 0.2)))),
        // Center the chunk around 0,0,0 based on its configured size
        Transform::from_xyz(-half_size, 0.0, -half_size),
    ));
}