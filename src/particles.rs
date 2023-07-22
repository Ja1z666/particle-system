use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

#[derive(Reflect)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Particle {
    speed: f32,
    direction: f32,
    life_time: f32,
    created_at: f32,
}

#[derive(Component, Reflect)]
pub struct Particles {
    pub position: Position,
    pub looping: bool,
    pub life_time: f32,
    pub color: Color,
    pub speed: f32,
    pub size: f32,
    pub direction: f32,
    pub angle: f32,
    pub rate: u32,
}

impl Default for Particles {
    fn default() -> Self {
        Self {
            position: Position { x: 0., y: 0. },
            looping: true,
            life_time: 3.,
            color: Color::WHITE,
            speed: 100.,
            size: 10.,
            direction: 0.,
            angle: 30.,
            rate: 300,
        }
    }
}

impl Particles {
    pub fn system(
        particles_query: Query<(&mut Particles, Entity)>,
        particle_query: Query<(&Particle, Entity, &mut Transform)>,
        mut commands: Commands,
        time: Res<Time>,
    ) {
        Particles::spawn_particle(particles_query, &mut commands, &time);
        Particles::transform_particle(particle_query, &mut commands, &time);
    }

    fn generate_particle(particles: &Particles, time: &Time) -> Particle {
        let mut rng = rand::thread_rng();
        Particle {
            speed: particles.speed + rng.gen_range(0_f32..=particles.speed / 100. * 20.),
            direction: particles.direction - particles.angle * 0.5
                + rng.gen_range(0_f32..=particles.angle),
            life_time: particles.life_time
                + rng.gen_range(0_f32..=particles.life_time / 100. * 30.),
            created_at: time.raw_elapsed_seconds(),
        }
    }

    fn spawn_particle(
        mut particles_query: Query<(&mut Particles, Entity)>,
        commands: &mut Commands,
        time: &Time,
    ) {
        let mut rng = rand::thread_rng();

        for (mut particles, entity) in &mut particles_query {
            if particles.looping {
                if rng.gen_range(0..=particles.rate / 100) == particles.rate / 100 {
                    let particle = commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                color: particles.color,
                                custom_size: Some(Vec2::new(particles.size, particles.size)),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(
                                particles.position.x,
                                particles.position.y,
                                0.,
                            )),
                            ..default()
                        })
                        .insert(Particles::generate_particle(&particles, &time))
                        .id();
                    commands.entity(entity).push_children(&[particle]);
                }
            } else {
                if particles.rate > 0 {
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                color: particles.color,
                                custom_size: Some(Vec2::new(particles.size, particles.size)),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(
                                particles.position.x,
                                particles.position.y,
                                0.,
                            )),
                            ..default()
                        })
                        .insert(Particles::generate_particle(&particles, &time));
                    particles.rate -= 1;
                } else {
                    commands.entity(entity).despawn();
                }
            }
        }
    }

    fn transform_particle(
        mut particle_query: Query<(&Particle, Entity, &mut Transform)>,
        commands: &mut Commands,
        time: &Time,
    ) {
        for (particle, entity, mut transform) in &mut particle_query {
            if particle.created_at + particle.life_time > time.raw_elapsed_seconds() {
                transform.translation += Vec3::new(
                    f32::cos(PI * particle.direction / 180.)
                        * particle.speed
                        * time.delta_seconds(),
                    f32::sin(PI * particle.direction / 180.)
                        * particle.speed
                        * time.delta_seconds(),
                    0.,
                );
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}
