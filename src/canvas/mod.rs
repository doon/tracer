use crate::base;

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<base::Tuple>>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        let pixels = vec![vec![base::Tuple::color(0.0, 0.0, 0.0); width]; height];
        Canvas {
            width,
            height,
            pixels,
        }
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: base::Tuple) {
        self.pixels[y][x] = color;
    }

    fn pixel_at(&self, x: usize, y: usize) -> &base::Tuple {
        &self.pixels[y][x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for (_y, row) in c.pixels.iter().enumerate() {
            for (_x, pixel) in row.iter().enumerate() {
                assert_eq!(pixel, &base::Tuple::color(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn writing_pixel_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = base::Tuple::color(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), &red);
    }
}
