use core::f32;

use crate::boid_store::{BoidStore, BoidWrapper};
use bevy::prelude::*;

const TRIANGLE_SIZE: f32 = 5f32;
const BOID_COUNT: usize = 2000;

#[derive(Component)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Boid;

#[derive(Component)]
pub struct Tracked;

#[derive(Default, Resource, PartialEq, Eq)]
pub enum BoidInitState {
    #[default]
    Uninitialized,
    Initialized,
}

pub fn spawn_boids(
    projection: Query<&OrthographicProjection>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut init_res: ResMut<BoidInitState>,
) {
    if *init_res == BoidInitState::Initialized {
        return;
    }

    *init_res = BoidInitState::Initialized;

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
        let x = rand::random::<f32>() * 500f32 - 250f32;
        let y = rand::random::<f32>() * 500f32 - 250f32;

        commands.spawn((
            Boid,
            Mesh2d(mesh.clone()),
            MeshMaterial2d(materials.add(Color::linear_rgb(0f32, 0f32, 1f32))),
            Velocity(Vec2::new(0f32, 50f32)),
            Transform::from_translation(Vec3::new(x, y, 0f32)),
        ));
    }
}

pub fn track_boids(
    query: Query<(Entity, &Transform, &Velocity, &Boid)>,
    mut boid_store: ResMut<BoidStore>,
) {
    for (entity, transform, direction, _) in query.iter() {
        println!(
            "Entity: {:?}, Position: {:?}, Direction: {:?}",
            entity, transform.translation, direction.0
        );
        boid_store.add_boid(entity, transform.translation.truncate(), direction.0);
    }
}

pub fn update_stored_info(
    changed_dir: Query<
        (Entity, &Velocity, &Transform),
        (Or<(Changed<Velocity>, Changed<Transform>)>),
    >,
    mut store: ResMut<BoidStore>,
) {
    for (entity, direction, transform) in changed_dir.iter() {
        store.update_boid(entity, transform.translation.truncate(), direction.0);
    }
}

#[derive(Resource)]
pub struct Factors {
    pub wall_avoidance_factor: f32,
    pub wall_avoidance_distance: f32,
    pub flock_distance: f32,
    pub avoidance_factor: f32,
    pub avoidance_distance: f32,
    pub align_factor: f32,
    pub cohere_factor: f32,
    pub max_speed: f32,
    pub min_speed: f32,
}

impl Default for Factors {
    fn default() -> Self {
        Self {
            wall_avoidance_factor: 0.5f32,
            wall_avoidance_distance: 100f32,
            flock_distance: 50f32,
            avoidance_factor: 1f32,
            avoidance_distance: 20f32,
            align_factor: 0.1f32,
            cohere_factor: 0.1f32,
            max_speed: 500f32,
            min_speed: 200f32,
        }
    }
}

pub fn run_physics(
    camera: Query<&OrthographicProjection>,
    mut directions: Query<(&mut Velocity, &mut Transform, Entity), (With<Boid>)>,
    time: Res<Time>,
    mut boids: ResMut<BoidStore>,
    factors: Res<Factors>,
) {
    let area = camera.single().area;
    let max = area.max;
    let min = area.min;

    directions.par_iter_mut().for_each(|(mut direction, transform, entity)| {
        let mut wall_dirs: Vec2 = Vec2::default();
        let loop_boid_pos: Vec2 = transform.translation.truncate();

        let dist = loop_boid_pos.x - min.x;
        if dist < factors.wall_avoidance_distance {
            wall_dirs += Vec2::new(1f32, 0f32)
                * (factors.wall_avoidance_distance - dist)
                * factors.wall_avoidance_factor;
        }

        let dist = max.x - loop_boid_pos.x;
        if dist < factors.wall_avoidance_distance {
            wall_dirs += Vec2::new(-1f32, 0f32)
                * (factors.wall_avoidance_distance - dist)
                * factors.wall_avoidance_factor;
        }

        let dist = loop_boid_pos.y - min.y;
        if dist < factors.wall_avoidance_distance {
            wall_dirs += Vec2::new(0f32, 1f32)
                * (factors.wall_avoidance_distance - dist)
                * factors.wall_avoidance_factor;
        }

        let dist = max.y - loop_boid_pos.y;
        if dist < factors.wall_avoidance_distance {
            wall_dirs += Vec2::new(0f32, -1f32)
                * (factors.wall_avoidance_distance - dist)
                * factors.wall_avoidance_factor;
        }
        direction.0 += wall_dirs;

        let flock = boids.get_boids(transform.translation.truncate());
        // let flock: Vec<_> = flock
        //     .iter()
        //     .filter(|b| {
        //         b.entity != entity && (b.position - loop_boid_pos).length() < factors.flock_distance
        //     })
        //     .collect();

        if flock.len() > 0 {
            let mut avoid_vec = Vec2::default();
            for boid in flock.iter() {
                if (boid.position - loop_boid_pos).length() < factors.avoidance_distance {
                    avoid_vec += loop_boid_pos - boid.position;
                }
            }

            direction.0 += avoid_vec * factors.avoidance_factor;

            let align_vec = (flock
                .iter()
                .fold(Vec2::default(), |acc, boid| acc + boid.velocity)
                / flock.len() as f32)
                - direction.0;
            direction.0 += align_vec * factors.align_factor;

            let cohere_vec = (flock
                .iter()
                .fold(Vec2::default(), |acc, boid| acc + boid.position)
                / flock.len() as f32)
                - loop_boid_pos;
            direction.0 += cohere_vec * factors.cohere_factor;
        }

        if direction.0.length() > factors.max_speed {
            direction.0 = direction.0.normalize() * factors.max_speed;
        } else if direction.0.length() < factors.min_speed {
            if direction.0.length() < 0.001f32 {
                direction.0 = Vec2::new(1f32, 0f32);
            }
            direction.0 = direction.0.normalize() * factors.min_speed;
        }
    });

    for (direction, transform, entity) in directions.iter() {
        boids.update_boid(entity, transform.translation.truncate(), direction.0);
    }
}

pub fn average(input: Vec<Vec2>) -> Vec2 {
    let mut sum = Vec2::new(0f32, 0f32);
    for i in input.iter() {
        sum += i;
    }
    sum / input.len() as f32
}

pub fn handle_rotation_and_movement(
    mut boids: Query<(&mut Transform, &Velocity), (With<Boid>)>,
    time: Res<Time>,
) {
    for (mut tranform, velocity) in boids.iter_mut() {
        println!("frame time: {}", 1f32 / time.delta_secs());

        let angle = velocity.0.to_angle();
        tranform.rotation = Quat::from_rotation_z(angle);

        let x_movement = velocity.0.x * time.delta_secs();
        let y_movement = velocity.0.y * time.delta_secs();
        let vec = Vec3::new(x_movement, y_movement, 0f32);
        tranform.translation += vec;
    }
}
