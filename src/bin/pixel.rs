//! Shows how to render to a texture. Useful for mirrors, UI, or exporting images.

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};
use bevy_editor_pls::EditorPlugin;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};
use std::f32::consts::PI;

fn main() {
    App::new()
        // Needed for pixelation not looking blurry
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        // Needed for pixelation not looking blurry
        .insert_resource(Msaa { samples: 1 })
        .add_plugin(EditorPlugin)
        .add_startup_system(setup)
        .add_system(rotate_canvas)
        .add_system(sync_cameras)
        .run();
}

// Marks the first pass cube (rendered to a texture.)
#[derive(Component)]
struct FirstPassCube;

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct MainPassCube;

#[derive(Component)]
struct InnerCamera;

#[derive(Component)]
struct OuterCamera;

const CUBE_SIZE: f32 = 4.0;
const INNER_CAMERA_DISTANCE: f32 = CUBE_SIZE * 3.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 128,
        height: 128,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: CUBE_SIZE }));
    let inner_cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        reflectance: 0.02,
        ..default()
    });

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(1);

    // The cube that will be rendered to the texture.
    commands.spawn((
        Name::new("Inner object"),
        PbrBundle {
            mesh: cube_handle.clone(),
            material: inner_cube_material_handle,
            ..default()
        },
        FirstPassCube,
        first_pass_layer,
    ));

    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 10.0)),
        ..default()
    });

    commands.spawn((
        Name::new("Inner camera"),
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::NONE),
                ..default()
            },
            camera: Camera {
                // render before the "main pass" camera
                priority: -1,
                target: RenderTarget::Image(image_handle.clone()),

                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        InnerCamera,
        first_pass_layer,
    ));

    // This material has the texture that has been rendered.
    let plane_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let plane_handle = meshes.add(Mesh::from(shape::Plane {
        size: INNER_CAMERA_DISTANCE,
    }));

    // Main pass cube, with material containing the rendered first pass texture.
    commands
        .spawn((
            Name::new("Outer object"),
            MainPassCube,
            SpatialBundle::from_transform(Transform::from_xyz(0.0, CUBE_SIZE / 2., 0.0)),
        ))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: plane_handle,
                material: plane_material_handle,
                transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 2.0)),
                ..default()
            });
        });

    let shadow_cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::NONE,
        alpha_mode: AlphaMode::Mask(1.),
        ..default()
    });
    // The shadow of the cube
    commands.spawn((
        Name::new("Shadow object"),
        PbrBundle {
            mesh: cube_handle.clone(),
            material: shadow_cube_material_handle,
            ..default()
        },
    ));

    // The main pass camera.
    commands
        .spawn((
            Name::new("Outer camera"),
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            OuterCamera,
        ))
        .insert(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(0.0, 0.0, 15.0),
            Vec3::ZERO,
            Vec3::Y,
        ));

    // Ground
    commands.spawn((
        Name::new("Ground"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
            material: materials.add(Color::WHITE.into()),
            ..default()
        },
    ));
}

/// Rotates the inner cube (first pass)
fn sync_cameras(
    mut inner_camera_query: Query<&mut Transform, (Without<OuterCamera>, With<InnerCamera>)>,
    outer_camera_query: Query<&Transform, (With<OuterCamera>, Without<MainPassCube>)>,
) {
    for mut inner_camera_transform in &mut inner_camera_query {
        for outer_camera_transform in outer_camera_query.iter() {
            inner_camera_transform.translation =
                outer_camera_transform.translation.normalize() * INNER_CAMERA_DISTANCE;
            inner_camera_transform.look_at(Vec3::ZERO, Vec3::Y);
        }
    }
}

/// Rotates the outer cube (main pass)
fn rotate_canvas(
    mut cube_query: Query<&mut Transform, (Without<OuterCamera>, With<MainPassCube>)>,
    camera_query: Query<&Transform, (With<OuterCamera>, Without<MainPassCube>)>,
) {
    for mut cube_transform in &mut cube_query {
        for camera_transform in camera_query.iter() {
            cube_transform.look_at(camera_transform.translation, Vec3::Y);
        }
    }
}
