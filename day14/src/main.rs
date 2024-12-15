use modular::modulo;
use modular::Modulo;
use std::collections::HashSet;
use std::io;
use vector2d::Vector2D;
use AoClib::*;

const sizex: i64 = 101;
const sizey: i64 = 103;

fn main() {
    let lines = read_lines("input".to_string());
    println!("{lines:?}");

    let mut robots = lines
        .into_iter()
        .map(|l| parse_line(&l))
        .collect::<Vec<Robot>>();
    let mut robots2 = robots.clone();

    println!("{robots:?}");

    for _ in 0..100 {
        for r in &mut robots {
            r.step();
        }
    }
    let part1 = count_quads(&robots);
    println!("{part1:?}");

    let mut i = 0;
    let mut _buffer = String::new();
    while true {
        for r in &mut robots2 {
            r.step();
        }
        println!("{i}");
        i += 1;
        visualise(&robots2);
        io::stdin().read_line(&mut _buffer);
    }
}

fn modulo(z: i64, m: i64) -> i64 {
    let remainder = z % m;
    if remainder < 0 {
        return remainder + m;
    }
    return remainder;
}

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
struct Robot {
    pub position: Vector2D<i64>,
    pub velocity: Vector2D<i64>,
}

impl Robot {
    pub fn step(&mut self) {
        self.position = self.position + self.velocity;
        self.normalize()
    }

    fn normalize(&mut self) {
        self.position.x = modulo(self.position.x, sizex);
        self.position.y = modulo(self.position.y, sizey);
    }
}

fn parse_line(line: &String) -> Robot {
    let relevant = &line
        .split("p=")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[1];
    let parts = relevant
        .split(" v=")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let poss = parts[0]
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let velocities = parts[1]
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let result = Robot {
        position: Vector2D::new(poss[0], poss[1]),
        velocity: Vector2D::new(velocities[0], velocities[1]),
    };

    return result;
}

fn count_quads(robots: &Vec<Robot>) -> u64 {
    let mut result = 0;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for r in robots {
        if r.position.x < (sizex - 1) / 2 {
            if r.position.y < (sizey - 1) / 2 {
                q1 += 1;
            }
            if r.position.y > (sizey - 1) / 2 {
                q2 += 1;
            }
        }
        if r.position.x > (sizex - 1) / 2 {
            if r.position.y < (sizey - 1) / 2 {
                q3 += 1;
            }
            if r.position.y > (sizey - 1) / 2 {
                q4 += 1;
            }
        }
    }
    return q1 * q2 * q3 * q4;
}

fn visualise(robots: &Vec<Robot>) {
    for y in 0..sizey {
        let mut line = "".to_string();
        for x in 0..sizex {
            let mut state = false;
            for r in robots {
                if r.position == Vector2D::new(x, y) {
                    line.push('#');
                    state = true;
                    break;
                }
            }
            if !state {
                line.push(' ')
            }
        }
        println!("{line}");
    }
}
