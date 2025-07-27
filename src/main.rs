use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod draw;
use draw::{drawing, draw_lines, draw_setup};

mod settings;
use crate::settings::SettingsPlugin;

mod fps_counter;
use crate::fps_counter::FpsPlugin;

fn main() {
    App::new()
        //.insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SVGdraw".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((bevy_svg::prelude::SvgPlugin, ShapePlugin, SettingsPlugin, FpsPlugin))
        .add_systems(Startup, (draw_setup, main_setup))
        .add_systems(FixedUpdate, (drawing, draw_lines))
        .run();
}

fn main_setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
