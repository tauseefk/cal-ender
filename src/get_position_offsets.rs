use crate::prelude::*;

pub fn get_position_offsets(stack_position: usize) -> (String, String) {
    let stack_position = stack_position as f64;
    match stack_position < 1.0 {
        true => (format!("{}", 0), format!("{}px", MAX_COL_WIDTH)),
        false => {
            let stack_separation = stack_position * BLOCK_STACK_PADDING;
            let left_offset =
                MAX_COL_WIDTH - MAX_COL_WIDTH / (stack_position - 1.0) + stack_separation;
            let width = MAX_COL_WIDTH / (stack_position - 1.0) - 2.0 * stack_separation;
            (format!("{}px", left_offset), format!("{}px", width))
        }
    }
}

pub fn get_g_transforms(stack_position: usize, subtree_depth: usize) -> (String, String) {
    let stack_position = stack_position as f64;
    let subtree_depth = subtree_depth as f64;

    let width_divisor = stack_position + subtree_depth;

    match stack_position < 1.0 {
        true => (0.to_string(), MAX_COL_WIDTH.to_string()),
        false => {
            let width = match subtree_depth > 0.0 {
                true => 1.8 / width_divisor,
                false => 1.0 / width_divisor,
            };

            return (
                format!("calc(100% * {})", (stack_position - 1.0) / width_divisor),
                format!("calc(100% * {})", width),
            );
        }
    }
}
