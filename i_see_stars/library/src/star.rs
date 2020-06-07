use std::io::Write;
use std::net::TcpStream;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct Star {
    pub i: u8,
    pub j: u8,
}

impl Star {
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

#[derive(Debug, Default)]
pub struct Stars {
    pub stars: Vec<Star>,
}

impl Stars {
    pub fn new() -> Self {
        Stars {
            ..Default::default()
        }
    }

    pub fn from_grid(grid: &[Vec<u8>]) -> Self {
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

    pub fn push(&mut self, i: usize, j: usize, bodies: &[Star]) {
        let i_less = if i > 0 {
            !bodies.iter().any(|a| *a == (i - 1, j)) && !bodies.iter().any(|a| *a == (i - 1, j + 1))
        } else {
            false
        };
        let j_less = if j > 0 {
            !bodies.iter().any(|a| *a == (i, j - 1)) && !bodies.iter().any(|a| *a == (i + 1, j - 1))
        } else {
            false
        };
        let ji_less = if i > 0 && j > 0 {
            !bodies.iter().any(|a| *a == (i - 1, j - 1))
        } else {
            false
        };
        let regular = !bodies.iter().any(|a| *a == (i + 1, j))
            && !bodies.iter().any(|a| *a == (i + 1, j + 1))
            && !bodies.iter().any(|a| *a == (i, j + 1));
        if i_less && j_less & ji_less && regular {
            self.stars.push(Star::new(i, j));
        }
    }

    pub fn write(&self, mut stream: &TcpStream) -> std::io::Result<()> {
        for star in &self.stars {
            let s = format!("{},{}\n", star.i, star.j);
            stream.write_all(s.as_bytes())?;
        }
        Ok(stream.write_all(b"\n")?)
    }
}

impl Deref for Stars {
    type Target = Vec<Star>;

    fn deref(&self) -> &Self::Target {
        &self.stars
    }
}

pub struct Grid {
    inner: Vec<Vec<u8>>,
}

impl Grid {
    pub fn new(inner: Vec<Vec<u8>>) -> Self {
        Grid { inner }
    }

    pub fn to_stream(&self) -> Vec<u8> {
        self.inner
            .to_vec()
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>()
    }
}

impl FromStr for Grid {
    type Err = std::num::ParseIntError;

    /// create a 2d vector of i and j data
    fn from_str(input: &str) -> Result<Self, Self::Err> {
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
        Ok(Self { inner: grid })
    }
}

impl Deref for Grid {
    type Target = Vec<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grid_stream() {
        let grid = Grid::new(vec![vec![0x01, 0x02], vec![0x03, 0x04]]);
        assert_eq!(vec![0x01, 0x02, 0x03, 0x04], grid.to_stream());
    }
}
