use std::io::{Read, Write};
use std::net::TcpStream;

use library::{generate, star};

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

            // if a char is received as '}', it's the flag
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

        // write string to file for testing
        //let mut file = std::fs::File::create(&format!("tests/{}.txt", iteration));
        //if let Ok(mut file) = file {
        //    file.write(input.as_bytes());
        //}

        // create grid for CCD image
        let grid = star::Grid::from_str(input);

        // find star positions
        let stars = star::Stars::from_grid(&grid);

        // we have all the stars, send them
        println!("{:#?}", stars);
        stars.write(&stream).unwrap();

        // generate actual ccd image
        generate::image(&grid, iteration);

        // recieve some boilerplate strings that come after a success
        loop {
            let mut buf = vec![0u8, 6];
            stream.read_exact(&mut buf)?;
            let s = std::str::from_utf8(&buf).unwrap();
            if s.chars().any(|a| a == '\n') {
                break;
            }
        }
        iteration += 1;
    }
}
