use bevy::prelude::*;
use bevy_svg::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::{
    fs::{self, File},
    io::prelude::*,
    path::PathBuf,
};

#[derive(Resource, Default)]
pub struct DrawingPoints {
    points: Vec<Vec2>,
    line_entities: Vec<Entity>,
}

#[derive(Resource, Default)]
pub struct DrawingInfo {
    counter: usize,
    last_pos: Option<Vec2>,
}

#[derive(Resource, Default)]
pub struct FollowConfig {
    speed: f32,
}

pub fn draw_setup(mut commands: Commands) {
        commands.insert_resource(DrawingInfo { 
        counter: 0,
        last_pos: None,
    });
        commands.insert_resource(DrawingPoints { 
        line_entities: Vec::new(),
        points: Vec::new(),
    });
        commands.insert_resource(FollowConfig { 
        speed: 1.5,
    });
}

fn mouse_press(
    window: &mut Window,
    cameras: &Query<(&Camera, &Transform)>,
    drawing_info: &mut ResMut<DrawingInfo>,
    drawing_points: &mut ResMut<DrawingPoints>,
    config: &Res<FollowConfig>,
) {
    let world_pos = world_mouse_position(window, cameras);
    let new_pos = compute_new_pos(drawing_info, world_pos, config.speed);
    update_drawing_points(drawing_info, drawing_points, new_pos);
}

pub fn drawing(
    mut _commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
    mut drawing_info: ResMut<DrawingInfo>,
    mut drawing_points: ResMut<DrawingPoints>,
    mouse_button_input: ResMut<'_, ButtonInput<MouseButton>>,
    cameras: Query<(&Camera, &Transform)>,
    config: Res<FollowConfig>,
) {
    let mut window = windows.single_mut();
    let left_pressed = mouse_button_input.pressed(MouseButton::Left);
    let left_released = mouse_button_input.just_released(MouseButton::Left);
    let points_not_empty = !drawing_points.points.is_empty();
    
    if left_pressed {
        mouse_press(&mut window, &cameras, &mut drawing_info, &mut drawing_points, &config);
    } 
    else if left_released {
        if points_not_empty {
            replace_svg(_commands, asset_server, &mut drawing_info, &mut drawing_points);
        }
    }
}

fn update_drawing_points(
    drawing_info: &mut ResMut<DrawingInfo>, 
    drawing_points: &mut ResMut<DrawingPoints>, 
    new_pos: Vec2
) {
    if drawing_info.last_pos.is_none() {
        drawing_info.last_pos = Some(new_pos);
    }
    drawing_points.points.push(new_pos);
    drawing_info.last_pos = Some(new_pos);
}

fn world_mouse_position(
    window: &mut Window, 
    cameras: &Query<(&Camera, &Transform)>
) -> Vec2 {
    let (_camera, camera_transform) = cameras.single();
    let pos = window.cursor_position().unwrap_or_default();
    let size = Vec2::new(window.width() as f32, window.height() as f32);
    let adjusted_point = Vec2::new(pos.x, size.y - pos.y) - size / 2.0; // Adjust cursor position relative to center of window
    let world_pos = camera_transform.compute_matrix() * adjusted_point.extend(0.0).extend(1.0); // Converts adjusted_point to world_pos
    Vec2::new(world_pos.x, world_pos.y) 
}

fn compute_new_pos(
    drawing_info: &ResMut<DrawingInfo>, 
    world_pos: Vec2,
    speed: f32,
) -> Vec2 {
    drawing_info.last_pos.map_or(world_pos, |last_pos| {
        let direction = world_pos - last_pos;
        if direction.length() > 0.0 {
            last_pos + direction.normalize() * speed
        } else {
            last_pos
        }
    })
}

fn draw_svg(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    filename: PathBuf,
    svg_pos: Vec2,
) {
    let filename_str = filename.strip_prefix("assets/").unwrap().to_owned();
    let svg = asset_server.load(filename_str);
    let transform = Transform::from_translation(Vec3::new(svg_pos.x, svg_pos.y, 0.0));
    let origin = Origin::Center;
    commands.spawn((
        Svg2d(svg),
        transform,
        origin,
    ));
}

fn save_svg(
    drawing_points: &ResMut<DrawingPoints>,
    drawing_info: &ResMut<DrawingInfo>,
) -> (PathBuf, f32, f32, f32, f32) {
    // Computes min & max of x, y coordinates of drawing points to make width & height of SVG as small as it needs to be
    let min_x = drawing_points.points.iter().map(|p| p.x).fold(f32::INFINITY, f32::min);
    let min_y = drawing_points.points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min);
    let max_x = drawing_points.points.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max);
    let max_y = drawing_points.points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max);
    
    let filename = format!("assets/svgs/drawing{}.svg", drawing_info.counter);
    let path = std::path::Path::new(&filename);
    
    if !path.exists() {
        fs::create_dir_all("assets/svgs").unwrap(); 
        let _filename = format!("=assets/svgs/drawing{}.svg", drawing_info.counter);
    }
    
    let mut file = File::create(&path).unwrap();
    let _ = write!(file, "<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}'>", max_x - min_x, max_y - min_y);
    let _ = write!(file, "<path d='M ");
    for point in &drawing_points.points {
        let _ = write!(file, "{} {} L ", point.x - min_x, max_y - point.y);
    }
    let _ = write!(file, "' fill='none' stroke='black'/>");
    let _ = write!(file, "</svg>");
    (path.to_path_buf(), min_x, min_y, max_x, max_y)
}

fn replace_svg(
    mut _commands: Commands, 
    asset_server: Res<AssetServer>, 
    drawing_info: &mut ResMut<DrawingInfo>, 
    drawing_points: &mut ResMut<DrawingPoints>
) {
    let (filename, min_x, min_y, max_x, max_y) = save_svg(drawing_points, drawing_info);
    let center = Vec2::new((min_x + max_x) / 2.0, (min_y + max_y) / 2.0); // Computes center of  drawing so Origin::Center is lined up for spawning where SVG was drawn
    draw_svg(_commands, asset_server, filename, center); 
    
    drawing_info.last_pos = None;
    drawing_points.points.clear(); 
    drawing_info.counter += 1; 
}

// Draws line shown to user while SVG data is still being collected
pub fn draw_lines(
    mut commands: Commands,
    mut drawing_points: ResMut<DrawingPoints>,
) {
    let points = drawing_points.points.clone();
    for entity in drawing_points.line_entities.drain(..) {
        commands.entity(entity).despawn(); // Done to save performance, try with this line commented out!
    }
    for points in points.windows(2) {
        let line = shapes::Line(points[0], points[1]); 
        let line_color = Srgba::rgb(1.0, 1.0, 1.0);
        let line_width = 2.0;
        let entity = commands.spawn((ShapeBundle {
            path: GeometryBuilder::build_as(&line),
            ..default()
        }, Stroke::new(line_color, line_width))).id();
        drawing_points.line_entities.push(entity);
    }
}
