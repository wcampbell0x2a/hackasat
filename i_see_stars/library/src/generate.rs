pub fn image(grid: &[Vec<u8>], iteration: u8) {
    let mut imgbuf = image::RgbImage::new(129, 129);
    for (i, line) in grid.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            imgbuf.put_pixel(j as u32, i as u32, image::Rgb([*val, *val, *val]));
        }
    }
    imgbuf.save(&format!("{}.png", iteration)).unwrap();
}
