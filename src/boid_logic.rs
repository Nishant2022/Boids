use bevy::{prelude::Component, math::Vec2};

use crate::BoidSettings;

// region:      Boid Struct

#[derive(Debug, Component, Clone)]
pub struct Boid {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub id: u32,
}

impl Boid {

    pub fn update(&mut self, boids: &Vec<Boid>, settings: &BoidSettings) {
        
        // Update velocity using boid rules
        self.cohesion(boids, settings);
        self.separation(boids, settings);
        self.alignment(boids, settings);
        
        // Normaize speed so it is constant
        self.normalize_speed(settings);

        // Update the position of the boid
        self.update_position();
    }

    fn update_position(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    fn cohesion(&mut self,  boids: &Vec<Boid>, settings: &BoidSettings) {
        // Boids try to stay close to each other

        let mut center_x: f32 = 0.0;
        let mut center_y: f32 = 0.0;

        let mut num_boids: u32 = 0;

        for boid in boids {
            if boid.id != self.id && self.distance_sqrd(boid) < settings.boid_view_distance_sqrd {
                center_x += boid.x;
                center_y += boid.y;
                num_boids += 1;
            }
        }

        if num_boids != 0 {
            center_x /= num_boids as f32;
            center_y /= num_boids as f32;

            self.dx += (center_x - self.x) * settings.cohesion_multiplier;
            self.dy += (center_y - self.y) * settings.cohesion_multiplier;
        }
    }

    fn separation(&mut self, boids: &Vec<Boid>, settings: &BoidSettings) {
        // Boids do not want to be too close to each other
        
        let mut move_x: f32 = 0.0;
        let mut move_y: f32 = 0.0;

        for boid in boids {
            if boid.id != self.id && self.distance_sqrd(boid) < settings.boid_view_distance_sqrd {
                move_x += self.x - boid.x;
                move_y += self.y - boid.y;
            }
        }

        self.dx += move_x * settings.separation_multiplier;
        self.dy += move_y * settings.separation_multiplier;
    }

    fn alignment(&mut self, boids: &Vec<Boid>, settings: &BoidSettings) {
        // Boids try to align themselves with other boids

        let mut avg_dx: f32 = 0.0;
        let mut avg_dy: f32 = 0.0;
        let mut num_boids: u32 = 0;

        for boid in boids {
            if boid.id != self.id && self.distance_sqrd(boid) < settings.boid_view_distance_sqrd {
                avg_dx += boid.dx;
                avg_dy += boid.dy;
                num_boids += 1;
            }
        }

        if num_boids != 0 {
            self.dx += (avg_dx - self.dx) * settings.alignment_multiplier;
            self.dy += (avg_dy - self.dy) * settings.alignment_multiplier;
        }

    }

    fn normalize_speed(&mut self, settings: &BoidSettings) {
        // Normalizes speed to be constant

        let speed_vec =  Vec2::new(self.dx, self.dy);
        let speed_vec = speed_vec.normalize() * settings.boid_max_speed;

        self.dx = speed_vec.x;
        self.dy = speed_vec.y;
    }

    fn distance_sqrd(&self, other: &Boid)  -> f32 {
        return (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y);
    }
}

// endregion:   Boid Struct