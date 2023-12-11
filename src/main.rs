use bevy::{prelude::*, window::PresentMode};
use bevy_prototype_lyon::prelude::*;

mod draw;
use draw::{drawing, draw_lines, draw_setup};

mod fps_counter;
use crate::fps_counter::FpsCounterPlugin;
use fps_counter::fps_setup;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SVGdraw".to_string(),
                present_mode: PresentMode::AutoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((bevy_svg::prelude::SvgPlugin, ShapePlugin, FpsCounterPlugin))
        .add_systems(Startup, (draw_setup, main_setup, fps_setup))
        .add_systems(Update, toggle_vsync)
        .add_systems(FixedUpdate, (drawing, draw_lines, camera_movement_system))
        .run();
}

#[derive(Resource, Default)]
pub struct DrawingConfig {
    translation_speed: f32,
}

fn main_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(DrawingConfig { 
        translation_speed: 250.0,
    });
}

fn toggle_vsync(input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    if input.just_pressed(KeyCode::V) {
        let mut window = windows.single_mut();

        window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        };
        info!("PRESENT_MODE: {:?}", window.present_mode);
    }
}

fn camera_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
    time: Res<Time>,
    drawingconfig: Res<DrawingConfig>,
) {
    for (_camera, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        
        let translation_speed = drawingconfig.translation_speed;
        
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
        transform.translation += time.delta_seconds() * translation_speed * direction;
    }
}
