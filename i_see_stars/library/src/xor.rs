use crate::star::Grid;
use histogram::Histogram;

pub fn xor_grid(grid: &Grid, key: u8) -> Grid {
    Grid::new(
        grid.iter()
            .map(|a| a.iter().map(|b| b ^ key).collect())
            .collect(),
    )
}

pub fn xor_repeating_grid(grid: &Grid, key: &[u8]) -> Grid {
    let mut count = 0;
    let length = key.len();
    Grid::new(
        grid.iter()
            .map(|a| {
                a.iter()
                    .map(|b| {
                        count += 1;
                        b ^ key[(count - 1 as usize) % length]
                    })
                    .collect()
            })
            .collect(),
    )
}

pub fn find_xor_key(grid: &Grid) -> Option<u8> {
    (0u8..0xff)
        .map(|key| (key, xor_grid(&grid, key)))
        .filter_map(|(key, xor_bytes)| Some((key, frequency_num(&xor_bytes)?)))
        .find(|(_, (mean, max, _stddev, ninety_ninth))| {
            *max == 255 && *mean < 10 && *ninety_ninth == 255
        })
        .map(|(key, _)| key)
}

pub fn find_xor_key_repeating(bytes: &[u8]) -> Option<u8> {
    (0u8..0xff)
        .map(|key| (key, bytes.iter().map(|a| a ^ key).collect::<Vec<u8>>()))
        .filter_map(|(key, xor_bytes)| Some((key, frequency_num_repeated(&xor_bytes)?)))
        .find(|(_, (mean, max, _stddev, ninety_ninth))| {
            *max == 255 && *mean < 10 && *ninety_ninth == 255
        })
        .map(|(key, _)| key)
}

pub fn find_xor_reapeating_keysizes(bytes: &[u8]) -> Vec<u8> {
    let mut v = vec![];
    for maybe_keysize in 3..=15 {
        let a = &bytes[..maybe_keysize];
        let b = &bytes[maybe_keysize..maybe_keysize * 2];
        let c = &bytes[maybe_keysize * 2..maybe_keysize * 3];
        let d = &bytes[maybe_keysize * 3..maybe_keysize * 4];
        let ham1 = hamming_distance(a, b);
        let ham2 = hamming_distance(c, d);
        let ham3 = hamming_distance(a, c);
        let ham4 = hamming_distance(a, d);
        let ham_total = ((ham1 + ham2 + ham3 + ham4) / 4) / maybe_keysize as u32;
        //let ham_total = ((ham1 + ham2) / 2) / maybe_keysize as u32;
        //let ham_total = ham1 / maybe_keysize as u32;
        println!("{}->{} ({})", maybe_keysize, ham_total, ham1);
        v.push((ham_total, maybe_keysize));
    }
    v.sort();
    v.iter().take(2).map(|a| a.1 as u8).collect()
}

pub fn transpose(bytes: &[u8], keysize: u8) -> Vec<Vec<u8>> {
    let mut transposed: Vec<Vec<u8>> = std::iter::repeat(Vec::new())
        .take(keysize as usize)
        .collect();

    for (index, byte) in bytes.iter().cloned().enumerate() {
        let bucket = index % keysize as usize;
        transposed[bucket].push(byte);
    }
    transposed
}

pub fn frequency_num(grid: &[Vec<u8>]) -> Option<(u64, u64, u64, u64)> {
    let mut histogram = Histogram::new();
    for line in grid.iter() {
        for val in line.iter() {
            histogram.increment(*val as u64).unwrap();
        }
    }

    if let (Ok(mean), Ok(max), Some(stddev), Ok(ninety_ninth)) = (
        histogram.mean(),
        histogram.maximum(),
        histogram.stddev(),
        histogram.percentile(99.9),
    ) {
        Some((mean, max, stddev, ninety_ninth))
    } else {
        None
    }
}

pub fn frequency_num_repeated(bytes: &[u8]) -> Option<(u64, u64, u64, u64)> {
    let mut histogram = Histogram::new();
    for byte in bytes.iter() {
        histogram.increment(*byte as u64).unwrap();
    }

    if let (Ok(mean), Ok(max), Some(stddev), Ok(ninety_ninth)) = (
        histogram.mean(),
        histogram.maximum(),
        histogram.stddev(),
        histogram.percentile(99.9),
    ) {
        Some((mean, max, stddev, ninety_ninth))
    } else {
        None
    }
}

pub fn hamming_distance(s1: &[u8], s2: &[u8]) -> u32 {
    s1.iter()
        .zip(s2.iter())
        .map(|(a, b)| (a ^ b).count_ones())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::star::Grid;

    #[test]
    fn repeating_xor() {
        let grid = Grid::new(vec![
            vec![0x00, 0x00, 0x00, 0x00],
            vec![0x00, 0x00, 0x00],
            vec![0x00, 0x00, 0x00],
        ]);
        let r = xor_repeating_grid(&grid, &[0x01, 0x02, 0x03]);
        let exp = Grid::new(vec![
            vec![0x01, 0x02, 0x03, 0x01],
            vec![0x02, 0x03, 0x01],
            vec![0x02, 0x03, 0x01],
        ]);
        assert_eq!(*r, *exp);
        let r = xor_repeating_grid(&exp, &[0x01, 0x02, 0x03]);
        assert_eq!(*r, *grid);
    }

    #[test]
    fn hamming_distance_test() {
        let input = b"this is a test";
        let input2 = b"wokka wokka!!!";
        assert_eq!(hamming_distance(input, input2), 37);
    }

    #[test]
    fn test_transpose() {
        let grid = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a];
        let t = transpose(&grid, 3);
        let exp = vec![vec![1, 4, 7, 10], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(t, exp);
    }
}
