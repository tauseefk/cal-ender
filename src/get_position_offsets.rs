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

    // Divide the row by the cluster's subtree height so siblings share one column grid; a
    // node off the longest path no longer shrinks itself by its own (shorter) depth.
    let height = subtree_height as f64;

    match stack_position < 1.0 {
        true => (0.to_string(), MAX_COL_WIDTH.to_string()),
        false => {
            let width = match subtree_depth > 0.0 {
                // non-leaf: cover its own subtree, filling behind its children
                true => (subtree_depth + 1.0) / height,
                // leaf: fill from its left edge to the right edge
                false => (height - stack_position + 1.0) / height,
            };

            (
                format!("calc(100% * {})", (stack_position - 1.0) / height),
                format!("calc(100% * {width})"),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Expected transform from explicit `numerator/denominator` fractions, so each case
    /// reads as intent rather than re-deriving the function's arithmetic.
    fn transform(left_num: f64, width_num: f64, denominator: f64) -> (String, String) {
        (
            format!("calc(100% * {})", left_num / denominator),
            format!("calc(100% * {})", width_num / denominator),
        )
    }

    #[test]
    fn transforms_for_default_blocks() {
        // (stack_position, subtree_depth, subtree_height) per block of the default dataset,
        // matching `calendar_tree::tests::traverse_default_structure`. height = 4.
        // left = (s-1)/H; non-leaf width = (t+1)/H; leaf width = (H-s+1)/H.
        let h = 4.0;

        // Available: s=1, t=3, non-leaf -> left 0/4, width 4/4 (fills, covers subtree).
        assert_eq!(get_subtree_depth_transforms(1, 3, 4), transform(0.0, 4.0, h));
        // Shower: s=2, t=1, non-leaf -> left 1/4, width 2/4.
        assert_eq!(get_subtree_depth_transforms(2, 1, 4), transform(1.0, 2.0, h));
        // Coffee: s=2, t=2, non-leaf -> left 1/4, width 3/4.
        assert_eq!(get_subtree_depth_transforms(2, 2, 4), transform(1.0, 3.0, h));
        // Contemplation: s=3, t=1, non-leaf -> left 2/4, width 2/4.
        assert_eq!(get_subtree_depth_transforms(3, 1, 4), transform(2.0, 2.0, h));
        // Code: s=4, t=0, leaf -> left 3/4, width (4-4+1)/4 = 1/4.
        assert_eq!(get_subtree_depth_transforms(4, 0, 4), transform(3.0, 1.0, h));
        // Brew & Shower Thoughts: s=3, t=0, leaf -> left 2/4, width (4-3+1)/4 = 2/4.
        assert_eq!(get_subtree_depth_transforms(3, 0, 4), transform(2.0, 2.0, h));
    }

    #[test]
    fn sibling_leaves_share_transform() {
        // Brew and Shower Thoughts share (stack_position, subtree_depth, subtree_height) =
        // (3, 0, 4), so they intentionally render to the same span [50%, 100%]. Both are
        // leaves filling to the right edge; sharing a transform is correct, not a defect.
        assert_eq!(
            get_subtree_depth_transforms(3, 0, 4),
            transform(2.0, 2.0, 4.0),
        );
    }
}
