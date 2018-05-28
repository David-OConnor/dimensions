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
            //   x = x0 + (1 / slope) * (ym - y0), where ym is ymin or ymax
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

fn line_plane_intersection(norm: Array1<f64>, plane_pt: Array1<f64>,
                           line: (Array1<f64>, Array1<f64>)) -> Option<Array1<f64>> {
    // 3d only.
    let ϵ = 1e-4;

    let line_direction = line.1.clone() - line.0.clone();
    let normal_dotted = norm.dot(&line_direction);
    if normal_dotted.abs() < ϵ {
//        return None;
        // todo address this kludge.
        return Some(array![999., 999., 999.]);
    }

    let w = line.0.clone() - plane_pt.clone();
    let si = -norm.dot(&w) / normal_dotted;
    Some(w + si * line_direction + plane_pt)
}

fn inside_frustum(pt: &Array1<f64>) -> bool {
    // Assume a clipspace -1 to +1 limit on all axes.  3d only.
    let (min, max) = (-1., 1.);
    if min < pt[0] && pt[0] < max && min < pt[1] && pt[1] < max && min < pt[2] && pt[2] < max {
        return true
    }
    false
}

fn dist_3d(pt0: &Array1<f64>, pt1: &Array1<f64>) -> f64 {
    let diff = pt1 - pt0;
    (diff[0].powi(2) + diff[1].powi(2) + diff[2].powi(2)).sqrt()
}

pub fn clip_3d(line: (Array1<f64>, Array1<f64>)) -> Option<(Array1<f64>, Array1<f64>)> {
    // Struggling on Cohen Sutherland for 3d; rolling my own algo.

    // Clip to a clipspace frustum, bounded by -1 and +1 on each axis.
    let (min, max) = (-1., 1.);

    // Assign to variables to make subsequent calcs easier to read.
    let (mut x0, mut y0, mut z0) = (line.0[0], line.0[1], line.0[2]);
    let (mut x1, mut y1, mut z1) = (line.1[0], line.1[1], line.1[2]);

    let (pt0_inside, pt1_inside) = (inside_frustum(&line.0), inside_frustum(&line.1));

    // Both points are inside the clipspace frustum; trivially accept.
    if pt0_inside && pt1_inside {
        return Some(line.clone())
    }
    // Both points are outside the same frustum plane; trivially reject.
    else if (x0 < min && x1 < min) || (x0 > max && x1 > max) ||
        (y0 < min && y1 < min) || (y0 > max && y1 > max) ||
        (z0 < min && z1 < min) || (z0 > max && z1 > max) {
        return None
    }
    // Otherwise, we need to clip one or both points, non-trivially.
    let line_3d = (
        array![line.0[0], line.0[1], line.0[2]],
        array![line.1[0], line.1[1], line.1[2]]
    );

    let intersections = vec![
        line_plane_intersection(
            array![0., 1., 0.], array![0., 1., 0.], line_3d.clone()
        ).unwrap(),
        line_plane_intersection(
            array![0., -1., 0.], array![0., -1., 0.], line_3d.clone()
        ).unwrap(),
        line_plane_intersection(
            array![1., 0., 0.], array![1., 0., 0.], line_3d.clone()
        ).unwrap(),
        line_plane_intersection(
            array![-1., 0., 0.], array![-1., 0., 0.], line_3d.clone()
        ).unwrap(),
        line_plane_intersection(
            array![0., 0., 1.], array![0., 0., 1.], line_3d.clone()
        ).unwrap(),
        line_plane_intersection(
            array![0., 0., -1.], array![0., 0., -1.], line_3d.clone()
        ).unwrap()
    ];

    // Only two of the calculated intersections should be contained within the
    // planes; these are our new endpoints.
    const ϵ: f64 = 0.1;  // todo re-attack.
    // todo maybe we just take the lowest two plane intersections.

    let mut clipped_ends = Vec::new();
    for intersection in &intersections {
        if intersection[0].abs() <= 1. + ϵ && intersection[1].abs() <= 1. + ϵ &&
            intersection[2].abs() <= 1. + ϵ {
            clipped_ends.push(intersection);
            if clipped_ends.len() >= 2 { break };
        }
    }
    // todo analyze how you handle this; it theroetrically should work, but it's
    // todo getting scrweed up by edge cases.
    println!("INTS: T{} B{} R{} L{} F{} B{}\n\n", &intersections[0], &intersections[1],
    &intersections[2], &intersections[3], &intersections[4], &intersections[5]);
//    assert_eq![clipped_ends.len(), 2];
    if clipped_ends.len() < 2 {
        return None;  // todo fix!
    }

    let clipped0;
    let clipped1;
    // Go with withever end is closer to the non-inside point.
    if pt0_inside {  // We clip pt2 only.
        clipped0 = line.0;
        let dist0 = dist_3d(&line_3d.1, &clipped_ends[0]);
        let dist1 = dist_3d(&line_3d.1, &clipped_ends[1]);
        clipped1 = if dist0 < dist1 { clipped_ends[0].clone() } else
            { clipped_ends[1].clone() };
    } else if pt1_inside {  // We clip pt1 only
        clipped1 = line.1;
        let dist0 = dist_3d(&line_3d.0, &clipped_ends[0]);
        let dist1 = dist_3d(&line_3d.0, &clipped_ends[1]);
        clipped0 = if dist0 < dist1 { clipped_ends[0].clone() } else
            { clipped_ends[1].clone() };
    } else {
        // We clip both points. Must keep the initial order.
        // todo not ordered!
         clipped0 = clipped_ends[0].clone();
         clipped1 = clipped_ends[0].clone();
    }

    let result0 = array![clipped0[0], clipped0[1], clipped0[2], 1.];
    let result1 = array![clipped1[0], clipped1[1], clipped1[2], 1.];

    Some((result0, result1))
}


pub fn cohen_sutherland_3d(line: (Array1<f64>, Array1<f64>)) ->
        Option<(Array1<f64>, Array1<f64>)> {
    // Clip to a "unit" (-1 to +1 on each axis) frustum.
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
            // bitwise OR is 0: both points inside window; trivially accept and
            // exit the loop.
            return Some((array![x_0, y_0, z_0], array![x_1, y_1, z_1]));
        } else if outcode_0 & outcode_1 > 0 {
            // bitwise AND is not 0: both points share an outside zone, so the
            // line won't intersect the frustum; trivially reject.
            return None
        } else {
            let x: f64;
            let y: f64;
            let z: f64;
            // At least one endpoint is outside the clip rectangle; pick it.
            let outcode_out = if outcode_0 > 0 { outcode_0 } else { outcode_1 };

            //
            //x = x_0 + (x_1 - x_0) * (y_min - y_0) / (y_1 - y_0);
            //
            // See notes in 2d cohen-sutherland function.  The calculations
            // follow this pattern: x = x_0 + dx/dy * dy, etc.
            if outcode_out & TOP > 0 {
                // point is above the clip window
                let clip_ratio = (y_1 - y_0) / (y_max - y_0);
                x = x_0 + (x_1 - x_0) / clip_ratio;
                y = y_max;
                z = z_0 + (z_1 - z_0) / clip_ratio;
            } else if outcode_out & BOTTOM > 0 {
                // point is below the clip window
                let clip_ratio = (y_1 - y_0) / (y_min - y_0);
                x = x_0 + (x_1 - x_0) / clip_ratio;
                y = y_min;
                z = z_0 + (z_1 - z_0) / clip_ratio;
            } else if outcode_out & RIGHT > 0 {
                // point is to the right of the clip window
                let clip_ratio = (x_1 - x_0) / (x_max - x_0);
                x = x_max;
                y = y_0 + (y_1 - y_0) / clip_ratio;
                z = z_0 + (z_1 - z_0) / clip_ratio;
            } else if outcode_out & LEFT > 0 {
                // point is to the left of the clip window
                let clip_ratio = (x_1 - x_0) / (x_min - x_0);
                x = x_min;
                y = y_0 + (y_1 - y_0) / clip_ratio;
                z = z_0 + (z_1 - z_0) / clip_ratio;
            } else if outcode_out & FORWARD > 0 {
                // point is to the foward part of the clip window
                let clip_ratio = (z_1 - z_0) / (z_max - z_0);
                x = x_0 + (x_1 - x_0) / clip_ratio;
                y = y_0 + (y_1 - y_0) / clip_ratio;
                z = z_max;
            } else {
                // point is in the back of the clip window
                let clip_ratio = (z_1 - z_0) / (z_min - z_0);
                x = x_0 + (x_1 - x_0) / clip_ratio;
                y = y_0 + (y_1 - y_0) / clip_ratio;
                z = z_min;
            }

            // Now we move outside point to intersection point to clip
            // and get ready for next pass.
            if outcode_out == outcode_0 {
                x_0 = x;
                y_0 = y;
                z_0 = z;
                outcode_0 = compute_outcode_3d(
                    &array![x_0, y_0, z_0],
                    x_min, x_max, y_min, y_max, z_min, z_max
                );
            } else {
                x_1 = x;
                y_1 = y;
                z_1 = z;
                outcode_1 = compute_outcode_3d(
                    &array![x_1, y_1, z_1],
                    x_min, x_max, y_min, y_max, z_min, z_max
                );
            }
        }
    }
}

pub fn cohen_sutherland_4d(cam: &Camera, line: (Array1<f64>, Array1<f64>)) ->
        Option<(Array1<f64>, Array1<f64>)> {
    // Clip to a "unit" (-1 to +1 on each axis) hyperfrustum.
    let (x_min, x_max, y_min, y_max, z_min, z_max, u_min, u_max) =
        (-1., 1., -1., 1., -1., 1., -1., 1.);

    let mut outcode_0 = compute_outcode_4d(
        &line.0, x_min, x_max, y_min, y_max, z_min, z_max, u_min, u_max
    );
    let mut outcode_1 = compute_outcode_4d(
        &line.1, x_min, x_max, y_min, y_max, z_min, z_max, u_min, u_max
    );

    let mut x_0 = line.0[0];
    let mut y_0 = line.0[1];
    let mut z_0 = line.0[2];
    let mut u_0 = line.0[3];
    let mut x_1 = line.1[0];
    let mut y_1 = line.1[1];
    let mut z_1 = line.1[2];
    let mut u_1 = line.1[3];
//    return Some((array![x_0, y_0, z_0, u_0], array![x_1, y_1, z_1, u_1]));
    loop {
        if outcode_0 | outcode_1 == 0 {
            // bitwise OR is 0: both points inside window; trivially accept and
            // exit the loop.
            return Some((array![x_0, y_0, z_0, u_0], array![x_1, y_1, z_1, u_1]));
        } else if outcode_0 & outcode_1 > 0 {
            // bitwise AND is not 0: both points share an outside zone, so the
            // line won't intersect the frustum; trivially reject.
            return None
        } else {
            let x: f64;
            let y: f64;
            let z: f64;
            let u: f64;
            // At least one endpoint is outside the clip rectangle; pick it.
            let outcode_out = if outcode_0 > 0 { outcode_0 } else { outcode_1 };

            // See notes in 2d cohen-sutherland function.  The calculations
            // follow this pattern: x = x_0 + dx/dy * dy, etc.
            if outcode_out & TOP > 0 {
                // point is above the clip window
                let clip_ratio = (y_1 - y_0) / (y_max - y_0);
                x = x_0 + (x_1 - x_0) / clip_ratio;
                y = y_max;
                z = z_0 + (z_1 - z_0) / clip_ratio;
                u = u_0 + (u_1 - u_0) / clip_ratio;
            } else if outcode_out & BOTTOM > 0 {
                // point is below the clip window
                let clip_ratio = (y_1 - y_0) / (y_min - y_0);
                x = x_0 + (x_1 - x_0) / clip_ratio;
                y = y_min;
                z = z_0 + (z_1 - z_0) / clip_ratio;
                u = u_0 + (u_1 - u_0) / clip_ratio;
            } else if outcode_out & RIGHT > 0 {
                // point is to the right of the clip window
                let clip_ratio = (x_1 - x_0) / (x_max - x_0);
                x = x_max;
                y = y_0 + (y_1 - y_0) / clip_ratio;
                z = z_0 + (z_1 - z_0) / clip_ratio;
                u = u_0 + (u_1 - u_0) / clip_ratio;
            } else if outcode_out & LEFT > 0 {
                // point is to the left of the clip window
                let clip_ratio = (x_1 - x_0) / (x_min - x_0);
                x = x_min;
                y = y_0 + (y_1 - y_0) / clip_ratio;
                z = z_0 + (z_1 - z_0) / clip_ratio;
                u = u_0 + (u_1 - u_0) / clip_ratio;
            } else if outcode_out & FORWARD > 0 {
                // point is to the foward part of the clip window
                let clip_ratio = (z_1 - z_0) / (z_max - z_0);
                x = x_0 + (x_1 - x_0) / clip_ratio;
                y = y_0 + (y_1 - y_0) / clip_ratio;
                z = z_max;
                u = u_0 + (u_1 - u_0) / clip_ratio;
            } else if outcode_out & BACK > 0 {
                // point is in the back of the clip window
                let clip_ratio = (z_1 - z_0) / (z_min - z_0);
                x = x_0 + (x_1 - x_0) / clip_ratio;
                y = y_0 + (y_1 - y_0) / clip_ratio;
                z = z_min;
                u = u_0 + (u_1 - u_0) / clip_ratio;
            } else if outcode_out & SKY > 0 {
                // point is in the sky of the clip window
                let clip_ratio = (u_1 - u_0) / (u_max - u_0);
                x = x_0 + (x_1 - x_0) / clip_ratio;
                y = y_0 + (y_1 - y_0) / clip_ratio;
                z = z_0 + (z_1 - z_0) / clip_ratio;
                u = u_max;
            } else {
                // point is in the earth of the clip window
                let clip_ratio = (u_1 - u_0) * (u_min - u_0);
                x = x_0 + (x_1 - x_0) / clip_ratio;
                y = y_0 + (y_1 - y_0) / clip_ratio;
                z = z_0 + (z_1 - z_0) / clip_ratio;
                u = u_min;
            }

            // Now we move outside point to intersection point to clip
            // and get ready for next pass.
            if outcode_out == outcode_0 {
                x_0 = x;
                y_0 = y;
                z_0 = z;
                u_0 = u;
                outcode_0 = compute_outcode_4d(
                    &array![x_0, y_0, z_0, u_0],
                    x_min, x_max, y_min, y_max, z_min, z_max, u_min, u_max
                );
            } else {
                x_1 = x;
                y_1 = y;
                z_1 = z;
                u_0 = u;
                outcode_1 = compute_outcode_4d(
                    &array![x_1, y_1, z_1, u_1],
                    x_min, x_max, y_min, y_max, z_min, z_max, u_min, u_max
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcodes() {
        // By testing outcodes in 4d, we implicitly test the 3 and 2d functions.
        let pt_0 = array![1.5, -0.2, 0., 1.3];  // Right, sky
        let pt_1 = array![-3.5, -0.2, -2.2, 2.4];  // Left, back, sky
        let pt_2 = array![0.5, -1.2, -0.3, 0.8];  // Down
        let pt_3 = array![0.99, -0.99, -0.1, -0.3];  // Inside

        assert_eq!(compute_outcode_4d(&pt_0, -1., 1., -1., 1., -1., 1., -1., 1.), 130);
        assert_eq!(compute_outcode_4d(&pt_1, -1., 1., -1., 1., -1., 1., -1., 1.), 145);
        assert_eq!(compute_outcode_4d(&pt_2, -1., 1., -1., 1., -1., 1., -1., 1.), 4);
        assert_eq!(compute_outcode_4d(&pt_3, -1., 1., -1., 1., -1., 1., -1., 1.), 0);
    }

    #[test]
    fn plane_intersection() {
        let epsilon = 1e-2;
        let line = (array![0.8, 1.3, -0.7], array![-0.9, -1.2, 0.6]);
        let expected = array![0.596, 1.0, -0.544];

        let actual = line_plane_intersection(
            array![0., 1., 0.], array![0., 1., 0.], line
        ).unwrap();

        assert!((expected[0] - actual[0]).abs() < epsilon);
        assert!((expected[1] - actual[1]).abs() < epsilon);
        assert!((expected[2] - actual[2]).abs() < epsilon);
    }
}