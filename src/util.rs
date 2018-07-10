pub fn value_from_grid(i: u32, res: u32, val_range: (f32, f32)) -> f32 {
    // Used for iterating over grids; correlate an index to a value.
    (i as f32 / res as f32) * (val_range.1 - val_range.0) - (val_range.1 - val_range.0) / 2.
}

//pub fn<T>to_vec(array: T) -> Vec<T> {
//
//}