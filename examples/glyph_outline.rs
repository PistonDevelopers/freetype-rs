#![feature(exit_status)]

extern crate "freetype" as ft;

use std::io::prelude::*;

use std::io::stderr;
use std::path::Path;
use std::env::{ args, set_exit_status };
use ft::outline::Curve;

fn show_curve(curve: Curve) {
    match curve {
        Curve::Line(pt) =>
            println!("L {} {}", pt.x, -pt.y),

        Curve::Bezier2(pt1, pt2) =>
            println!("Q {} {} {} {}", pt1.x, -pt1.y, pt2.x, -pt2.y),

        Curve::Bezier3(pt1, pt2, pt3) =>
            println!("C {} {} {} {} {} {}", pt1.x, -pt1.y,
                                            pt2.x, -pt2.y,
                                            pt3.x, -pt3.y),
    }
}

fn main() {
    let ref mut stderr = stderr();
    let ref mut args = args();
    if let (3, _) = args.size_hint() {}
    else {
        let exe = args.next().unwrap();
        let _ = writeln!(stderr, "Usage: {} font character", exe);
        set_exit_status(1);
        return
    }

    let filename = args.nth(1).unwrap();
    let text = args.next().unwrap();

    let library = ft::Library::init().unwrap();
    let face = library.new_face(&Path::new(&filename), 0).unwrap();
    face.set_char_size(40 * 64, 0, 50, 0).unwrap();
    face.load_char(text.chars().next().unwrap() as usize, ft::face::NO_SCALE).unwrap();

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

    for contour in outline.contours_iter() {
        let start = contour.start();
        println!("<path fill=\"none\" stroke=\"black\" stroke-width=\"1\" d=\"M {} {}", start.x, -start.y);
        for curve in contour {
            show_curve(curve);
        }
        println!("Z \" />");
    }
    println!("</svg>");
}
