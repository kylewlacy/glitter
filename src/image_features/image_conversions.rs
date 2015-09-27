use image;
use image_data;

impl<P: image::Pixel<Subpixel=u8>> From<P> for image_data::Pixel {
    fn from(pixel: P) -> image_data::Pixel {
        let rgba = pixel.to_rgba();
        let (r, g, b, a) = (rgba[0], rgba[1], rgba[2], rgba[3]);
        image_data::Pixel::r_g_b_a(r, g, b, a)
    }
}

impl<'a, I, P> From<I> for image_data::Pixels
    where I: image::GenericImage<Pixel=P>,
          P: image::Pixel<Subpixel=u8>
{
    fn from(img: I) -> image_data::Pixels {
        let (w, h) = img.dimensions();
        let mut pixels = image_data::Pixels::new(w as usize, h as usize);
        for (x, y, pixel) in img.pixels() {
            pixels[x as usize][y as usize] = pixel.into();
        }

        pixels
    }
}
