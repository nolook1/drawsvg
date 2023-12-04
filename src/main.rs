use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_fps_counter::FpsCounterPlugin;

// Includes draw.rs file for use
mod draw;
use draw::{drawing, draw_lines, draw_setup};

// Function main where application is initialized and run
fn main() {
    // Create a new Bevy application
    App::new()
        .insert_resource(Msaa::Sample4) // Insert a resource for multi-sample anti-aliasing with 4 samples
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SVGdraw".to_string(), // Set window title to whatever!
                ..Default::default()
            }),
            ..Default::default()
        }))
        // Adds SVG, Shape, FpsCounter plugins and systems, then runs the application
        .add_plugins(bevy_svg::prelude::SvgPlugin)
        .add_plugins(ShapePlugin)
        .add_plugins(FpsCounterPlugin)
        .add_systems(Startup, draw_setup)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (drawing, draw_lines, camera_movement_system),)
        .run();
}

// A resource for storing the drawing configuration
#[derive(Resource, Default)]
pub struct DrawingConfig {
    translation_speed: f32,
}

// Setup function to spawn a 2D camera and insert a DrawingConfig resource
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()); // Spawns 2D camera
    commands.insert_resource(DrawingConfig { 
        translation_speed: 250.0, // Sets speed of camera movement, WASD controls
    });
}

// A system for moving the camera based on keyboard input
fn camera_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
    time: Res<Time>,
    drawingconfig: Res<DrawingConfig>,
) {
    // Iterate over each camera and its transform
    for (_camera, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO; // Initialize the direction vector
        
        // Get the translation speed from the drawing configuration
        let translation_speed = drawingconfig.translation_speed;
        
        // Update the direction vector based on the pressed keys
        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        // Update the translation of the transform based on the direction, speed, and delta time
        transform.translation += time.delta_seconds() * translation_speed * direction;
    }
}
