use crate::prelude::*;

pub fn get_position_offsets(stack_position: usize) -> (String, String) {
    let stack_position = stack_position as f64;
    match stack_position < 1.0 {
        true => (format!("{}", 0), format!("{MAX_COL_WIDTH}px")),
        false => {
            let stack_separation = stack_position * BLOCK_STACK_PADDING;
            let left_offset =
                MAX_COL_WIDTH - MAX_COL_WIDTH / (stack_position - 1.0) + stack_separation;
            let width = MAX_COL_WIDTH / (stack_position - 1.0) - 2.0 * stack_separation;
            (format!("{left_offset}px"), format!("{width}px"))
        }
    }
}

pub fn get_subtree_depth_transforms(
    stack_position: usize,
    subtree_depth: usize,
    subtree_height: usize,
) -> (String, String) {
    let stack_position = stack_position as f64;
    let subtree_depth = subtree_depth as f64;

    // height is used to ensure that siblings on different-depth
    // branches stay horizontally aligned
    let height = subtree_height as f64;

    match stack_position < 1.0 {
        true => (0.to_string(), MAX_COL_WIDTH.to_string()),
        false => {
            let width = match subtree_depth > 0.0 {
                // non-leaf: one column plus a half-column overlap into its child
                true => 1.5 / height,
                // leaf: fill from its own column to the container's right edge
                false => (height - stack_position + 1.0) / height,
            };

            (
                format!("calc(100% * {})", (stack_position - 1.0) / height),
                format!("calc(100% * {width})"),
            )
        }
    }
}
