use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::window::PresentMode;
use bevy_editor_pls::prelude::*;
use smooth_bevy_cameras::{
    controllers::unreal::{UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.,
                height: 600.,
                title: "Bevy game".to_string(),
                canvas: Some("#bevy".to_owned()),
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(MaterialPlugin::<GlowyMaterial>::default())
        .add_plugin(EditorPlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(UnrealCameraPlugin::default())
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<GlowyMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let texture = asset_server.load("texture.jpg");

    let material = materials.add(GlowyMaterial {
        texture: texture.clone(),
    });

    let mesh = meshes.add(Mesh::from(shape::UVSphere {
        radius: 1.0,
        ..default()
    }));
    commands.spawn((
        Name::new("Orb"),
        TransformBundle::default(),
        material,
        mesh,
        VisibilityBundle::default(),
    ));

    commands
        .spawn((Name::new("Camera"), Camera3dBundle::default()))
        .insert(UnrealCameraBundle::new(
            UnrealCameraController::default(),
            Vec3::new(0.0, 3.0, 0.0),
            Vec3::new(0., 0., 0.),
        ));
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "bd5c76fd-6fdd-4de4-9744-4e8beea8daaf"]
pub struct GlowyMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

impl Material for GlowyMaterial {
    fn fragment_shader() -> ShaderRef {
        "glowy.wgsl".into()
    }
}
