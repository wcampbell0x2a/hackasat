use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
struct Star {
    i: u8,
    j: u8,
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

type Bodies = Vec<Star>;

#[derive(Debug)]
struct Stars {
    stars: Vec<Star>,
}

impl Stars {
    pub fn new() -> Self {
        Stars { stars: Vec::new() }
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

fn generate_image(grid: &Vec<Vec<u8>>, iteration: u8) {
    let mut imgbuf = image::RgbImage::new(129, 129);
    for (i, line) in grid.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            imgbuf.put_pixel(i as u32, j as u32, image::Rgb([*val, 0, 0]));
        }
    }
    imgbuf.save(&format!("{}.png", iteration)).unwrap();
}

fn main() -> std::io::Result<()> {
    // read from tcp socket
    let mut stream = TcpStream::connect("stars.satellitesabove.me:5013")?;
    let mut buf = [0; 128];
    let bytes_read = stream.read(&mut buf)?;
    println!("{}", std::str::from_utf8(&buf[..bytes_read]).unwrap());

    // read/send ticket
    let mut file = std::fs::File::open("ticket")?;
    let mut ticket = String::new();
    file.read_to_string(&mut ticket)?;

    stream.write_all(format!("{}\n", ticket).as_bytes())?;

    let mut iteration = 0;

    loop {
        println!("receiving");
        let mut real_buf = vec![];
        loop {
            // Attempt to recieve CCD camera data and put into vec
            let mut buf = vec![0u8, 2];
            stream.read_exact(&mut buf)?;
            let s = std::str::from_utf8(&buf).unwrap();

            // if a char is recieved as '}', it's the flag
            if s.chars().any(|a| a == '}') {
                real_buf.append(&mut buf);
                // print flag!
                println!("{}", std::str::from_utf8(&real_buf).unwrap());
                return Ok(());
            }
            // otherwise, it's a end of data with a newline
            if buf == [10, 69] || buf == [10, 10] {
                break;
            } else {
                real_buf.append(&mut buf);
            }
        }
        let input = std::str::from_utf8(&real_buf).unwrap();

        // read in useless strings after CCD data
        loop {
            let mut buf = vec![0u8, 2];
            stream.read_exact(&mut buf)?;
            let s = std::str::from_utf8(&buf).unwrap();
            if s.chars().any(|a| a == '\n') {
                break;
            }
        }

        // create a 2d vector of i and j data
        let mut grid = vec![vec![]];

        for line in input.lines() {
            println!("{}", line);
            let mut v = vec![];
            for val in line.split(',') {
                if let Ok(val) = val.parse::<u8>() {
                    v.push(val);
                }
            }
            grid.push(v);
        }
        grid.remove(0);

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
        // we have all the stars, send them
        println!("{:#?}", stars);
        stars.write(&stream).unwrap();

        // generate actual ccd image
        generate_image(&grid, iteration);

        // recieve some boilerplate strings that come after a success
        loop {
            let mut buf = vec![0u8, 6];
            stream.read_exact(&mut buf)?;
            let s = std::str::from_utf8(&buf).unwrap();
            if s.chars().any(|a| a == '\n') {
                break;
            }
        }
        iteration = iteration + 1;
    }
}
