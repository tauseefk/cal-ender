use crate::prelude::*;

pub fn get_position_offsets(stack_position: usize) -> (f64, f64) {
    let stack_position = stack_position as f64;
    match stack_position < 1.0 {
        true => (0.0, MAX_COL_WIDTH),
        false => {
            let stack_separation = stack_position * BLOCK_STACK_PADDING;
            let left_offset =
                MAX_COL_WIDTH - MAX_COL_WIDTH / (stack_position - 1.0) + stack_separation;
            let width = MAX_COL_WIDTH / (stack_position - 1.0) - 2.0 * stack_separation;
            (left_offset, width)
        }
    }
}

pub fn get_g_transforms(stack_position: usize, subtree_depth: usize) -> (f64, f64) {
    let stack_position = stack_position as f64;
    let subtree_depth = subtree_depth as f64;

    let width_divisor = stack_position + subtree_depth;

    match stack_position < 1.0 {
        true => (0.0, MAX_COL_WIDTH),
        false => {
            let width = match subtree_depth > 0.0 {
                true => 1.8 / width_divisor,
                false => 1.0 / width_divisor,
            };

            return ((stack_position - 1.0) / width_divisor, width);
        }
    }
}
