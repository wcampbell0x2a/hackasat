pub fn image(grid: &[Vec<u8>], iteration: u8) {
    let mut imgbuf = image::RgbImage::new(129, 129);
    for (i, line) in grid.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            imgbuf.put_pixel(i as u32, j as u32, image::Rgb([*val, 0, 0]));
        }
    }
    imgbuf.save(&format!("{}.png", iteration)).unwrap();
}
