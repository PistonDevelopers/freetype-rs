
extern crate freetype;

use freetype as ft;
use freetype::outline::{Line, Bezier2, Bezier3, Curve};

fn show_curve(curve: Curve) {
    match curve {
        Line(pt) => println!("L {} {}", pt.x, -pt.y),
        Bezier2(pt1, pt2) => { println!("Q {} {} {} {}", pt1.x, -pt1.y, pt2.x, -pt2.y) }
        Bezier3(pt1, pt2, pt3) => println!("C {} {} {} {} {} {}",
                                           pt1.x, -pt1.y,
                                           pt2.x, -pt2.y,
                                           pt3.x, -pt3.y),
    }
}

fn main() {
    let args = std::os::args();
    if args.len() != 3 {
        println!("usage: {} font character", &args[0]);
        return;
    }

    let ref filename = args[1];
    let ref text = args[2];

    let library = ft::Library::init().unwrap();
    let mut face = library.new_face(filename.as_slice(), 0).unwrap();
    face.set_char_size(40 * 64, 0, 50, 0).unwrap();
    face.load_char(text.as_bytes()[0] as u64, ft::face::NO_SCALE).unwrap();

    let metrics = &face.glyph().raw().metrics;
    let xmin = metrics.horiBearingX - 5;
    let width = metrics.width + 10;
    let ymin = -metrics.horiBearingY - 5;
    let height = metrics.height + 10;

    let outline = face.glyph().outline().unwrap();

    println!("<?xml version=\"1.0\" standalone=\"no\"?>");
    println!("<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\"");
    println!("\"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">");
    println!("<svg viewBox=\"{} {} {} {}\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">",
             xmin, ymin, width, height);

    for mut contour in outline.contours_iter() {
        let start = contour.start();
        println!("<path fill=\"none\" stroke=\"black\" stroke-width=\"1\" d=\"M {} {}", start.x, -start.y);
        for curve in contour {
            show_curve(curve);
        }
        println!("Z \" />");
    }
    println!("</svg>");
}
