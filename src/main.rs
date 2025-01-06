use core::f32;
use std::time::Duration;

use bevy::prelude::*;
use bevy_framepace::Limiter;

const MAX_ROTATION: f32 = f32::consts::PI * 2f32;
const BOID_COUNT: u32 = 50;
const TRIANGLE_SIZE: f32 = 5f32;
const BOID_SPEED: f32 = 100f32;
const FLOCK_DIST: f32 = 100f32;
const WANTED_DIST: f32 = 25f32;
const BORDER_SEPERATION_DISTANCE: f32 = 50f32;

#[derive(Resource, Default)]
enum GraphicsInitilized {
    True,
    #[default]
    False
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, setup_graphics)
        .add_systems(Update, setup_physics.run_if(should_run_setup_graphics).after(setup_graphics))
        .add_systems(Update, run_physics)
        .add_systems(Update, handle_rotation_and_movement)
        .add_systems(Startup, framepace)
        .init_resource::<GraphicsInitilized>()
        .run();
}

fn should_run_setup_graphics(graphics_initilized: Res<GraphicsInitilized>, time: Res<Time>) -> bool {
    if time.elapsed_secs() < 0.1f32 {
        return false
    }

    match *graphics_initilized {
        GraphicsInitilized::True => false,
        GraphicsInitilized::False => true,
    }
}
 
fn setup_physics (projection: Query<&OrthographicProjection>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, mut graphics_initilized: ResMut<GraphicsInitilized>) {
    *graphics_initilized = GraphicsInitilized::True;
    
    let area = projection.single().area;
    let min = area.min;
    let width = area.width();
    let height = area.height();

    let mesh = meshes.add(Triangle2d::new(
        Vec2::new(TRIANGLE_SIZE, 0f32),
        Vec2::new(-TRIANGLE_SIZE, TRIANGLE_SIZE),
        Vec2::new(-TRIANGLE_SIZE, -TRIANGLE_SIZE),
    ));

    for i in 0..BOID_COUNT {
        let x = rand::random::<f32>() * width + min.x;
        let y = rand::random::<f32>() * height + min.y;
        
        commands.spawn((
            Boid,
            Mesh2d(mesh.clone()),
            MeshMaterial2d(materials.add(Color::linear_rgb(0f32, 0f32, 1f32))),
            Direction(f32::consts::PI * rand::random::<f32>()),
            Transform::from_translation(Vec3::new(x, y, 0f32))
        ));
    }
}

#[derive(Component)]
struct Boid;

#[derive(Component)]
struct Direction(f32);

fn handle_rotation_and_movement(
    mut boids: Query<(&mut Transform, &Direction), (With<Boid>)>,
    time: Res<Time>,
) {
    for (mut tranform, direction) in boids.iter_mut() {
        println!("frame time: {}", 1f32 / time.delta_secs());

        let angle = tranform.rotation - Quat::from_euler(EulerRot::XYZ, 0f32, 0f32, direction.0);
        let dist = angle.to_euler(EulerRot::XYZ).2.abs();
        println!("{}", dist);
        let max = MAX_ROTATION * time.delta_secs();
        println!("max {}", max);
        let rot = if dist.abs() < max.abs() {
            dist
        } else {
            dist.signum() * max
        };

        println!("rot {}", rot);

        let z = tranform.rotation.to_euler(EulerRot::XYZ).2;
        tranform.rotation = Quat::from_euler(EulerRot::XYZ, 0f32, 0f32, z + rot);
    }
}

fn setup_graphics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut projection: Query<&OrthographicProjection>,
) {
    commands.spawn(Camera2d);
}

fn run_physics(
    camera: Query<&OrthographicProjection>,
    mut directions: Query<(&mut Direction, &mut Transform), (With<Boid>)>,
    time: Res<Time>,
) {
    let area = camera.single().area;
    let max = area.max;
    let min = area.min;

    for (mut direction, transform) in directions.iter_mut() {
        let mut output_directions_with_part: Vec<(f32, u32)> = Vec::new();
        let pos: Vec2 = transform.translation.truncate();
        if pos.x - min.x < 50f32 {
            println!("too close to left wall");
            output_directions_with_part.push((0f32, 1));
        }

        if max.x - pos.x < 50f32 {
            println!("too close to right wall");
            output_directions_with_part.push((f32::consts::PI, 1));
        }

        if pos.y - min.y < 50f32 {
            println!("too close to top wall");
            output_directions_with_part.push((f32::consts::PI / 2f32, 1));
        }

        if max.y - pos.y < 50f32 {
            println!("too close to bottom wall");
            output_directions_with_part.push((-f32::consts::PI / 2f32, 1));
        }

        if output_directions_with_part.len() > 0 {
            let output_direction = output_directions_with_part.first().unwrap();
            let mut dir = output_direction.0;
            let mut parts = output_direction.1 as f32;

            for (out_dir, out_parts) in output_directions_with_part.iter().skip(1) {
                let lerp = *out_parts as f32 / parts;
                dir = out_dir.lerp(*out_dir, lerp);
                parts += *out_parts as f32;
            }

            direction.0 = dir;
        }
    }

    // move
    for (_direction, mut transform) in directions.iter_mut() {
        let transform_right = transform.right();
        transform.translation += transform_right * BOID_SPEED * time.delta_secs();
    }
}

fn framepace(mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    settings.limiter = Limiter::Manual(Duration::from_secs_f32(1f32 / 120f32))
}
