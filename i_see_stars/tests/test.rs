#[cfg(test)]
mod tests {
    use library::star::{Star, Stars};
    use library::{generate, star};
    use std::io::Read;

    fn test_str(iteration: u8) -> Stars {
        let mut file = std::fs::File::open(format!("tests/{}.txt", iteration)).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        println!("{}", s);

        // create grid for CCD image
        let grid = star::Grid::from_str(&s);

        // generate realistic CCD image
        generate::image(&grid, iteration);

        // find star positions
        star::Stars::from_grid(&grid)
    }

    #[test]
    fn test() -> std::io::Result<()> {
        let stars = test_str(0);

        println!("{:?}", stars);
        assert_eq!(
            stars.stars,
            vec![
                Star { i: 14, j: 51 },
                Star { i: 20, j: 115 },
                Star { i: 25, j: 30 },
                Star { i: 47, j: 17 },
                Star { i: 55, j: 79 },
                Star { i: 64, j: 106 },
                Star { i: 72, j: 25 },
                Star { i: 95, j: 98 },
                Star { i: 111, j: 28 },
                Star { i: 117, j: 82 }
            ]
        );

        let stars = test_str(1);
        println!("{:?}", stars);
        assert_eq!(
            stars.stars,
            vec![
                Star { i: 15, j: 43 },
                Star { i: 18, j: 119 },
                Star { i: 36, j: 80 },
                Star { i: 47, j: 52 },
                Star { i: 67, j: 89 },
                Star { i: 80, j: 46 },
                Star { i: 97, j: 42 },
                Star { i: 109, j: 112 },
                Star { i: 118, j: 54 },
                Star { i: 118, j: 85 }
            ]
        );

        let stars = test_str(2);
        println!("{:?}", stars);
        assert_eq!(
            stars.stars,
            vec![
                Star { i: 6, j: 47 },
                Star { i: 8, j: 23 },
                Star { i: 10, j: 121 },
                Star { i: 23, j: 40 },
                Star { i: 27, j: 103 },
                Star { i: 64, j: 44 },
                Star { i: 75, j: 82 },
                Star { i: 91, j: 9 },
                Star { i: 104, j: 97 },
                Star { i: 122, j: 53 }
            ]
        );

        let stars = test_str(3);
        println!("{:?}", stars);
        assert_eq!(
            stars.stars,
            vec![
                Star { i: 11, j: 43 },
                Star { i: 18, j: 65 },
                Star { i: 32, j: 83 },
                Star { i: 42, j: 7 },
                Star { i: 51, j: 66 },
                Star { i: 58, j: 92 },
                Star { i: 67, j: 43 },
                Star { i: 81, j: 25 },
                Star { i: 83, j: 95 },
                Star { i: 84, j: 53 }
            ]
        );

        let stars = test_str(4);
        println!("{:?}", stars);
        assert_eq!(
            stars.stars,
            vec![
                Star { i: 11, j: 36 },
                Star { i: 32, j: 85 },
                Star { i: 41, j: 60 },
                Star { i: 57, j: 99 },
                Star { i: 60, j: 67 },
                Star { i: 63, j: 122 },
                Star { i: 78, j: 39 },
                Star { i: 81, j: 116 },
                Star { i: 105, j: 57 },
                Star { i: 123, j: 21 }
            ]
        );

        Ok(())
    }
}
