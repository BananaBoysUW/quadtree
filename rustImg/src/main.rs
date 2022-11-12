use std::io::Result;
use image::{io::Reader as ImageReader, GenericImageView, RgbImage};

fn main() -> Result<()> {
    let img = ImageReader::open("apple.jpeg")?.decode().expect("could not decode image");

    let (w, h) = img.dimensions();
    println!("w: {} h: {}", w, h);

    // let (rectw, recth) = (12u8, 12u8);
    let (rectw, recth) = (6u8, 6u8);
    let mut new_img = RgbImage::new(w / (rectw as u32), h / (recth as u32));

    for i in 0..w {
        for j in 0..h {
            let p = img.get_pixel(i, j);

            let np = new_img.get_pixel_mut(i / (rectw as u32), j / (recth as u32));
            for g in 0..3 {
                np.0[g] += p.0[g] / (rectw * recth);
            }
        }
    }

    new_img.save("new.jpg").expect("could not write image");


    Ok(())
}
