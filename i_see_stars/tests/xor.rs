#[cfg(test)]
mod tests {
    use library::star::Star;
    use library::{star, xor};
    use std::io::Read;

    #[test]
    fn test() -> std::io::Result<()> {
        let mut file = std::fs::File::open("tests/0.txt").unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        // create grid for CCD image
        let grid = star::Grid::from_str(&s);

        let grid = xor::xor_grid(&grid, 0x0a);
        if let Some(key) = xor::find_xor_key(&grid) {
            let grid = xor::xor_grid(&grid, key);

            // find star positions
            let stars = star::Stars::from_grid(&grid);
            assert_eq!(
                *stars,
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

    #[test]
    fn ayy() {
        let mut file = std::fs::File::open("tests/0.txt").unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        println!("before\n{}", s);

        // create grid for CCD image
        let orig_grid = star::Grid::from_str(&s);

        let grid = xor::xor_repeating_grid(&orig_grid, &[0x01, 0x1a, 0xaa, 0xab, 0x11]);
        let stream = grid.to_stream();
        let keysizes = xor::find_xor_reapeating_keysizes(&stream);
        println!("keysize?: {:?}", keysizes);
        for key in keysizes {
            let t = xor::transpose(&stream, key);
            let mut v = vec![];
            for group in t {
                let r = xor::find_xor_key_repeating(&group);
                if let Some(r) = r {
                    v.push(r);
                }
            }
            println!("maybe key?: {:x?}", v);
            let last_grid = xor::xor_repeating_grid(&grid, &v);
            assert_eq!(*last_grid, *orig_grid);

            // TODO this test actually finds two keys, so that's a problem
            break;
        }
    }
}
