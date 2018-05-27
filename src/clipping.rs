// Functions for clipping; Cohen-Sutherland algorithm, from:
// https://en.wikipedia.org/wiki/Cohen%E2%80%93Sutherland_algorithm

// This algorithm clips post-projection, ie on the 2d screen.

use ndarray::prelude::*;

use types::{Pt2D, Camera};

const INSIDE: i16 = 0;
const LEFT: i16 = 1;
const RIGHT: i16 = 2;
const BOTTOM: i16 = 4;
const TOP: i16 = 8;
const BACK: i16 = 16;
const FORWARD: i16 = 32;
const EARTH: i16 = 64;
const SKY: i16 = 128;

fn compute_outcode_2d(pt: &Pt2D, x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> i16 {
    // Initialised as being inside of clip window
    let mut code = INSIDE;

    if pt.x < x_min {
        code |= LEFT;
    }
    else if pt.x > x_max {
        code |= RIGHT;
    }
    if pt.y < y_min {
        code |= BOTTOM;
    }
    else if pt.y > y_max {
        code |= TOP;
    }
    code
}

fn compute_outcode_3d(
    pt: &Array1<f64>,
    x_min: f64, x_max: f64,
    y_min: f64, y_max: f64,
    z_min: f64, z_max: f64,
) -> i16 {

    let mut code = compute_outcode_2d(&Pt2D {x: pt[0], y: pt[1]}, x_min, x_max, y_min, y_max);

    if pt[2] < z_min {
        code |= BACK;
    }
    else if pt[2] > z_max {
        code |= FORWARD;
    }
    code
}

fn compute_outcode_4d(
    pt: &Array1<f64>,
    x_min: f64, x_max: f64,
    y_min: f64, y_max: f64,
    z_min: f64, z_max: f64,
    u_min: f64, u_max: f64
) -> i16 {

    let mut code = compute_outcode_3d(pt, x_min, x_max, y_min, y_max, z_min, z_max);

    if pt[3] < u_min {
        code |= EARTH;
    }
    else if pt[3] > u_max {
        code |= SKY;
    }
    code
}

pub fn cohen_sutherland_2d(pt_0: &Pt2D, pt_1: &Pt2D, x_min: f64, x_max: f64,
                           y_min: f64, y_max: f64) -> Option<(Pt2D, Pt2D)> {
    // Cohen–Sutherland clipping algorithm clips a line from
    // P0 = (x0, y0) to P1 = (x1, y1) against a rectangle with 
    // diagonal from (xmin, ymin) to (xmax, ymax).

    let mut outcode_0 = compute_outcode_2d(&pt_0, x_min, x_max, y_min, y_max);
    let mut outcode_1 = compute_outcode_2d(&pt_1, x_min, x_max, y_min, y_max);

    let mut x_0 = pt_0.x;
    let mut y_0 = pt_0.y;
    let mut x_1 = pt_1.x;
    let mut y_1 = pt_1.y;

    loop {
        if outcode_0 | outcode_1 == 0 {
            // bitwise OR is 0: both points inside window; trivially accept and exit loop
            return Some((Pt2D {x: x_0, y: y_0}, Pt2D {x: x_1, y: y_1}))
        } else if outcode_0 & outcode_1 > 0 {
            // bitwise AND is not 0: both points share an outside zone (LEFT, RIGHT, TOP,
            // or BOTTOM), so both must be outside window; return None.
            return None
        } else {
            let x: f64;
            let y: f64;
            // At least one endpoint is outside the clip rectangle; pick it.
            let outcode_out = if outcode_0 > 0 { outcode_0 } else { outcode_1 };

            // Now find the intersection point;
            // use formulas:
            //   slope = (y1 - y0) / (x1 - x0)
            //   x = x0 + (1 / slope) * (ym bv - y0), where ym is ymin or ymax
            //   y = y0 + slope * (xm - x0), where xm is xmin or xmax
            // No need to worry about divide-by-zero because, in each case, the
            // outcode bit being tested guarantees the denominator is non-zero
            if outcode_out & TOP > 0 {
                // point is above the clip window
                x = x_0 + (x_1 - x_0) * (y_max - y_0) / (y_1 - y_0);
                y = y_max;
            } else if outcode_out & BOTTOM > 0 {
                // point is below the clip window
                x = x_0 + (x_1 - x_0) * (y_min - y_0) / (y_1 - y_0);
                y = y_min;
            } else if outcode_out & RIGHT > 0 {
                // point is to the right of the clip window
                y = y_0 + (y_1 - y_0) * (x_max - x_0) / (x_1 - x_0);
                x = x_max;
            } else {
                // point is to the left of the clip window
                y = y_0 + (y_1 - y_0) * (x_min - x_0) / (x_1 - x_0);
                x = x_min;
            }

            // Now we move outside point to intersection point to clip
            // and get ready for next pass.
            if outcode_out == outcode_0 {
                x_0 = x;
                y_0 = y;
                outcode_0 = compute_outcode_2d(&Pt2D {x: x_0, y: y_0}, x_min, x_max, y_min, y_max);
            } else {
                x_1 = x;
                y_1 = y;
                outcode_1 = compute_outcode_2d(&Pt2D {x: x_1, y: y_1}, x_min, x_max, y_min, y_max);
            }
        }
    }
}

pub fn cohen_sutherland_3d(cam: &Camera, line: (Array1<f64>, Array1<f64>)) ->
        Option<(Array1<f64>, Array1<f64>)> {
    let (x_min, x_max, y_min, y_max, z_min, z_max) = (-1., 1., -1., 1., -1., 1.);

    let mut outcode_0 = compute_outcode_3d(&line.0, x_min, x_max, y_min, y_max, z_min, z_max);
    let mut outcode_1 = compute_outcode_3d(&line.1, x_min, x_max, y_min, y_max, z_min, z_max);

    let mut x_0 = line.0[0];
    let mut y_0 = line.0[1];
    let mut z_0 = line.0[2];
    let mut x_1 = line.1[0];
    let mut y_1 = line.1[1];
    let mut z_1 = line.1[2];

    loop {
        if outcode_0 | outcode_1 == 0 {
            // bitwise OR is 0: both points inside window; trivially accept and exit loop
            return Some((array![x_0, y_0, z_0], array![x_1, y_1, z_1]));
        } else if outcode_0 & outcode_1 > 0 {
            // bitwise AND is not 0: both points share an outside zone so both
            // must be outside window; return None. Trivial reject.
            return None
        } else {
            // todo temp
            return Some((array![x_0, y_0, z_0], array![x_1, y_1, z_1]));

            let x: f64;
            let y: f64;
            let z: f64;
            let t: f64;
            // At least one endpoint is outside the clip rectangle; pick it.
            let outcode_out = if outcode_0 > 0 { outcode_0 } else { outcode_1 };

            // See notes in 2d cohen-sutherland function.
            if outcode_out & TOP > 0 {
                // point is above the clip window
                t = (z_0 - y_0) / ((x_1 - x_0) - (z_1 - z_0));
                x = x_0 + t * (x_1 - x_0);
                y = y_max;
                z = z_0 + t * (z_1 - z_0);
            } else if outcode_out & BOTTOM > 0 {
                // point is below the clip window
                t = (z_0 - x_0) / ((x_1 - x_0) - (z_1 - z_0));
                x = x_0 + t * (x_1 - x_0);
                y = y_min;
                z = z_0 + t * (z_1 - z_0);
            } else if outcode_out & RIGHT > 0 {
                // point is to the right of the clip window
                t = (z_0 - y_0) / ((y_1 - y_0) - (z_1 - z_0));
                x = x_max;
                y = y_0 + t * (y_1 - y_0);
                z = z_0 + t * (z_1 - z_0);
            } else if outcode_out & LEFT > 0 {
                // point is to the left of the clip window
                t = (z_0 - y_0) / ((y_1 - y_0) - (z_1 - z_0));
                x = x_min;
                y = y_0 + t * (y_1 - y_0);
                z = z_0 + t * (z_1 - z_0);
            } else if outcode_out & FORWARD > 0 {
                // point is to the right of the clip window
                t = (x_0 - y_0) / ((y_1 - y_0) - (x_1 - x_0));
                x = x_0 + t * (x_1 - x_0);
                y = y_0 + t * (y_1 - y_0);
                z = z_max;
            } else {
                // point is in the back of the clip window
                t = (x_0 - y_0) / ((y_1 - y_0) - (x_1 - x_0));
                x = x_0 + t * (x_1 - x_0);
                y = y_0 + t * (y_1 - y_0);
                z = z_min;
            }

            // Now we move outside point to intersection point to clip
            // and get ready for next pass.
            if outcode_out == outcode_0 {
                outcode_0 = compute_outcode_3d(
                    &array![x, y, z],
                    x_min, x_max, y_min, y_max, z_min, z_max
                );
            } else {
                outcode_1 = compute_outcode_3d(
                    &array![x, y, z],
                    x_min, x_max, y_min, y_max, z_min, z_max
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_1() {
        let pt_0 = (32., -11.);
        let pt_1 = (0., -1.2);

        let expected = 0;

        assert_eq!(clipping::clip_and_draw(pt_0[0], pt_0[1], pt_1[0], pt_1[1]), expected);
    }

    #[test]
    fn outcodes() {
        let pt_0 = (32., -11.);
        let pt_1 = (0., -1.2);

        let expected = 0;

        assert_eq!(clipping::clip_and_draw(pt_0[0], pt_0[1], pt_1[0], pt_1[1]), expected);
    }
}