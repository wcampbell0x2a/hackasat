#[cfg(test)]
mod tests {
    use i_see_stars::star::{Star};
    use i_see_stars::{star, xor};
    use std::io::Read;

    #[test]
    fn test() -> std::io::Result<()> {
        let mut file = std::fs::File::open("tests/0.txt").unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        // create grid for CCD image
        let grid = star::create_grid(&s);

        let grid = xor::xor_grid(&grid, 0x0a);
        if let Some(key) = xor::find_xor_key(&grid) {
            let grid = xor::xor_grid(&grid, key);

            // find star positions
            let stars = star::Stars::from_grid(&grid);
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
        } else {
            unreachable!();
        }

        Ok(())
    }
}
