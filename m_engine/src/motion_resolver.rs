use crate::collision_utils;
use crate::collision_utils::find_particle_vs_polygon_collision;
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
            // Allow for collisions that are slightly in the past. These can appear due to
            // floating point errors
            if collision_time > (particle_times[main_index] - TIME_SEC_EPS)
                && collision_time > (particle_times[i] - TIME_SEC_EPS)
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

/// Finds all collisions between a particle and a range of walls
fn find_collisions_with_walls(
    particle_index: usize,
    particle: &Particle,
    particle_class: &ParticleClass,
    other_walls: &[Wall],
    particle_time: f64,
    time_threshold: f64,
) -> Vec<Collision> {
    let mut collisions = vec![];

    for (i, wall) in other_walls.iter().enumerate() {
        // Bring particle to t=0
        let pos = particle.position - particle.velocity * particle_time;
        let collision_res = find_particle_vs_polygon_collision(
            pos,
            particle_class.radius(),
            particle.velocity,
            wall.polygon(),
        );
        if let Some((collision_time, collision_normal)) = collision_res {
            // Check if the collision is in the future. But not too far in the future
            // Allow for collisions that are slightly in the past. These can appear due to
            // floating point errors
            if collision_time > (particle_time - TIME_SEC_EPS) && collision_time < time_threshold {
                collisions.push(Collision {
                    particle: particle_index,
                    other: OtherObject::Wall(i),
                    normal: collision_normal,
                    time: ordered_float::OrderedFloat(collision_time),
                });
            }
        }
    }

    return collisions;
}

fn resolve_particle_vs_particle(
    mut particle1: Particle,
    mut particle2: Particle,
    particle1_t: f64,
    particle2_t: f64,
    collision_t: f64,
    collision_normal: Vec2,
    velocity_resolver: &impl Fn(&Particle, &Particle, Vec2) -> (Vec2, Vec2),
) -> (Particle, Particle) {
    // Advance particles to the moment of collision
    particle1.position += particle1.velocity * (collision_t - particle1_t);
    particle2.position += particle2.velocity * (collision_t - particle2_t);

    // Resolve new velocities
    let (new_velocity1, new_velocity2) =
        velocity_resolver(&particle1, &particle2, collision_normal);
    particle1.velocity = new_velocity1;
    particle2.velocity = new_velocity2;

    return (particle1, particle2);
}

fn resolve_particle_vs_wall(
    mut particle1: Particle,
    wall: &Wall,
    particle1_t: f64,
    collision_t: f64,
    collision_normal: Vec2,
    velocity_resolver: &impl Fn(&Particle, &Wall, Vec2) -> Vec2,
) -> Particle {
    // Advance particle to the moment of collision
    particle1.position += particle1.velocity * (collision_t - particle1_t);

    // Resolve new velocity
    let new_velocity = velocity_resolver(&particle1, wall, collision_normal);
    particle1.velocity = new_velocity;

    return particle1;
}

pub(crate) fn resolve(
    particles: &mut [Particle],
    particle_class_map: &HashMap<ClassId, ParticleClass>,
    walls: &[Wall],
    timestep: f64,
    particle_vs_particle_velocity_resolver: &impl Fn(&Particle, &Particle, Vec2) -> (Vec2, Vec2),
    particle_vs_wall_velocity_resolver: &impl Fn(&Particle, &Wall, Vec2) -> Vec2,
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
    for i in 0..particles.len() {
        // With particles in front
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
        // With all wals
        merge(
            &mut current_collisions,
            &find_collisions_with_walls(
                i,
                &particles[i],
                particle_class_map.get(&particles[i].class()).unwrap(),
                walls,
                particle_time[i],
                timestep,
            ),
        );
    }

    // Keep resolving collisions while there are any
    while let Some(Reverse(collision)) = current_collisions.pop() {
        let time_to_collision = collision.time.0;

        // Track the particles that collided and need collision reset
        let mut particles_to_reset_collisions = vec![];

        // Collision with other particle
        match collision.other {
            OtherObject::Particle(particle2_idx) => {
                let (p1, p2) = resolve_particle_vs_particle(
                    particles[collision.particle],
                    particles[particle2_idx],
                    particle_time[collision.particle],
                    particle_time[particle2_idx],
                    time_to_collision,
                    collision.normal,
                    particle_vs_particle_velocity_resolver,
                );
                particles[collision.particle] = p1;
                particles[particle2_idx] = p2;

                // Track the particle time
                particle_time[collision.particle] = time_to_collision;
                particle_time[particle2_idx] = time_to_collision;

                // Particle had collision. That means all other collisions with this particle
                // are invalid. We need to recalculate them
                particles_to_reset_collisions.push(collision.particle);
                particles_to_reset_collisions.push(particle2_idx);
            }
            OtherObject::Wall(wall_idx) => {
                let p1 = resolve_particle_vs_wall(
                    particles[collision.particle],
                    &walls[wall_idx],
                    particle_time[collision.particle],
                    time_to_collision,
                    collision.normal,
                    particle_vs_wall_velocity_resolver,
                );
                particles[collision.particle] = p1;

                // Track the particle time
                particle_time[collision.particle] = time_to_collision;

                // Particle had collision. That means all other collisions with this particle
                // are invalid. We need to recalculate them
                particles_to_reset_collisions.push(collision.particle);
            }
        }

        // Delete all collisions of involved partciles
        for &particle_idx in &particles_to_reset_collisions {
            current_collisions.retain(|Reverse(c)| !c.involves_particle(particle_idx));
        }

        // Generate new collisions for each involved particle
        for &particle_idx in &particles_to_reset_collisions {
            merge(
                &mut current_collisions,
                &find_collisions_with_particles(
                    particle_idx,
                    0..particles.len(),
                    particles,
                    particle_class_map,
                    &particle_time,
                    timestep,
                ),
            );
            merge(
                &mut current_collisions,
                &find_collisions_with_walls(
                    particle_idx,
                    &particles[particle_idx],
                    particle_class_map
                        .get(&particles[particle_idx].class())
                        .unwrap(),
                    walls,
                    particle_time[particle_idx],
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

pub fn default_particle_vs_particle_velocity_resovler<'a>(
    particle_classes: &'a HashMap<ClassId, ParticleClass>
) -> impl Fn(&Particle, &Particle, Vec2) -> (Vec2, Vec2) + 'a {
    move |p1: &Particle, p2: &Particle, n: Vec2| {
        collision_utils::particles_collision_separation_velocity(
            p1.velocity,
            particle_classes.get(&p1.class()).unwrap().mass(),
            p2.velocity,
            particle_classes.get(&p2.class()).unwrap().mass(),
            n,
            1.0,
        )
    }
}

pub fn default_particle_vs_wall_velocity_resolver<'a>(
    wall_classes: &'a HashMap<ClassId, WallClass>
) -> impl Fn(&Particle, &Wall, Vec2) -> Vec2 + 'a {
    move |p: &Particle, w: &Wall, n: Vec2| {
        collision_utils::particles_vs_wall_collision_separation_velocity(
            p.velocity,
            n,
            wall_classes.get(&w.class()).unwrap().coefficient_of_restitution(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math_core, Polygon, WallClass};

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

        {
            // Test special case. When collision is barely in the past - it is still accepted
            // Such collisions may happen due to floating point errors
            let particle1 = Particle::new(
                Vec2::new(0.0, 2.0 - TIME_SEC_EPS * 0.9 * 1.0),
                Vec2::new(0.0, -1.0),
                1,
            );
            let particle2 = Particle::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0), 1);
            let collisions = find_collisions_with_particles(
                0,
                0..2,
                &[particle1, particle2],
                &classes,
                &[0.0, 0.0],
                100.0,
            );
            assert_eq!(collisions.len(), 1);

            // And case with collision that is deeper than time eps
            let particle1 = Particle::new(
                Vec2::new(0.0, 2.0 - TIME_SEC_EPS * 1.1 * 1.0),
                Vec2::new(0.0, -1.0),
                1,
            );
            let collisions = find_collisions_with_particles(
                0,
                0..2,
                &[particle1, particle2],
                &classes,
                &[0.0, 0.0],
                100.0,
            );
            assert_eq!(collisions.len(), 0);
        }
    }

    #[test]
    fn test_find_collisions_with_walls() {
        // Make 4 walls in a row
        let wall1 = Wall::new(Polygon::new_rectangle(1.0, 1.0, 10.0, 2.0), 0);
        let wall2 = Wall::new(Polygon::new_rectangle(1.0, 5.0, 10.0, 6.0), 0);
        let wall3 = Wall::new(Polygon::new_rectangle(1.0, 9.0, 10.0, 10.0), 0);
        let wall4 = Wall::new(Polygon::new_rectangle(1.0, 15.0, 10.0, 16.0), 0);
        let walls = vec![wall1, wall2, wall3, wall4];

        // Particle and class
        let particle_class = ParticleClass::new("Class1", 1.0, 1.0);
        let speed = 2.0;
        let y0 = -1.0;
        let mut particle = Particle::new(Vec2::new(4.0, y0), Vec2::new(0.0, speed), 1);
        // Advance particle forward, so it cant hit wall 1
        let particle_time = (3.0 - y0) / speed;
        particle.position += particle.velocity * particle_time;

        // Time threshold is such that it has not enought time to hit wall4
        let time_threshold = (13.0 - y0) / speed;

        // Find collisions
        let collisions = find_collisions_with_walls(
            123,
            &particle,
            &particle_class,
            &walls,
            particle_time,
            time_threshold,
        );
        assert_eq!(collisions.len(), 2);
        assert_eq!(collisions[0].particle, 123);
        assert_eq!(collisions[0].other, OtherObject::Wall(1));
        assert!(collisions[0]
            .normal
            .approx_eq(Vec2::new(0.0, -1.0), DOUBLE_COMPARE_EPS_STRICT));
        assert!(math_core::approx_eq(
            collisions[0].time.0,
            (4.0 - y0) / speed,
            DOUBLE_COMPARE_EPS_STRICT
        ));
        assert_eq!(collisions[1].particle, 123);
        assert_eq!(collisions[1].other, OtherObject::Wall(2));
        assert!(collisions[1]
            .normal
            .approx_eq(Vec2::new(0.0, -1.0), DOUBLE_COMPARE_EPS_STRICT));
        assert!(math_core::approx_eq(
            collisions[1].time.0,
            (8.0 - y0) / speed,
            DOUBLE_COMPARE_EPS_STRICT
        ));

        // Test special case. When collision is barely in the past - it is still accepted
        // Such collisions may happen due to floating point errors
        let y = 14.0 + TIME_SEC_EPS * 0.9 * 1.0;
        let particle = Particle::new(Vec2::new(4.0, y), Vec2::new(0.0, 1.0), 1);
        let collisions =
            find_collisions_with_walls(123, &particle, &particle_class, &walls, 0.0, 100.0);
        assert_eq!(collisions.len(), 1);
        // And case with collision that is deeper than time eps
        let y = 14.0 + TIME_SEC_EPS * 1.1 * 1.0;
        let particle = Particle::new(Vec2::new(4.0, y), Vec2::new(0.0, 1.0), 1);
        let collisions =
            find_collisions_with_walls(123, &particle, &particle_class, &walls, 0.0, 100.0);
        assert_eq!(collisions.len(), 0);
    }

    #[test]
    pub fn test_resolve() {
        // Add couple of classes. All particles have the same mass.
        // Combined with coefficient of restitution of 1 - it will lead to simpler numbers
        // and easier to analyze code.
        let mut classes = HashMap::new();
        classes.insert(1, ParticleClass::new("Class1", 1.0, 1.0));
        classes.insert(2, ParticleClass::new("Class2", 1.0, 2.0));
        let wall_classes = HashMap::new();
        // Lamda that resolve velocity
        let resolve_velocity = default_particle_vs_particle_velocity_resovler(&classes);

        // resolver with walls. Is not needed
        let resolve_wall = default_particle_vs_wall_velocity_resolver(&wall_classes);

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
            30.0,
            &resolve_velocity,
            &resolve_wall,
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

    #[test]
    pub fn test_resolve_long() {
        // Main utility of resolve() function is to resolve multiple collisions
        // happening at the same time step.
        // One way to test it's correctness is compare results of multiple tiny steps
        // with results of single large steps.

        // Make single wall class and single particles class
        let mut particle_classes = HashMap::new();
        particle_classes.insert(1, ParticleClass::new("Class1", 1.0, 1.0));
        let mut wall_classes = HashMap::new();
        wall_classes.insert(1, WallClass::new("Wall", 1.0));


        // Lamda that resolve velocity
        let resolve_p_p = default_particle_vs_particle_velocity_resovler(&particle_classes);
        let resolve_p_w = default_particle_vs_wall_velocity_resolver(&wall_classes);
        
        
        // Make a box for a scene (about 8x8 on the inside)
        let walls = Wall::make_box(-5.0, -5.0, 5.0, 5.0, 1.0, 1);
        // Add some particles flying in random directions
        let mut particles1 = vec![
            Particle::new(Vec2::new(-2.0, 2.0), Vec2::new(1.0, 2.0), 1),
            Particle::new(Vec2::new(2.0, 2.0), Vec2::new(-1.12, -5.0), 1),
            Particle::new(Vec2::new(2.0, -2.0), Vec2::new(-3.12, -1.0), 1),
            Particle::new(Vec2::new(-2.0, -2.0), Vec2::new(8.12, 0.5), 1),
            Particle::new(Vec2::new(0.0, 0.0), Vec2::new(3.0, 1.0), 1),
        ];
        // Second copy of particles to simulate
        let mut particles2 = particles1.clone();

        // Configure simulation duration
        // Pick reasonable duration that would involve multiple collisions
        // Can't take too large of a duration because this is chaotic system
        // and even numerical errors can lead divergence in long run
        // The divergence is actually very significant. So can't stretch this too much
        let duration = 10.0;
        let steps = 50;
        let time_step = duration / steps as f64;

        // First is simulated in single step
        resolve(
            &mut particles1,
            &particle_classes,
            &walls,
            duration,
            &resolve_p_p,
            &resolve_p_w,
        );

        // Second is simulated in multiple steps
        for _ in 0..steps {
            resolve(
                &mut particles2,
                &particle_classes,
                &walls,
                time_step,
                &resolve_p_p,
                &resolve_p_w,
            );
        }

        // use larger epsilon to counter numerical errors buildup
        let eps = 0.01;
        // Now compare
        for (p1, p2) in particles1.iter().zip(particles2.iter()) {
            assert!(p1.position.approx_eq(p2.position, eps));
            assert!(p1.velocity.approx_eq(p2.velocity, eps));
        }
    }

}
