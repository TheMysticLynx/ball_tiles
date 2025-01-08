use bevy::prelude::*;

const TRIANGLE_SIZE: f32 = 5f32;
const BOID_COUNT: usize = 100;

fn spawn_boids(projection: Query<&OrthographicProjection>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
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