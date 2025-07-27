use bevy::{prelude::*};

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, settings_setup)
            .add_systems(Update, camera_movement_system);
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

fn camera_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
    time: Res<Time>,
    drawingconfig: Res<DrawingConfig>,
) {
    for (_camera, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let translation_speed = drawingconfig.translation_speed;
        
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        transform.translation += time.delta_secs() * translation_speed * direction;
    }
}
