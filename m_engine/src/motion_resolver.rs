use crate::collision_utils;
use crate::prelude::*;
use crate::{Particle, ParticleClass, Vec2, Wall, WallClass};
use ordered_float;
use std::cmp::{Ord, PartialOrd, Reverse};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::ops::Range;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OtherObject {
    Particle(usize),
    Wall(usize),
}

/// Represents a collision between 2 objects
#[derive(Debug, Clone, Copy)]
struct Collision {
    particle: usize,
    other: OtherObject,
    normal: Vec2,
    // The time must be this weird type to enable sorting
    time: ordered_float::OrderedFloat<f64>,
}

impl Collision {
    pub fn involves_particle(&self, particle_index: usize) -> bool {
        (self.particle == particle_index)
            || (matches!(self.other, OtherObject::Particle(i) if i == particle_index))
    }
}

impl PartialEq for Collision {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.particle == other.particle && self.other == other.other
    }
}

impl Eq for Collision {}

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
fn find_collisions_with_particles(
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

        let collision_time = collision_utils::find_particle_vs_particle_collision(
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
                let normal = collision_utils::particles_collision_normal(
                    pos1 + p1.velocity * collision_time,
                    p1.velocity,
                    pos2 + p2.velocity * collision_time,
                    p2.velocity,
                );
                // There is a chance that there is no normal if scene was setup with
                // overlaps. In this case we just ignore the collision
                if let Some(normal) = normal {
                    collisions.push(Collision {
                        particle: main_index,
                        other: OtherObject::Particle(i),
                        normal,
                        time: ordered_float::OrderedFloat(collision_time),
                    });
                }
            }
        }
    }
    return collisions;
}

pub(crate) fn resolve(
    particles: &mut [Particle],
    particle_class_map: &HashMap<ClassId, ParticleClass>,
    _walls: &[Wall],
    _walls_map: &HashMap<ClassId, WallClass>,
    timestep: f64,
    particle_vs_particle_velocity_resolver: impl Fn(&Particle, &Particle, Vec2) -> (Vec2, Vec2),
    _particle_vs_wall_velocity_resolver: impl Fn(&Particle, &Wall, Vec2) -> Vec2,
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
            &find_collisions_with_particles(
                i,
                i + 1..particles.len(),
                particles,
                particle_class_map,
                &particle_time,
                timestep,
            ),
        );
    }

    // Keep resolving collisions while there are any
    while let Some(Reverse(collision)) = current_collisions.pop() {
        if let OtherObject::Particle(particle2_idx) = collision.other
        {
            // Advance both particles forward to the moment of collision
            let time_to_collision = collision.time.0;
            let mut particle1 = particles[collision.particle];
            particle1.position +=
                particle1.velocity * (time_to_collision - particle_time[collision.particle]);
            let mut particle2 = particles[particle2_idx];
            particle2.position +=
                particle2.velocity * (time_to_collision - particle_time[particle2_idx]);


            // Resolve new velocities
            let (new_velocity1, new_velocity2) =
                particle_vs_particle_velocity_resolver(&particle1, &particle2, collision.normal);
            particle1.velocity = new_velocity1;
            particle2.velocity = new_velocity2;
        
            // Assign new particle state back
            particles[collision.particle] = particle1;
            particles[particle2_idx] = particle2;
            // Track the particle time
            particle_time[collision.particle] = time_to_collision;
            particle_time[particle2_idx] = time_to_collision;

            // Delete all other collisions that involved these 2 particles.
            // They become invalid because particles velocity was altered
            current_collisions.retain(|Reverse(c)| {
                !c.involves_particle(collision.particle)
            });
            current_collisions.retain(|Reverse(c)| {
                !c.involves_particle(particle2_idx)
            });
            // Now we need to calculate all new collisions with particle1 and particle2
            merge(
                &mut current_collisions,
                &find_collisions_with_particles(
                    collision.particle,
                    0..particles.len(),
                    particles,
                    particle_class_map,
                    &particle_time,
                    timestep,
                ),
            );
            merge(
                &mut current_collisions,
                &find_collisions_with_particles(
                    particle2_idx,
                    0..particles.len(),
                    particles,
                    particle_class_map,
                    &particle_time,
                    timestep,
                ),
            );
            
        }
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
            particle: 0,
            other: OtherObject::Particle(1),
            normal: Vec2::new(0.0, 0.0),
            time: ordered_float::OrderedFloat(0.0),
        }));
        heap.push(Reverse(Collision {
            particle: 2,
            other: OtherObject::Particle(3),
            normal: Vec2::new(0.0, 0.0),
            time: ordered_float::OrderedFloat(1.0),
        }));
        heap.push(Reverse(Collision {
            particle: 3,
            other: OtherObject::Particle(4),
            normal: Vec2::new(0.0, 0.0),
            time: ordered_float::OrderedFloat(0.5),
        }));
        assert_eq!(heap.pop().unwrap().0.time, ordered_float::OrderedFloat(0.0));
        assert_eq!(heap.pop().unwrap().0.time, ordered_float::OrderedFloat(0.5));
        assert_eq!(heap.pop().unwrap().0.time, ordered_float::OrderedFloat(1.0));
    }

    #[test]
    fn test_find_collisions_with_particles() {
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
                let collisions = find_collisions_with_particles(
                    1,
                    0..particles.len(),
                    &particles,
                    &classes,
                    &times,
                    time_threshold, // no enough to catch up to last
                );
                assert_eq!(collisions.len(), 2);
                assert_eq!(collisions[0].particle, 1);
                assert_eq!(collisions[0].other, OtherObject::Particle(2));
                assert!(math_core::approx_eq(
                    collisions[0].time.0,
                    time_to_first,
                    DOUBLE_COMPARE_EPS_STRICT
                ));
                assert_eq!(collisions[1].particle, 1);
                assert_eq!(collisions[1].other, OtherObject::Particle(3));
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
        let resolve_velocity = |p1: &Particle, p2: &Particle, n: Vec2| {
            return collision_utils::particles_collision_separation_velocity(
                p1.velocity,
                classes.get(&p1.class()).unwrap().mass(),
                p2.velocity,
                classes.get(&p2.class()).unwrap().mass(),
                n,
                1.0,
            );
        };
        // resolver with walls. Is not needed
        let resolve_wall = |_: &Particle, _: &Wall, _: Vec2| -> Vec2 { Vec2::new(0.0, 0.0) };

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
        resolve(
            &mut particles,
            &classes,
            &[],
            &HashMap::new(),
            30.0,
            resolve_velocity,
            resolve_wall,
        );

        // Check the result
        assert!(particles[0]
            .position
            .approx_eq(Vec2::new(10.0 + 2.0, 0.0 - 20.0), DISTANCE_EPS));
        assert!(particles[0]
            .velocity
            .approx_eq(Vec2::new(0.0, -1.0), DISTANCE_EPS));

        assert!(particles[1]
            .position
            .approx_eq(Vec2::new(7.0, 0.0), DISTANCE_EPS));
        assert!(particles[1]
            .velocity
            .approx_eq(Vec2::new(0.0, 0.0), DISTANCE_EPS));

        assert!(particles[2]
            .position
            .approx_eq(Vec2::new(14.0 + 21.0, 0.0), DISTANCE_EPS));
        assert!(particles[2]
            .velocity
            .approx_eq(Vec2::new(1.0, 0.0), DISTANCE_EPS));

        assert!(particles[3]
            .position
            .approx_eq(Vec2::new(14.0 + 10.0, -15.0 + 30.0), DISTANCE_EPS));
        assert!(particles[3]
            .velocity
            .approx_eq(Vec2::new(1.0, 1.0), DISTANCE_EPS));

        assert!(particles[4]
            .position
            .approx_eq(Vec2::new(-8.0 + 20.0, 5.0), DISTANCE_EPS));
        assert!(particles[4]
            .velocity
            .approx_eq(Vec2::new(0.0, 0.0), DISTANCE_EPS));

        assert!(particles[5]
            .position
            .approx_eq(Vec2::new(20.0 + 30.0, 20.0 + 30.0), DISTANCE_EPS));
        assert!(particles[5]
            .velocity
            .approx_eq(Vec2::new(1.0, 1.0), DISTANCE_EPS));

        assert!(particles[6]
            .position
            .approx_eq(Vec2::new(12.0, 2.0), DISTANCE_EPS));
        assert!(particles[6]
            .velocity
            .approx_eq(Vec2::new(0.0, 0.0), DISTANCE_EPS));
    }
}