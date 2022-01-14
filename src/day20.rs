use counter::Counter;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone)]
struct Particle {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
    acceleration: (i64, i64, i64)
}

impl Particle {
    fn from(string: &str) -> Option<Particle> {
        lazy_static! {
            static ref RE: Regex = 
                Regex::new(r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>")
                    .expect("Invalid Regex");
        }

        let captures = RE.captures(string)?;
        Some(
            Particle {
                position: (
                    captures.get(1)?.as_str().parse().ok()?,
                    captures.get(2)?.as_str().parse().ok()?,
                    captures.get(3)?.as_str().parse().ok()?
                ),
                velocity: (
                    captures.get(4)?.as_str().parse().ok()?,
                    captures.get(5)?.as_str().parse().ok()?,
                    captures.get(6)?.as_str().parse().ok()?
                ),
                acceleration: (
                    captures.get(7)?.as_str().parse().ok()?,
                    captures.get(8)?.as_str().parse().ok()?,
                    captures.get(9)?.as_str().parse().ok()?
                )
            }
        )
    }

    fn tick(&mut self) {
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;
        self.velocity.2 += self.acceleration.2;
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    fn manhattan_distance(&self) -> i64 {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }
}

pub fn part1(input: String) {
    let mut particles = input.lines().flat_map(Particle::from).collect_vec();
    let mut min = 0;
    for _ in 0..500 {
        particles.iter_mut().for_each(|p| p.tick());
        min = particles
            .iter()
            .enumerate()
            .min_by_key(|(_, p)| p.manhattan_distance())
            .map(|t| t.0)
            .unwrap_or_default();
    }
    println!("{}", min);
}

pub fn part2(input: String) {
    let mut particles = input.lines().flat_map(Particle::from).collect_vec();
    for _ in 0..500 {
        particles.iter_mut().for_each(|p| p.tick());
        let counts = particles.iter().map(|p| p.position).collect::<Counter<_>>();
        for (pos, _) in counts.iter().filter(|(_, c)| c > &&1) {
            particles = particles.iter().filter(|p| p.position != *pos).map(|p| p.clone()).collect_vec();
        }
    }
    println!("{}", particles.len());
}
