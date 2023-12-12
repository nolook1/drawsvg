use bevy::{prelude::*, window::PresentMode};

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, settings_setup)
            .add_systems(Update, (toggle_vsync, camera_movement_system));
    }
}

#[derive(Resource, Default)]
pub struct DrawingConfig {
    translation_speed: f32,
}

fn settings_setup(mut commands: Commands) {
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