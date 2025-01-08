use bevy::prelude::*;
pub struct BoidWrapper {
    entity: Entity,
    position: Vec2,
    velocity: Vec2,
}

#[derive(Resource)]
pub struct BoidStore {
    boids: Vec<BoidWrapper>
}

impl BoidStore {
    pub fn new() -> Self {
        BoidStore {
            boids: Vec::new()
        }
    }

    pub fn add_boid(&mut self, entity: Entity, position: Vec2, velocity: Vec2) {
        self.boids.push(BoidWrapper {
            entity,
            position,
            velocity
        });
    }

    pub fn get_boids(&self) -> &Vec<BoidWrapper> {
        &self.boids
    }
}