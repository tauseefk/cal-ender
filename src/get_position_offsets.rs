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

/// Returns the CSS `transform` offsets for a block.
///
/// The block's left edge as a fraction of the container
/// left_fraction: f64,
/// One column's width as a fraction of the container
/// column_fraction: f64,
/// Whether the block is a leaf (i.e., spans the full container width)
/// is_leaf: bool,
///
pub fn get_subtree_depth_transforms(
    left_fraction: f64,
    column_fraction: f64,
    is_leaf: bool,
) -> (String, String) {
    let width = match is_leaf {
        // leaf: fill from its own left edge to the container's right edge
        true => 1.0 - left_fraction,
        // non-leaf: one column plus a half-column overlap into its child
        false => 1.5 * column_fraction,
    };

    (
        format!("calc(100% * {left_fraction})"),
        format!("calc(100% * {width})"),
    )
}
