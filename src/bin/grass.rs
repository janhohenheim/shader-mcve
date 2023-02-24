use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use shader_playground::generator::standard_generator::Plane;
use shader_playground::generator::GrassGenerator;
use shader_playground::generator::StandardGeneratorConfig;
use shader_playground::plugin::GrassPlugin;
use shader_playground::GrassBundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.,
                height: 600.,
                title: "Shader Playground".to_string(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            },
            ..default()
        }))
        .add_plugin(GrassPlugin)
        .add_plugin(EditorPlugin)
        .add_startup_system(setup_grass)
        .run();
}
// In this example 2 planes are used for generating grass blades
fn setup_grass(mut commands: Commands) {
    let config = StandardGeneratorConfig {
        density: 10.,
        height: 3.,
        height_deviation: 0.5,
        seed: Some(0x121),
    };
    // translation indicates the outer point
    let plane1 = Plane {
        dimensions: Transform::from_xyz(30., 0., 10.),
    };
    let plane2 = Plane {
        dimensions: Transform::from_xyz(10., 2., -10.),
    };

    let mut grass = plane1.generate_grass(config.clone());
    grass
        .instances
        .extend(plane2.generate_grass(config).instances);
    commands.spawn((GrassBundle { grass, ..default() },));

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 8.5, 0.0)
            .looking_at(Vec3::new(4.0, 5.0, 0.0), Vec3::Y),
        ..default()
    });
}
