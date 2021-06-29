// use std::io;
use crate::color::*;
// use rand::Rng;

pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

pub struct Canvas {
    pub dimensions: Dimensions,
    pub pixels: Vec<u8>,
}

pub enum ImageType {
    PNG,
    PPM,
}

pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas {
        pixels: vec![0; width * height * 3],
        dimensions: Dimensions { width, height },
    }
}

impl Canvas {
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), String> {
        let Dimensions { width, .. } = self.dimensions;

        if self.within_bounds(x, y) {
            let Color { r, g, b } = color;
            let i = (width * y + x) * 3;
            self.pixels[i] = self.rgb_to_u8(r);
            self.pixels[i + 1] = self.rgb_to_u8(g);
            self.pixels[i + 2] = self.rgb_to_u8(b);
        } else {
            println!("write_pixel received out of bounds inputs ({},{})", x, y);
        }
        Ok(())
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        let Canvas { pixels, dimensions } = self;
        let Dimensions { width, .. } = dimensions;

        match self.within_bounds(x, y) {
            true => {
                let i = (width * y + x) * 3;
                return Color {
                    r: pixels[i] as f32 / 255.,
                    g: pixels[i + 1] as f32 / 255.,
                    b: pixels[i + 2] as f32 / 255.,
                };
            }
            false => panic!("Received out of bounds inputs ({},{})", x, y),
        }
    }

    fn rgb_to_u8(&self, c: f32) -> u8 {
        (255. * c).round().clamp(0., 255.) as u8
    }

    #[allow(unused_comparisons)]
    fn within_bounds(&self, x: usize, y: usize) -> bool {
        let Dimensions { width, height } = self.dimensions;
        if x <= width - 1 && y <= height - 1 && y >= 0 && x >= 0 {
            true
        } else {
            false
        }
    }

    pub fn height(&self) -> usize {
        self.dimensions.height
    }

    pub fn width(&self) -> usize {
        self.dimensions.width
    }

    pub fn save(&self, image_type: ImageType, filename: &str) {
        match image_type {
            ImageType::PPM => render::save_as_ppm(&self, filename),
            _ => panic!("ImageType not yet supported"),
        }
    }

    pub fn canvas_to_ppm(&self) -> String {
        return render::canvas_to_ppm(&self);
    }
}

pub mod render {
    use super::{Canvas, Dimensions};
    use std::fs::File;
    use std::io::{self, Write};
    use std::path::Path;

    pub fn save_as(filepath: &String, data: String) {
        match write_file(filepath, data) {
            Ok(()) => println!("Writing to file was successful"),
            Err(ex) => println!("Something went wrong: {}", ex),
        }
    }

    pub fn save_as_ppm(canvas: &Canvas, filename: &str) {
        let data = canvas_to_ppm(canvas);
        let path = format!("./images/{}.ppm", filename);
        save_as(&path, data);
    }

    pub fn canvas_to_ppm(canvas: &Canvas) -> String {
        let values: Vec<String> = stringify(&canvas.pixels);
        return build_ppm(canvas, values);
    }

    fn build_ppm(canvas: &Canvas, values: Vec<String>) -> String {
        let mut image_data: Vec<String> = vec![];
        image_data.push(ppm_header(&canvas)); // header
        let mut result = chunk(values, &mut image_data); //body
        result.push_str("\n"); // required for ending ppm files

        return result;
    }

    fn stringify(values: &Vec<u8>) -> Vec<String> {
        values.iter().map(|n| n.to_string()).collect()
    }

    fn ppm_header(canvas: &Canvas) -> String {
        let Dimensions { width, height } = canvas.dimensions;
        [
            "P3".to_string(),
            format!("{} {}", width, height),
            "255".to_string(),
        ]
        .join("\n")
    }

    fn chunk(source: Vec<String>, output: &mut Vec<String>) -> String {
        for chunk in source.chunks(15) {
            output.push(chunk.join(" "));
        }
        return output.join("\n");
    }

    fn write_file(name: &String, data: String) -> io::Result<()> {
        let path = Path::new(name);
        let mut file = File::create(path)?;
        writeln!(file, "{}", data)?;
        Ok(())
    }
}
