use histogram::Histogram;

pub fn xor_grid(grid: &[Vec<u8>], key: u8) -> Vec<Vec<u8>> {
    grid.iter()
        .map(|a| a.iter().map(|b| b ^ key).collect())
        .collect()
}

pub fn find_xor_key(grid: &[Vec<u8>]) -> Option<u8> {
    for (key, xor_bytes) in (0u8..0xff).map(|key| (key, xor_grid(grid, key))) {
        if let Some((mean, max, stddev, ninety_ninth)) = frequency_num(&xor_bytes) {
            if max == 255 && mean < 10 && ninety_ninth == 255 {
                println!("mean: {}, max: {}, stddev: {}, ninety_ninth: {}", mean, max, stddev, ninety_ninth);
                return Some(key);
            }
        }
    }
    None
}

pub fn frequency_num(grid: &[Vec<u8>]) -> Option<(u64, u64, u64, u64)> {
    let mut histogram = Histogram::new();
    for line in grid.iter() {
        for val in line.iter() {
            histogram.increment(*val as u64).unwrap();
        }
    }

    if let (Ok(mean), Ok(max), Some(stddev), Ok(ninety_ninth)) =
        (histogram.mean(), histogram.maximum(), histogram.stddev(), histogram.percentile(99.9))
    {
        Some((mean, max, stddev, ninety_ninth))
    } else {
        None
    }
}
