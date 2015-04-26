use std::slice;
use std::marker::PhantomData;
use libc::{ c_short, c_char };
use { ffi, Vector };

#[derive(Copy, Clone)]
pub enum Curve {
    Line(Vector),
    Bezier2(Vector, Vector),
    Bezier3(Vector, Vector, Vector)
}

pub struct Outline<'a> {
    raw: &'a ffi::FT_Outline
}

impl<'a> Outline<'a> {
    pub unsafe fn from_raw(raw: &'a ffi::FT_Outline) -> Self {
        Outline {
            raw: raw
        }
    }

    pub fn points(&self) -> &'a [Vector] {
        unsafe {
            slice::from_raw_parts(self.raw.points, self.raw.n_points as usize)
        }
    }

    pub fn tags(&self) -> &'a [c_char] {
        unsafe {
            slice::from_raw_parts(self.raw.tags, self.raw.n_points as usize)
        }
    }

    pub fn contours(&self) -> &'a [c_short] {
        unsafe {
            slice::from_raw_parts(self.raw.contours, self.raw.n_contours as usize)
        }
    }

    pub fn contours_iter(&self) -> ContourIterator<'a> {
        unsafe {
            ContourIterator::from_raw(self.raw)
        }
    }
}

const TAG_ONCURVE: c_char = 0x01;
const TAG_BEZIER3: c_char = 0x02;

pub struct CurveIterator<'a> {
    start_point: *const Vector,
    start_tag: *const c_char,
    idx: isize,
    length: isize,
    marker: PhantomData<&'a ()>
}

impl<'a> CurveIterator<'a> {
    pub unsafe fn from_raw(outline: &'a ffi::FT_Outline,
                               start_idx: isize,
                               end_idx: isize) -> Self {
        CurveIterator {
            start_point: outline.points.offset(start_idx),
            start_tag: outline.tags.offset(start_idx),
            idx: 0,
            length: end_idx - start_idx + 1,
            marker: PhantomData
        }
    }

    pub fn start(&self) -> &'a Vector {
        unsafe {
            &*self.start_point
        }
    }

    // Retrieves the point at offset i from the current point. Note that contours implicitly repeat their
    // first point at the end.
    unsafe fn pt(&self, i: isize) -> Vector {
        if self.idx + i < self.length {
            *self.start_point.offset(self.idx + i)
        } else {
            *self.start_point
        }
    }

    unsafe fn tg(&self, i: isize) -> c_char {
        if self.idx + i < self.length {
            *self.start_tag.offset(self.idx + i)
        } else {
            *self.start_tag
        }
    }
}

impl<'a> Iterator for CurveIterator<'a> {
    type Item = Curve;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.length {
            None
        } else {
            unsafe {
                let tag1 = self.tg(1);

                let (shift, curve) = if (tag1 & TAG_ONCURVE) == TAG_ONCURVE {
                    (1, Curve::Line(self.pt(1)))
                } else if (tag1 & TAG_BEZIER3) == TAG_BEZIER3 {
                    (3, Curve::Bezier3(self.pt(1), self.pt(2), self.pt(3)))
                } else {
                    // We are some kind of quadratic Bezier.
                    // Quadratic Bezier curves have a special treatment in TTF outlines:
                    // as an optimization, curves are often constructed from sequences
                    // of off-curve control points. In this case, there are implied on-curve
                    // points in between each pair of off-curve points.
                    if (self.tg(2) & TAG_ONCURVE) == TAG_ONCURVE {
                        (2, Curve::Bezier2(self.pt(1), self.pt(2)))
                    } else {
                        let pt = ffi::FT_Vector {
                            x: (self.pt(1).x + self.pt(2).x) / 2,
                            y: (self.pt(1).y + self.pt(2).y) / 2,
                        };

                        (1, Curve::Bezier2(self.pt(1), pt))
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
    last_end_idx: *const c_short
}

impl<'a> ContourIterator<'a> {
    pub unsafe fn from_raw(outline: &'a ffi::FT_Outline) -> Self {
        ContourIterator {
            outline: outline,
            contour_start: 0,
            contour_end_idx: outline.contours,
            last_end_idx: outline.contours.offset(outline.n_contours as isize - 1)
        }
    }
}

impl<'a> Iterator for ContourIterator<'a> {
    type Item = CurveIterator<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.contour_end_idx > self.last_end_idx {
            None
        } else {
            unsafe {
                let contour_end = *self.contour_end_idx;
                let curves = CurveIterator::from_raw(self.outline, self.contour_start as isize,
                                                     contour_end as isize);
                self.contour_start = contour_end + 1;
                self.contour_end_idx = self.contour_end_idx.offset(1);

                Some(curves)
            }
        }
    }
}
