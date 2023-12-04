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

// Function to initialize all resources for this file
pub fn draw_setup(mut commands: Commands) {
        commands.insert_resource(DrawingInfo { 
        counter: 0, // Counter used to count up... ie for SVG file names, if multiple drawn
        last_pos: None, // Latest drawn SVG path known in ongoing drawing
    });
        commands.insert_resource(DrawingPoints { 
        line_entities: Vec::new(), // Entities made to hold display SVG while getting drawn
        points: Vec::new(), // Points making up SVG
    });
        commands.insert_resource(FollowConfig { 
        speed: 1.5, // Speed of SVG path drawing while following cursor
    });
}

// Main drawing function that is called from main
pub fn drawing(
    mut _commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
    mut drawing_info: ResMut<DrawingInfo>,
    mut drawing_points: ResMut<DrawingPoints>,
    mouse_button_input: Res<Input<MouseButton>>,
    cameras: Query<(&Camera, &Transform)>,
    config: Res<FollowConfig>,
) {
    let mut window = windows.single_mut(); // Getting window
    let left_pressed = mouse_button_input.pressed(MouseButton::Left);
    let left_released = mouse_button_input.just_released(MouseButton::Left);
    let points_not_empty = !drawing_points.points.is_empty(); // Check if points vector is not empty
    
    // Handles fn mouse_press if condition met
    if left_pressed {
        mouse_press(&mut window, &cameras, &mut drawing_info, &mut drawing_points, &config);
    } 
    // Handles fn save_and_draw SVG if condition met
    else if left_released {
        if points_not_empty {
            replace_svg(_commands, asset_server, &mut drawing_info, &mut drawing_points);
        }
    }
}

// Function to update drawing points
fn update_drawing_points(
    drawing_info: &mut ResMut<DrawingInfo>, 
    drawing_points: &mut ResMut<DrawingPoints>, 
    new_pos: Vec2
) {
    if drawing_info.last_pos.is_none() { // If last_pos in drawing_info is None, initialize with new_pos
        drawing_info.last_pos = Some(new_pos);
    }
    drawing_points.points.push(new_pos); // Add new_pos to points vector in drawing_points
    drawing_info.last_pos = Some(new_pos); // Update last_pos in drawing_info with new_pos
}

// Function that handles if mouse button is pressed
fn mouse_press(
    window: &mut Window,
    cameras: &Query<(&Camera, &Transform)>,
    drawing_info: &mut ResMut<DrawingInfo>,
    drawing_points: &mut ResMut<DrawingPoints>,
    config: &Res<FollowConfig>,
) {
    let world_pos = world_mouse_position(window, cameras); // Gets world position of cursor
    let new_pos = compute_new_pos(drawing_info, world_pos, config.speed); // Computes updating position of mouse
    update_drawing_points(drawing_info, drawing_points, new_pos); // Updates drawing points with new positions
}

// Function to get world position of cursor
fn world_mouse_position(
    window: &mut Window, 
    cameras: &Query<(&Camera, &Transform)>
) -> Vec2 {
    let (_camera, camera_transform) = cameras.single(); // Get camera and its transform
    let pos = window.cursor_position().unwrap_or_default(); // Get current cursor position
    let size = Vec2::new(window.width() as f32, window.height() as f32); // Get size of window
    let adjusted_point = Vec2::new(pos.x, size.y - pos.y) - size / 2.0; // Adjust cursor position relative to center of window
    let world_pos = camera_transform.compute_matrix() * adjusted_point.extend(0.0).extend(1.0); // Converts adjusted_point to world_pos
    Vec2::new(world_pos.x, world_pos.y) // Returns world_pos
}

// Function to determin if SVG path needs to continue to follow cursor
fn compute_new_pos(
    drawing_info: &ResMut<DrawingInfo>, 
    world_pos: Vec2, //displaying SVG
    speed: f32,
) -> Vec2 {
    drawing_info.last_pos.map_or(world_pos, |last_pos| { // If there is a last_pos, compute new_pos, otherwise return world_pos
        let direction = world_pos - last_pos; // Compute direction of drawing
        if direction.length() > 0.0 {
            last_pos + direction.normalize() * speed // If direction is not zero, draws in direction of cursor
        } else {
            last_pos // Otherwise cursor and SVG path are at same points, no need to move path to cursor
        }
    })
}

// Function to draw SVG from file after user has drawn it
fn draw_svg(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    filename: PathBuf,
    svg_pos: Vec2,
) {
    let filename_str = filename.strip_prefix("assets/").unwrap().to_owned(); // Get filename string
    let svg = asset_server.load(filename_str); // Loads SVG file
    let transform = Transform::from_translation(Vec3::new(svg_pos.x, svg_pos.y, 0.0)); // Create a transform for SVG 
    commands.spawn(Svg2dBundle { // Spawn an SVG bundle with help from bevy_svg crate
        svg,
        origin: Origin::Center,
        transform,
        ..Default::default()
    });
}

// Function to save drawing after mouse button release
fn save_svg(
    drawing_points: &ResMut<DrawingPoints>,
    drawing_info: &ResMut<DrawingInfo>,
) -> (PathBuf, f32, f32, f32, f32) {
    // Computes min & max of x, y coordinates of drawing points
    let min_x = drawing_points.points.iter().map(|p| p.x).fold(f32::INFINITY, f32::min);
    let min_y = drawing_points.points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min);
    let max_x = drawing_points.points.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max);
    let max_y = drawing_points.points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max);
    
    let filename = format!("assets/svgs/drawing{}.svg", drawing_info.counter); // Creates filename for SVG
    let path = std::path::Path::new(&filename); // Create a path object from filename
    
    if !path.exists() {
        fs::create_dir_all("assets/svgs").unwrap(); // If directory path does not exist one is made
        let _filename = format!("=assets/svgs/drawing{}.svg", drawing_info.counter);
    }
    
    let mut file = File::create(&path).unwrap(); // Create SVG file
    let _ = write!(file, "<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}'>", max_x - min_x, max_y - min_y); // Writes SVG header
    let _ = write!(file, "<path d='M "); // Starts SVG path
    for point in &drawing_points.points {
        let _ = write!(file, "{} {} L ", point.x - min_x, max_y - point.y); // Writes all points to SVG path
    }
    let _ = write!(file, "' fill='none' stroke='black'/>"); // Ends SVG path tag
    let _ = write!(file, "</svg>"); // Ends SVG tag
    (path.to_path_buf(), min_x, min_y, max_x, max_y) // Returns path and bounding box of  drawing
}

// Replaces temporary entities with SVG drawn by user
fn replace_svg(
    mut _commands: Commands, 
    asset_server: Res<AssetServer>, 
    drawing_info: &mut ResMut<DrawingInfo>, 
    drawing_points: &mut ResMut<DrawingPoints>
) {
    let (filename, min_x, min_y, max_x, max_y) = save_svg(drawing_points, drawing_info); // Saves SVG as file from save_svg
    let center = Vec2::new((min_x + max_x) / 2.0, (min_y + max_y) / 2.0); // Computes center of  drawing so Origin::Center is lined up for spawning
    draw_svg(_commands, asset_server, filename, center); // Draws SVG
    
    drawing_info.last_pos = None; // Resets last position for next drawing
    drawing_points.points.clear(); // Clears drawing points
    drawing_info.counter += 1; // Increments counter to be able to create a seperate SVG
}

// Function used to draw temporary entities to show user what they are drawing
pub fn draw_lines(
    mut commands: Commands,
    mut drawing_points: ResMut<DrawingPoints>,
) {
    let points = drawing_points.points.clone(); // Clone points from drawing_points resource
    for entity in drawing_points.line_entities.drain(..) {
        commands.entity(entity).despawn(); // Iterate over line_entities in drawing_points and despawn each entity, done to save performance, try with this line commented out!
    }
    // Iterate over each pair of points
    for points in points.windows(2) {
        let line = shapes::Line(points[0], points[1]); // Create a line shape from the pair of points
        // Spawn a new entity with shape being a line with, stroke, and fill with color
        let entity = commands.spawn((ShapeBundle {
            path: GeometryBuilder::build_as(&line), // Builds line
            ..default()
        }, Stroke::new(Color::rgb(1.0, 1.0, 1.0), 2.0))).id(); // Gives shape white color and a width of 2.0
        drawing_points.line_entities.push(entity); // Pushs the entity to the line_entities in drawing_points
    }
}
