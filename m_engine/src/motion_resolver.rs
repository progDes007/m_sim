use crate::collision_utils;
use crate::prelude::*;
use crate::{Particle, ParticleClass, Vec2};
use ordered_float;
use std::cmp::{Ord, PartialOrd, Reverse};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::ops::Range;
use std::vec::Vec;

/// Represents a collision between 2 particles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Collision {
    particle1: usize,
    particle2: usize,
    // The time must be this weird type to enable sorting
    time: ordered_float::OrderedFloat<f64>,
}

impl Collision {
    pub fn involves_particle(&self, particle_index: usize) -> bool {
        self.particle1 == particle_index || self.particle2 == particle_index
    }
}

impl Ord for Collision {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.partial_cmp(&other.time).unwrap()
    }
}

impl PartialOrd for Collision {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Finds all collisions between a particle and a range of particles
/// The range may contain particle itself, in which case it's ignored.
/// Some particles already have time advanced for them. If collision happens
/// in the "past" it's ignored
fn find_collisions_multi(
    main_index: usize,
    other_indices: Range<usize>,
    particles: &[Particle],
    class_map: &HashMap<ClassId, ParticleClass>,
    particle_times: &[f64],
    time_threshold: f64,
) -> Vec<Collision> {
    let mut collisions = vec![];
    for i in other_indices {
        // Skip collisions agains itself
        if i == main_index {
            continue;
        }
        let p1 = &particles[main_index];
        let p2 = &particles[i];
        // Both particles live at different time step. We need to bring them to the same time 0.
        let pos1 = p1.position - p1.velocity * particle_times[main_index];
        let pos2 = p2.position - p2.velocity * particle_times[i];

        let collision_time = collision_utils::find_circle_vs_circle_collision(
            pos1,
            class_map.get(&p1.class()).unwrap().radius(),
            p1.velocity,
            pos2,
            class_map.get(&p2.class()).unwrap().radius(),
            p2.velocity,
        );
        if let Some(collision_time) = collision_time {
            // Check if the collision is in the future. But not too far in the future
            if collision_time > particle_times[main_index]
                && collision_time > particle_times[i]
                && collision_time < time_threshold
            {
                collisions.push(Collision {
                    particle1: main_index,
                    particle2: i,
                    time: ordered_float::OrderedFloat(collision_time),
                });
            }
        }
    }
    return collisions;
}

pub(crate) fn resolve(
    particles: &mut [Particle],
    class_map: &HashMap<ClassId, ParticleClass>,
    timestep: f64,
    velocity_resolver: impl Fn(&Particle, &Particle) -> (Vec2, Vec2),
) {
    // For each particle we shall track the time we already simulated
    let mut particle_time: Vec<f64> = vec![0.0; particles.len()];
    let mut current_collisions = BinaryHeap::new();
    // Lamda for merging incoming collisions into the heap
    let merge = |left: &mut BinaryHeap<Reverse<Collision>>, right: &[Collision]| {
        for collision in right {
            left.push(Reverse(collision.clone())); // sorted
        }
    };

    // Generate collisions for each vs each
    for i in 0..particles.len() - 1 {
        merge(
            &mut current_collisions,
            &find_collisions_multi(
                i,
                i + 1..particles.len(),
                particles,
                class_map,
                &particle_time,
                timestep,
            ),
        );
    }

    // Keep resolving collisions while there are any
    while let Some(Reverse(collision)) = current_collisions.pop() {
        // Advance both particles forward to the moment of collision
        let time_to_collision = collision.time.0;
        let mut particle1 = particles[collision.particle1];
        particle1.position +=
            particle1.velocity * (time_to_collision - particle_time[collision.particle1]);
        let mut particle2 = particles[collision.particle2];
        particle2.position +=
            particle2.velocity * (time_to_collision - particle_time[collision.particle2]);
        // Resolve new velocities
        let (new_velocity1, new_velocity2) = velocity_resolver(&particle1, &particle2);
        particle1.velocity = new_velocity1;
        particle2.velocity = new_velocity2;
        // Assign new particle state back
        particles[collision.particle1] = particle1;
        particles[collision.particle2] = particle2;
        // Track the particle time
        particle_time[collision.particle1] = time_to_collision;
        particle_time[collision.particle2] = time_to_collision;

        // Delete all other collisions that involved these 2 particles.
        // They become invalid because particles velocity was altered
        current_collisions.retain(|Reverse(c)| {
            !c.involves_particle(collision.particle1) && !c.involves_particle(collision.particle2)
        });
        // Now we need to calculate all new collisions with particle1 and particle2
        merge(
            &mut current_collisions,
            &find_collisions_multi(
                collision.particle1,
                0..particles.len(),
                particles,
                class_map,
                &particle_time,
                timestep,
            ),
        );
        merge(
            &mut current_collisions,
            &find_collisions_multi(
                collision.particle2,
                0..particles.len(),
                particles,
                class_map,
                &particle_time,
                timestep,
            ),
        );
    }
    // When there are no more collisions left - just advance all particles to the end
    for i in 0..particles.len() {
        particles[i].position += particles[i].velocity * (timestep - particle_time[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math_core;

    // Test the ordered binary heap of collisions
    #[test]
    fn test_collision_order() {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(Collision {
            particle1: 0,
            particle2: 1,
            time: ordered_float::OrderedFloat(0.0),
        }));
        heap.push(Reverse(Collision {
            particle1: 2,
            particle2: 3,
            time: ordered_float::OrderedFloat(1.0),
        }));
        heap.push(Reverse(Collision {
            particle1: 3,
            particle2: 4,
            time: ordered_float::OrderedFloat(0.5),
        }));
        assert_eq!(heap.pop().unwrap().0.time, ordered_float::OrderedFloat(0.0));
        assert_eq!(heap.pop().unwrap().0.time, ordered_float::OrderedFloat(0.5));
        assert_eq!(heap.pop().unwrap().0.time, ordered_float::OrderedFloat(1.0));
    }

    #[test]
    fn test_find_collisions_multi() {
        let mut classes = HashMap::new();
        classes.insert(1, ParticleClass::new("Class1", 1.0, 1.0));
        classes.insert(2, ParticleClass::new("Class2", 2.0, 2.0));

        // Summary: 5 particles. All moving to the right.

        let particles = vec![
            Particle::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), 1),
            Particle::new(Vec2::new(10.0, 0.0), Vec2::new(2.0, 0.0), 2),
            Particle::new(Vec2::new(20.0, 0.0), Vec2::new(1.0, 0.0), 1),
            Particle::new(Vec2::new(31.0, 0.0), Vec2::new(1.0, 0.0), 1),
            Particle::new(Vec2::new(40.0, 0.0), Vec2::new(1.0, 0.0), 1),
        ];

        let test_and_assert =
            |time_threshold: f64, time_to_first: f64, time_to_second: f64, times: Vec<f64>| {
                let collisions = find_collisions_multi(
                    1,
                    0..particles.len(),
                    &particles,
                    &classes,
                    &times,
                    time_threshold, // no enough to catch up to last
                );
                assert_eq!(collisions.len(), 2);
                assert_eq!(collisions[0].particle1, 1);
                assert_eq!(collisions[0].particle2, 2);
                assert!(math_core::approx_eq(
                    collisions[0].time.0,
                    time_to_first,
                    DOUBLE_COMPARE_EPS_STRICT
                ));
                assert_eq!(collisions[1].particle1, 1);
                assert_eq!(collisions[1].particle2, 3);
                assert!(math_core::approx_eq(
                    collisions[1].time.0,
                    time_to_second,
                    DOUBLE_COMPARE_EPS_STRICT
                ));
            };

        // Vanila case (in terms of time shifts). There is enough to time to catch up
        // with 2 of 3 particles in front
        let mut times = vec![0.0, 0.0, 0.0, 0.0, 0.0];
        let mut time_threshold = 25.0;
        let mut time_to_first = 7.0;
        let mut time_to_second = 18.0;
        test_and_assert(time_threshold, time_to_first, time_to_second, times.clone());

        // Now simulate all particles being 1 second in front in time
        for t in times.iter_mut() {
            *t = *t + 1.0;
        }
        // The time to reach the same point (as measured from 0) is also increased
        time_threshold += 1.0;
        time_to_first += 1.0;
        time_to_second += 1.0;
        test_and_assert(time_threshold, time_to_first, time_to_second, times.clone());

        // Now simulate the particle #1 is actually far into the future.
        // Such that in the past it actual did hit particle #0
        times[1] += 10.0;
        // The time to reach the same collision point is increased
        // The distance at t=0 is 20 units behind (because time is 10 and speed is 2).
        // The catchup speed is 1 unit per second.
        time_threshold += 20.0;
        time_to_first += 20.0;
        time_to_second += 20.0;
        // Note that we still only have 2 collisions because collisions in the "past" don't count
        test_and_assert(time_threshold, time_to_first, time_to_second, times.clone());
    }

    #[test]
    pub fn test_resolve() {
        
        // Add couple of classes. All particles have the same mass.
        // Combined with coefficient of restitution of 1 - it will lead to simpler numbers
        // and easier to analyze code.
        let mut classes = HashMap::new();
        classes.insert(1, ParticleClass::new("Class1", 1.0, 1.0));
        classes.insert(2, ParticleClass::new("Class2", 1.0, 2.0));
        // Lamda that resolve velocity
        let resolve_velocity = |p1: &Particle, p2: &Particle| {
            let velocities = collision_utils::collision_separation_velocity(
                p1.position, p1.velocity, classes.get(&p1.class()).unwrap().mass(), 
                p2.position, p2.velocity, classes.get(&p2.class()).unwrap().mass(), 
                1.0);
            return velocities.unwrap();
        };
        // Add particles
        let mut particles = vec![
            Particle::new(Vec2::new(10.0, 0.0), Vec2::new(0.0, 0.0), 1),
            Particle::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), 2),
            Particle::new(Vec2::new(14.0, 0.0), Vec2::new(0.0, 0.0), 1), 
            Particle::new(Vec2::new(14.0, -15.0), Vec2::new(0.0, 1.0), 1), 
            Particle::new(Vec2::new(-8.0, 5.0), Vec2::new(1.0, 0.0), 1), 
            Particle::new(Vec2::new(20.0, 20.0), Vec2::new(1.0, 1.0), 2), 
            Particle::new(Vec2::new(12.0, 12.0), Vec2::new(0.0, -1.0), 1), 
        ];

        // Story line outline:
        // 1. #1 will hit #0 at 7sec. #1 will stop. #0 will continue at velocity 1.0
        // 2. #0 will hit #2 at 9sec. #0 will stop. #2 will continue at velocity 1.0
        // 3. #3 WOULD have hit #2. But because #2 was hit before it will miss and continue travel
        // 4. #4 will hit #3 from the side at t=20
        // 5. #5 hits nobody
        // 6. #6 WOULD NOT hit #0. But because #0 stopped at 12.0 due to hitting other. 
        // It will be hit by #6 at t=10

        // Resolve
        resolve(&mut particles, &classes, 30.0, resolve_velocity);

        // Check the result
        assert!(particles[0].position.approx_eq(Vec2::new(10.0 + 2.0, 0.0 - 20.0), DISTANCE_EPS));
        assert!(particles[0].velocity.approx_eq(Vec2::new(0.0, -1.0), DISTANCE_EPS));

        assert!(particles[1].position.approx_eq(Vec2::new(7.0, 0.0), DISTANCE_EPS));
        assert!(particles[1].velocity.approx_eq(Vec2::new(0.0, 0.0), DISTANCE_EPS));

        assert!(particles[2].position.approx_eq(Vec2::new(14.0 + 21.0, 0.0), DISTANCE_EPS));
        assert!(particles[2].velocity.approx_eq(Vec2::new(1.0, 0.0), DISTANCE_EPS));

        assert!(particles[3].position.approx_eq(Vec2::new(14.0 + 10.0, -15.0 + 30.0), DISTANCE_EPS));
        assert!(particles[3].velocity.approx_eq(Vec2::new(1.0, 1.0), DISTANCE_EPS));

        assert!(particles[4].position.approx_eq(Vec2::new(-8.0 + 20.0, 5.0), DISTANCE_EPS));
        assert!(particles[4].velocity.approx_eq(Vec2::new(0.0, 0.0), DISTANCE_EPS));

        assert!(particles[5].position.approx_eq(Vec2::new(20.0 + 30.0, 20.0 + 30.0), DISTANCE_EPS));
        assert!(particles[5].velocity.approx_eq(Vec2::new(1.0, 1.0), DISTANCE_EPS));

        assert!(particles[6].position.approx_eq(Vec2::new(12.0, 2.0), DISTANCE_EPS));
        assert!(particles[6].velocity.approx_eq(Vec2::new(0.0, 0.0), DISTANCE_EPS));

    }
}
