use ray_tracer::canvas::*;
use ray_tracer::color::*;

#[test]
fn test_canvas() {
    let width = 10;
    let height = 20;
    let c = canvas(width, height);
    
    assert_eq!(c.width(), width);
    assert_eq!(c.height(), height);
    assert_eq!(c.pixels, vec![0; width*height*3]);
}


#[test]
fn test_pixel_at() {
    let mut c = canvas(10, 20);
    let pixel_color = color(1., 0., 0.);
    c.write_pixel(2, 3, pixel_color).unwrap();

    assert_eq!(c.pixel_at(2, 3), pixel_color);
}

#[test]
fn test_ppm_header() {
    let c = canvas(5, 3);
    let expected_output=  "\
    P3\n\
    5 3\n\
    255\n\
    0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
    0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
    0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n";

    assert_eq!(c.canvas_to_ppm(), expected_output);
}

#[test]
fn test_ppm_body() {
    let mut c = canvas(5, 3);
    c.write_pixel(0, 0, color(1.5, 0., 0.)).unwrap();
    c.write_pixel(2, 1, color(0., 0.5, 0.)).unwrap();
    c.write_pixel(4, 2, color(-0.5, 0., 1.)).unwrap();
    let expected_output=  "\
    P3\n\
    5 3\n\
    255\n\
    255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
    0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
    0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";

    assert_eq!(c.canvas_to_ppm(), expected_output);
}

#[test]
fn test_long_lines_in_ppm() {
    let mut c = canvas(10, 2);
    let width = c.width();
    let pixel_color = color(1.0, 0.8, 0.6);
    for i in 0..c.pixels.len() {
        let x = (i / 3) % width;
        let y = (i / 3) / width;
        c.write_pixel(x, y, pixel_color).unwrap();
    }

    let expected_output=  "\
    P3\n\
    10 2\n\
    255\n\
    255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n\
    255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n\
    255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n\
    255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n";

    assert_eq!(c.canvas_to_ppm(), expected_output);
}
