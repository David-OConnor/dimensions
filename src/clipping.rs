// Functions for clipping; Cohen-Sutherland algorithm, from:
// https://en.wikipedia.org/wiki/Cohen%E2%80%93Sutherland_algorithm

// This algorithm clips post-projection, ie on the 2d screen.

use ndarray::prelude::*;

use types::{Pt2D, Pt3D, Camera, Node};

const INSIDE: i8 = 0;
const LEFT: i8 = 1;
const RIGHT: i8 = 2;
const BOTTOM: i8 = 4;
const TOP: i8 = 8;

struct Frustum {
    // 3d frustrum.
    // eg FUL for Far Up Left. Front means high z coord.
    FUL: Array1<f64>,
    FUR: Array1<f64>,
    FDL: Array1<f64>,
    FDR: Array1<f64>,

    NUL: Array1<f64>,
    NUR: Array1<f64>,
    NDL: Array1<f64>,
    NDR: Array1<f64>,
}


fn compute_outcode(pt: &Pt2D, x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> i8 {
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

pub fn clip(pt_0: &Pt2D, pt_1: &Pt2D, x_min: f64, x_max: f64, 
            y_min: f64, y_max: f64) -> Option<(Pt2D, Pt2D)> {
    // Cohen–Sutherland clipping algorithm clips a line from
    // P0 = (x0, y0) to P1 = (x1, y1) against a rectangle with 
    // diagonal from (xmin, ymin) to (xmax, ymax).
    
    let mut outcode_0 = compute_outcode(&pt_0, x_min, x_max, y_min, y_max);
    let mut outcode_1 = compute_outcode(&pt_1, x_min, x_max, y_min, y_max);

    let x_0: f64;
    let y_0: f64;
    let x_1: f64;
    let y_1: f64;

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
                outcode_0 = compute_outcode(&Pt2D {x: x_0, y: y_0}, x_min, x_max, y_min, y_max);
            } else {
                x_1 = x;
                y_1 = y;
                outcode_1 = compute_outcode(&Pt2D {x: x_1, y: y_1}, x_min, x_max, y_min, y_max);
            }
        }
    }
}

fn cross(a: Array1<f64>, b: Array1<f64>) -> Array1<f64> {
    // Calculate the cross product between two vectors... Why isn't this
    // included with the ndarray crate?
    assert![a.len() == 3 && b.len() == 3];
    array![a[1]*b[2] - a[2]*b[1], a[2]*b[0] - a[0]*b[2], a[0]*b[1] - a[1]*b[0]]
}

fn find_intersection_3d(
    line: (Array1<f64>, Array1<f64>),
                        plane: (Array1<f64>, Array1<f64>, Array1<f64>, Array1<f64>)
    ) -> Option<Array1<f64>> {

    // The normal vector is perpendicular to two vectors we create from our plane.
    // We don't need the fourth point.
    let norm_vec = cross(
        array![plane.1[0] - plane.0[0], plane.1[1] - plane.0[1], plane.1[2] - plane.0[2]],
        array![plane.2[0] - plane.0[0], plane.2[1] - plane.0[1], plane.2[2] - plane.0[2]]
    );

    // todo remove unit_norm if we don't need normalized
    let unit_norm = norm_vec / (norm_vec[0].powi(2) + norm_vec[1].powi(2) +
        norm_vec[2].powi(2)).sqrt();

    let parallel_check = unit_norm.dot(&(line[1] - line[0]));
    if parallel_check == 0. { // todo check floating point math isn't fucking you.
        // No intersection if the line and planes are parallel.
        return None;
    }
        return Some
}

fn clip_to_frustrum_3d(fm: &Frustum, line: (Array1<f64>, Array1<f64>)) ->
        (Array1<f64>, Array1<f64>) {
    let planes = vec![
        // Left
        (fm.FUL, fm.FDL, fm.NUL, fm.NDL),
        // Right
        (fm.FUR, fm.FDR, fm.NUR, fm.NDR),
        // Top
        (fm.FUL, fm.FUR, fm.NUL, fm.NUR),
        // Bottom
        (fm.FDL, fm.FDR, fm.NDL, fm.NDR),
        // Front
        (fm.FUL, fm.FUR, fm.FDL, fm.FDR),
        // Near
        (fm.NUL, fm.NUR, fm.NDL, fm.NDR)
    ];

    for plane in &planes {
        let intersection = find_intersection_3d(&line, plane);
    }
}

fn _make_frustrum(cam: &Camera) -> _Frustum {
    // todo as method for camera? m for frustum???
    let (far_width, far_height) = cam.view_size(true);
    let (near_width, near_height) = cam.view_size(false);
    
    let FUL = array![-far_width / 2., far_height / 2., cam.clip_far];
    let FUR = array![far_width / 2., far_height / 2., cam.clip_far];
    let FDL = array![-far_width / 2., -far_height / 2., cam.clip_far];
    let FDR = array![far_width / 2., -far_height / 2., cam.clip_far];

    let NUL = array![-near_width / 2., near_height / 2., cam.clip_near];
    let NUR = array![near_width / 2., near_height / 2., cam.clip_near];
    let NDL = array![-near_width / 2., -near_height / 2., cam.clip_near];
    let NDR = array![near_width / 2., -near_height / 2., cam.clip_near];

    _Frustrum {FUL, FUR, FDL, FDR, NUL, NUR, NDL, NDR}
}

fn _normalize_to_frustrum(shifted_node: Node) {
    // Using the frustrum's slope, calculate a simulated point location, along
    // with simulated min and max values for clipping.
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