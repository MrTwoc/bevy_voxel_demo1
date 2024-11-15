use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin}, prelude::*, render::{mesh::{Indices, PrimitiveTopology}, render_asset::RenderAssetUsages}
};
use bevy_flycam::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const CHUNK_WEIGHT: i32 = 8;
const CHUNK_HEIGHT: i32 = 2;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextStyle {
                        // Here we define size of our overlay
                        font_size: 50.0,
                        // We can also change color of the overlay
                        color: Color::srgb(0.0, 1.0, 0.0),
                        // If we want, we can use a custom font
                        font: default(),
                    },
                },
            },
            PlayerPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cube_mesh = meshes.add(create_cube_mesh());
    let custom_texture_handle: Handle<Image> = asset_server.load("array_texture.png");

    // 顶点数量：Chunk_Weight * Chunk_Height * 8
    println!("顶点数量: {:?}",&create_cube_mesh().count_vertices());

    commands.spawn(PbrBundle {
        mesh: cube_mesh,
        material: materials.add(StandardMaterial {
            base_color_texture: Some(custom_texture_handle.clone()),
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, 1.5, -2.0)),
        ..Default::default()
    });
}

fn create_cube_mesh() -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    for x in 0..CHUNK_WEIGHT {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_WEIGHT {
                // 可以从这里判断当前坐标的方块是否需要绘制
                let pos = [x as f32, y as f32, z as f32];
                add_cube_to_mesh(&mut positions, &mut normals, &mut uvs, &mut indices, pos);
            }
        }
    }

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

#[rustfmt::skip]
fn add_cube_to_mesh(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
    pos: [f32; 3],
) {
    let start_index = positions.len() as u32;

    /*
        TODO:
        各种优化剔除：
        遮挡剔除、视锥剔除、LOD技术、八叉树等
        一、遮挡剔除：
        判断坐标的方块周围是否被其他方块遮挡，如果被遮挡从pos中删除顶点
        fn is_cube_occluded(pos: [f32; 3]) -> bool {}
        二、视锥剔除：
        判断坐标的方块是否在视锥内，如果不在视锥内从pos中删除顶点(没有方块遮挡与空气接触，但不在视锥内的方块)
        fn is_cube_in_view(pos: [f32; 3]) -> bool {}
        三、LOD技术：
        借鉴我的世界中《遥远的地平线》模组
     */
    // 顶点位置
    positions.extend_from_slice(&[
        [pos[0], pos[1] + 1.0, pos[2]], // 0
        [pos[0] + 1.0, pos[1] + 1.0, pos[2]], // 1
        [pos[0] + 1.0, pos[1] + 1.0, pos[2] + 1.0], // 2
        [pos[0], pos[1] + 1.0, pos[2] + 1.0], // 3
        [pos[0], pos[1], pos[2]], // 4
        [pos[0] + 1.0, pos[1], pos[2]], // 5
        [pos[0] + 1.0, pos[1], pos[2] + 1.0], // 6
        [pos[0], pos[1], pos[2] + 1.0], // 7
    ]);

    // 法线
    normals.extend_from_slice(&[
        [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], // 顶面
        [0.0, -1.0, 0.0], [0.0, -1.0, 0.0], [0.0, -1.0, 0.0], [0.0, -1.0, 0.0], // 底面
        [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], // 右侧面
        [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], // 左侧面
        [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], // 背面
        [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], // 前面
    ]);

    // UV 坐标
    uvs.extend_from_slice(&[
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0], // 顶面
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0], // 底面
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0], // 右侧面
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0], // 左侧面
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0], // 背面
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0], // 前面
    ]);

    // 索引
    /*
        这个条件判断检查当前方块是否位于立方体的边缘或角落。
        pos[0] == 0.0 或 pos[0] == CHUNK_WEIGHT as f32：检查方块是否位于 x 轴的最小或最大位置。
        pos[1] == 0.0 或 pos[1] == CHUNK_HEIGHT as f32：检查方块是否位于 y 轴的最小或最大位置。
        pos[2] == 0.0 或 pos[2] == CHUNK_WEIGHT as f32：检查方块是否位于 z 轴的最小或最大位置。
     */
    if pos[0] >= 0.0 || pos[0] <= CHUNK_WEIGHT as f32 || pos[1] >= 0.0 || pos[1] <= CHUNK_HEIGHT as f32 || pos[2] >= 0.0 || pos[2] <= CHUNK_WEIGHT as f32 {
        indices.extend_from_slice(&[
            start_index + 0, start_index + 3, start_index + 1, start_index + 1, start_index + 3, start_index + 2, // 顶面
            start_index + 4, start_index + 5, start_index + 7, start_index + 5, start_index + 6, start_index + 7, // 底面
            start_index + 1, start_index + 2, start_index + 5, start_index + 5, start_index + 2, start_index + 6, // 右侧面
            start_index + 0, start_index + 4, start_index + 3, start_index + 3, start_index + 4, start_index + 7, // 左侧面
            start_index + 2, start_index + 3, start_index + 6, start_index + 6, start_index + 3, start_index + 7, // 背面
            start_index + 0, start_index + 1, start_index + 4, start_index + 4, start_index + 1, start_index + 5, // 前面
        ]);
    }
}
