extern crate image;

pub fn write_swatch(colors: &[(u8, u8, u8)], out_file: &String) -> std::io::Result<()>{
    let imgx: u32 = colors.len() as u32;
    let imgy = 100;

    let mut swatch = image::ImageBuffer::new(imgx, imgy);

    for i in 0..imgx {
        for j in 0..imgy {
            let pixel = swatch.get_pixel_mut(i, j);
            *pixel = image::Rgb([colors[i as usize].0, colors[i as usize].1, colors[i as usize].2])
        }
    }

    swatch.save(out_file).unwrap();
    Ok(())
}
