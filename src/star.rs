use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Star {
    pub i: u8,
    pub j: u8,
}

impl Star {
    // TODO trait for usize/u8
    pub fn new(i: usize, j: usize) -> Self {
        Star {
            i: i as u8,
            j: j as u8,
        }
    }
}

impl PartialEq<(usize, usize)> for Star {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.i == other.0 as u8 && self.j == other.1 as u8
    }
}

impl PartialEq<Star> for Star {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j
    }
}

pub type Bodies = Vec<Star>;

#[derive(Debug)]
pub struct Stars {
    pub stars: Vec<Star>,
}

impl Stars {
    pub fn new() -> Self {
        Stars { stars: Vec::new() }
    }

    pub fn from_grid(grid: &Vec<Vec<u8>>) -> Self {
        // keep track of bodies of stars and the brightest stars
        let mut stars = Stars::new();
        let mut bodies = Bodies::new();
        for (i, line) in grid.iter().enumerate() {
            for (j, val) in line.iter().enumerate() {
                if *val > 150 as u8 {
                    bodies.push(Star::new(i, j));
                    stars.push(i, j, &bodies);
                }
            }
        }
        stars
    }

    pub fn push(&mut self, i: usize, j: usize, bodies: &Bodies) {
        if !bodies.iter().any(|a| *a == (i - 1, j))
            && !bodies.iter().any(|a| *a == (i, j - 1))
            && !bodies.iter().any(|a| *a == (i - 1, j - 1))
            && !bodies.iter().any(|a| *a == (i + 1, j - 1))
            && !bodies.iter().any(|a| *a == (i + 1, j))
            && !bodies.iter().any(|a| *a == (i + 1, j + 1))
            && !bodies.iter().any(|a| *a == (i, j + 1))
            && !bodies.iter().any(|a| *a == (i - 1, j + 1))
        {
            self.stars.push(Star::new(i, j));
        }
    }

    pub fn write(&self, mut stream: &TcpStream) -> std::io::Result<()> {
        for star in &self.stars {
            let s = format!("{},{}\n", star.i, star.j);
            stream.write_all(s.as_bytes())?;
        }
        stream.write_all(b"\n")?;
        Ok(())
    }
}

/// create a 2d vector of i and j data
pub fn create_grid(input: &str) -> Vec<Vec<u8>> {
    let mut grid = vec![vec![]];
    for line in input.lines() {
        let mut v = vec![];
        for val in line.split(',') {
            if let Ok(val) = val.parse::<u8>() {
                v.push(val);
            }
        }
        grid.push(v);
    }
    grid.remove(0);
    grid
}
