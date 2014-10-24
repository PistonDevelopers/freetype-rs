use ffi;
use std::{mem, raw};
use libc::{c_short, c_char};

pub use ffi::FT_Vector as Vector;

pub enum Curve<'a> {
    Line(Vector),
    Bezier2(Vector, Vector),
    Bezier3(Vector, Vector, Vector),
}

pub struct Outline<'a> {
    raw: &'a ffi::FT_Outline,
}

impl<'a> Outline<'a> {
    pub unsafe fn from_raw(raw: &'a ffi::FT_Outline) -> Outline<'a> {
        Outline { raw: raw }
    }

    pub fn points(&self) -> &'a [Vector] {
        unsafe { mem::transmute(raw::Slice { data: self.raw.points, len: self.raw.n_points as uint }) }
    }

    pub fn tags(&self) -> &'a [c_char] {
        unsafe { mem::transmute(raw::Slice { data: self.raw.tags, len: self.raw.n_points as uint }) }
    }

    pub fn contours(&self) -> &'a [c_short] {
        unsafe { mem::transmute(raw::Slice { data: self.raw.contours, len: self.raw.n_contours as uint }) }
    }

    pub fn contours_iter(&self) -> ContourIterator<'a> {
        unsafe { ContourIterator::from_raw(self.raw) }
    }
}

const TAG_ONCURVE: c_char = 0x01;
const TAG_BEZIER3: c_char = 0x02;

pub struct CurveIterator<'a> {
    start_point: *const Vector,
    start_tag: *const c_char,
    idx: int,
    length: int,
}

impl<'a> CurveIterator<'a> {
    pub unsafe fn from_raw<'a>(outline: &'a ffi::FT_Outline,
                               start_idx: int,
                               end_idx: int) -> CurveIterator<'a> {
        CurveIterator {
            start_point: outline.points.offset(start_idx),
            start_tag: outline.tags.offset(start_idx),
            idx: 0,
            length: end_idx - start_idx + 1,
        }
    }

    pub fn start(&self) -> &'a Vector {
        unsafe { mem::transmute(self.start_point) }
    }

    // Retrieves the point at offset i from the current point. Note that contours implicitly repeat their
    // first point at the end.
    unsafe fn pt(&self, i: int) -> Vector {
        if self.idx + i < self.length {
            *self.start_point.offset(self.idx + i)
        } else {
            *self.start_point
        }
    }

    unsafe fn tg(&self, i: int) -> c_char {
        if self.idx + i < self.length {
            *self.start_tag.offset(self.idx + i)
        } else {
            *self.start_tag
        }
    }
}

impl<'a> Iterator<Curve<'a>> for CurveIterator<'a> {
    fn next(&mut self) -> Option<Curve<'a>> {
        if self.idx >= self.length {
            None
        } else {
            unsafe {
                let tag1 = self.tg(1);

                let (shift, curve) = if (tag1 & TAG_ONCURVE) == TAG_ONCURVE {
                    (1, Line(self.pt(1)))
                } else if (tag1 & TAG_BEZIER3) == TAG_BEZIER3 {
                    (3, Bezier3(self.pt(1), self.pt(2), self.pt(3)))
                } else {
                    // We are some kind of quadratic Bezier.
                    // Quadratic Bezier curves have a special treatment in TTF outlines:
                    // as an optimization, curves are often constructed from sequences
                    // of off-curve control points. In this case, there are implied on-curve
                    // points in between each pair of off-curve points.
                    if (self.tg(2) & TAG_ONCURVE) == TAG_ONCURVE {
                        (2, Bezier2(self.pt(1), self.pt(2)))
                    } else {
                        (1, Bezier2(self.pt(1), (self.pt(1) + self.pt(2))/2))
                    }
                };

                self.idx += shift;
                Some(curve)
            }
        }
    }
}

pub struct ContourIterator<'a> {
    outline: &'a ffi::FT_Outline,
    contour_start: c_short,
    contour_end_idx: *const c_short,
    last_end_idx: *const c_short,
}

impl<'a> ContourIterator<'a> {
    pub unsafe fn from_raw<'a>(outline: &'a ffi::FT_Outline) -> ContourIterator<'a> {
        ContourIterator {
            outline: outline,
            contour_start: 0,
            contour_end_idx: outline.contours,
            last_end_idx: outline.contours.offset(outline.n_contours as int - 1),
        }
    }
}

impl<'a> Iterator<CurveIterator<'a>> for ContourIterator<'a> {
    fn next(&mut self) -> Option<CurveIterator<'a>> {
        if self.contour_end_idx > self.last_end_idx {
            None
        } else {
            unsafe {
                let contour_end = *self.contour_end_idx;
                let curves = CurveIterator::from_raw(self.outline, self.contour_start as int, contour_end as int);

                self.contour_start = contour_end + 1;
                self.contour_end_idx = self.contour_end_idx.offset(1);

                Some(curves)
            }
        }
    }
}


