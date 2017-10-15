extern crate freetype as ft;

fn draw_curve(curve: ft::outline::Curve) {
    match curve {
        ft::outline::Curve::Line(pt) =>
            println!("L {} {}", pt.x, -pt.y),
        ft::outline::Curve::Bezier2(pt1, pt2) =>
            println!("Q {} {} {} {}", pt1.x, -pt1.y, pt2.x, -pt2.y),
        ft::outline::Curve::Bezier3(pt1, pt2, pt3) =>
            println!("C {} {} {} {} {} {}", pt1.x, -pt1.y,
                                            pt2.x, -pt2.y,
                                            pt3.x, -pt3.y)
    }
}

fn main() {
    let ref mut args = std::env::args();

    if args.len() != 3 {
        let exe = args.next().unwrap();
        println!("Usage: {} font character", exe);
        return
    }

    let ref font = args.nth(1).unwrap();
    let character = args.next().and_then(|s| s.chars().next()).unwrap() as usize;
    let library = ft::Library::init().unwrap();
    let face = library.new_face(font, 0).unwrap();

    face.set_char_size(40 * 64, 0, 50, 0).unwrap();
    face.load_char(character, ft::face::LoadFlag::NO_SCALE).unwrap();

    let glyph = face.glyph();
    let metrics = glyph.metrics();
    let xmin = metrics.horiBearingX - 5;
    let width = metrics.width + 10;
    let ymin = -metrics.horiBearingY - 5;
    let height = metrics.height + 10;
    let outline = glyph.outline().unwrap();

    println!("<?xml version=\"1.0\" standalone=\"no\"?>");
    println!("<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\"");
    println!("\"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">");
    println!("<svg viewBox=\"{} {} {} {}\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">",
             xmin, ymin, width, height);

    for contour in outline.contours_iter() {
        let start = contour.start();
        println!("<path fill=\"none\" stroke=\"black\" stroke-width=\"1\" d=\"M {} {}", start.x, -start.y);
        for curve in contour {
            draw_curve(curve);
        }
        println!("Z \" />");
    }
    println!("</svg>");
}
