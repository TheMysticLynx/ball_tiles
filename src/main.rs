use core::f32;
use std::time::Duration;

use crate::boid_store::BoidStore;
use bevy::prelude::*;
use bevy_framepace::Limiter;
use bevy_mod_imgui::ImguiContext;
use boids::{
    handle_rotation_and_movement, run_physics, spawn_boids, track_boids, update_stored_info,
    BoidInitState, Factors,
};

mod boid_store;
mod boids;

fn main() {
    App::new()
        .add_systems(
            Startup,
            (
                spawn_boids,
                setup_camera.before(spawn_boids),
                track_boids.after(spawn_boids),
            ),
        )
        .add_systems(
            Update,
            (
                handle_rotation_and_movement,
                run_physics,
                update_stored_info,
                imgui_example_ui,
            ),
        )
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_mod_imgui::ImguiPlugin::default())
        .init_resource::<BoidStore>()
        .init_resource::<BoidInitState>()
        .init_resource::<Factors>()
        .run();
}

fn setup_camera(mut commads: Commands) {
    commads.spawn(Camera2d::default());
}

fn imgui_example_ui(mut context: NonSendMut<ImguiContext>, mut factors: ResMut<Factors>, time: Res<Time>) {
    let mut window = context.ui();

    // frame time
    window.text(format!("Frame Time: {:.2}", time.delta().as_secs_f32()));
    window.text(format!("FPS: {:.2}", 1.0 / time.delta().as_secs_f32()));

    // inputs
    window
        .input_float("Wall Avoidance Factor", &mut factors.wall_avoidance_factor)
        .build();
    window.slider(
        "WallAvoidance Factor",
        0.001f32,
        2f32,
        &mut factors.wall_avoidance_factor,
    );

    window
        .input_float(
            "Wall Avoidance Distance",
            &mut factors.wall_avoidance_distance,
        )
        .build();
    window.slider(
        "Wall Avoidance Distance",
        0.001f32,
        1000f32,
        &mut factors.wall_avoidance_distance,
    );
    window
        .input_float("Flock Distance", &mut factors.flock_distance)
        .build();
    window.slider(
        "Flock Distance",
        0.001f32,
        1000f32,
        &mut factors.flock_distance,
    );
    window
        .input_float("Avoidance Factor", &mut factors.avoidance_factor)
        .build();
    window.slider(
        "Avoidance Factor",
        0.001f32,
        2f32,
        &mut factors.avoidance_factor,
    );
    window
        .input_float("Avoidance Distance", &mut factors.avoidance_distance)
        .build();
    window.slider(
        "Avoidance Distance",
        0.001f32,
        1000f32,
        &mut factors.avoidance_distance,
    );
    window
        .input_float("Max Speed", &mut factors.max_speed)
        .build();
    window.slider("Max Speed", 0.001f32, 1000f32, &mut factors.max_speed);
    window
        .input_float("Min Speed", &mut factors.min_speed)
        .build();
    window.slider("Min Speed", 0.001f32, 1000f32, &mut factors.min_speed);
    window
        .input_float("Align Factor", &mut factors.align_factor)
        .build();
    window.slider("Align Factor", 0.001f32, 2f32, &mut factors.align_factor);
    window
        .input_float("Cohere Factor", &mut factors.cohere_factor)
        .build();
    window.slider("Cohere Factor", 0.001f32, 2f32, &mut factors.cohere_factor);
}
