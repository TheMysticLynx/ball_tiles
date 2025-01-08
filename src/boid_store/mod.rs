use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
pub struct BoidWrapper {
    pub entity: Entity,
    pub position: Vec2,
    pub velocity: Vec2,
}

#[derive(Resource)]
pub struct BoidStore {
    boids: HashMap<Entity, BoidWrapper>,
    positional_store: HashMap<(i32, i32), HashSet<Entity>>,
    size: f32,
}

impl Default for BoidStore {
    fn default() -> Self {
        Self::new()
    }
}

impl BoidStore {
    pub fn new() -> Self {
        BoidStore {
            boids: HashMap::default(),
            positional_store: HashMap::default(),
            size: 35.0f32,
        }
    }

    pub fn add_boid(&mut self, entity: Entity, position: Vec2, velocity: Vec2) {
        self.boids.insert(
            entity.clone(),
            BoidWrapper {
                entity,
                position,
                velocity,
            },
        );

        let (x, y) = self.get_x_y(position);
        self.positional_store
            .entry((x, y))
            .or_insert(HashSet::new())
            .insert(entity);
    }

    pub fn update_boid(&mut self, entity: Entity, position: Vec2, velocity: Vec2) {
        if let Some(boid) = self.boids.get(&entity) {
            let (old_x, old_y) = self.get_x_y(boid.position);
            if let Some(set) = self.positional_store.get_mut(&(old_x, old_y)) {
                set.remove(&entity);
            }
        }

        if let Some(boid) = self.boids.get_mut(&entity) {
            boid.position = position;
            boid.velocity = velocity;
        }

        let (x, y) = self.get_x_y(position);
        self.positional_store
            .entry((x, y))
            .or_insert(HashSet::new())
            .insert(entity);
    }

    pub fn get_boids(&self, pos: Vec2) -> Vec<&BoidWrapper> {
        let (x, y) = self.get_x_y(pos);

        let mut ents: HashSet<Entity> = HashSet::new();
        for w in -1..=1 {
            for h in -1..=1 {
                if let Some(set) = self.positional_store.get(&(x + w, y + h)) {
                    for e in set.iter() {
                        ents.insert(e.clone());
                    }
                }
            }
        }

        let mut result = Vec::new();
        for e in ents.iter() {
            if let Some(boid) = self.boids.get(e) {
                result.push(boid);
            }
        }

        result
    }

    fn get_x_y(&self, position: Vec2) -> (i32, i32) {
        let x = (position.x / self.size).floor();
        let y = (position.y / self.size).floor();
        (x as i32, y as i32)
    }
}
