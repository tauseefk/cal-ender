use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FlattenedCalendarBlock {
    pub block: CalendarBlock,
    pub stack_position: usize,
    pub subtree_height: usize,
}

pub struct CalendarBlockTree {
    root_idx: NodeIndex,
    adjacency: Graph<Uuid, usize>,
    id_to_block_map: HashMap<Uuid, CalendarBlock>,
}

impl CalendarBlockTree {
    pub fn new() -> Self {
        let mut id_to_block_map = HashMap::new();

        let root_node = CalendarBlock {
            id: Uuid::new_v4(),
            start_minute: 0,
            end_minute: 1440,
            block_type: CalendarBlockType::Wrapper,
            subtree_depth: 0,
            label: String::from("Today's Date"),
        };

        id_to_block_map.insert(root_node.id, root_node.clone());

        let mut adjacency_map = Graph::new();
        let root_idx = adjacency_map.add_node(root_node.id);

        Self {
            root_idx,
            adjacency: adjacency_map,
            id_to_block_map,
        }
    }

    pub fn add(
        &mut self,
        block: CalendarBlock,
        destination: Option<NodeIndex>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Recursive Add
        // 1. find overlaps
        //      if no overlap
        //          add edge from destination to new block
        //          return
        //      else if new block gets swallowed
        //          call add with new destination
        // TODO:else
        //          add edge from destination to new block
        //          add edges from new block to overlapping blocks
        //          remove edges from destination to overlapping blocks

        let destination = destination.unwrap_or(self.root_idx);

        let mut forward_neighbors = self
            .adjacency
            .edges_directed(destination, petgraph::Direction::Outgoing)
            .map(|e| e.target());

        let adjacency_list = &mut self.adjacency.clone();

        let overlap = forward_neighbors.find_map(|forward_n_idx| {
            let current_block = adjacency_list[forward_n_idx];
            let current_block = self.id_to_block_map.get(&current_block).unwrap();

            block
                .does_overlap(current_block.clone())
                .map(|o| (o, forward_n_idx))
        });

        let _ = match overlap {
            Some((_, node_idx)) => self.add(block, Some(node_idx)),
            None => {
                let idx = adjacency_list.add_node(block.id);
                adjacency_list.add_edge(destination, idx, 1);
                self.adjacency = adjacency_list.clone();

                self.update_subtree_depth_until_root(destination, 1);
                self.id_to_block_map.insert(block.id, block);
                Ok(())
            }
        };

        Ok(())
    }

    fn update_subtree_depth_until_root(&mut self, node_idx: NodeIndex, value: usize) {
        let node_id = self.adjacency[node_idx];
        let maybe_node = self.id_to_block_map.get_mut(&node_id);

        if let Some(node) = maybe_node {
            if node.subtree_depth < value {
                node.subtree_depth = value;

                let mut parent = self
                    .adjacency
                    .edges_directed(node_idx, petgraph::Direction::Incoming)
                    .map(|e| e.source());
                if let Some(p) = parent.next() {
                    self.update_subtree_depth_until_root(p, value + 1);
                }
            }
        }
    }

    pub fn display(&self) {
        info!("{}", Dot::new(&self.adjacency));
    }

    pub fn traverse(&self) -> Vec<FlattenedCalendarBlock> {
        let mut traversal_queue: VecDeque<(NodeIndex, usize, usize)> =
            VecDeque::with_capacity(self.id_to_block_map.iter().len());

        let mut buffer: Vec<(NodeIndex, usize, usize)> =
            Vec::with_capacity(self.id_to_block_map.iter().len());

        traversal_queue.push_back((self.root_idx, 0, 0));

        while !traversal_queue.is_empty() {
            let (node_idx, stack_position, subtree_height) = traversal_queue.pop_front().unwrap();
            buffer.push((node_idx, stack_position, subtree_height));

            let forward_neighbors = self
                .adjacency
                .edges_directed(node_idx, petgraph::Direction::Outgoing)
                .map(|e| e.target());

            forward_neighbors.for_each(|n| {
                // Cluster tops are the root's direct children; a top's subtree_depth is the
                // longest chain in its cluster, so its subtree height is 1 + subtree_depth.
                // Deeper nodes inherit their cluster top's height, so a node's column grid
                // never depends on its own depth.
                let child_subtree_height = if stack_position == 0 {
                    let block_id = self.adjacency[n];
                    1 + self.id_to_block_map.get(&block_id).unwrap().subtree_depth
                } else {
                    subtree_height
                };
                traversal_queue.push_back((n, stack_position + 1, child_subtree_height));
            });
        }

        buffer
            .iter()
            .map(|(node_idx, stack_position, subtree_height)| {
                let current_block_id = self.adjacency[*node_idx];
                let current_block = self.id_to_block_map.get(&current_block_id).unwrap();
                FlattenedCalendarBlock {
                    block: current_block.clone(),
                    stack_position: *stack_position,
                    subtree_height: *subtree_height,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The default dataset from `main.rs` (`app`'s `calendar_blocks`), including the same
    /// `start/end - BLOCK_TOP_OFFSET` minutes and the same `sort_by` ordering, so the tree
    /// built here matches what the app builds.
    fn default_blocks() -> Vec<CalendarBlock> {
        let mut blocks = vec![
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 530 - BLOCK_TOP_OFFSET,
                end_minute: 830 - BLOCK_TOP_OFFSET,
                block_type: CalendarBlockType::Available,
                subtree_depth: 0,
                label: String::from("Available"),
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 550 - BLOCK_TOP_OFFSET,
                end_minute: 590 - BLOCK_TOP_OFFSET,
                block_type: CalendarBlockType::Busy,
                subtree_depth: 0,
                label: String::from("Shower"),
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 550 - BLOCK_TOP_OFFSET,
                end_minute: 580 - BLOCK_TOP_OFFSET,
                block_type: CalendarBlockType::Busy,
                subtree_depth: 0,
                label: String::from("Shower Thoughts"),
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 605 - BLOCK_TOP_OFFSET,
                end_minute: 665 - BLOCK_TOP_OFFSET,
                block_type: CalendarBlockType::Busy,
                subtree_depth: 0,
                label: String::from("Coffee"),
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 605 - BLOCK_TOP_OFFSET,
                end_minute: 630 - BLOCK_TOP_OFFSET,
                block_type: CalendarBlockType::Busy,
                subtree_depth: 0,
                label: String::from("Brew"),
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 635 - BLOCK_TOP_OFFSET,
                end_minute: 710 - BLOCK_TOP_OFFSET,
                block_type: CalendarBlockType::Busy,
                subtree_depth: 0,
                label: String::from("Contemplation"),
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 650 - BLOCK_TOP_OFFSET,
                end_minute: 830 - BLOCK_TOP_OFFSET,
                block_type: CalendarBlockType::Busy,
                subtree_depth: 0,
                label: String::from("Code"),
            },
        ];

        blocks.sort_by(|a, b| {
            if a.start_minute < b.start_minute
                || a.start_minute == b.start_minute && a.end_minute >= b.end_minute
            {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        blocks
    }

    fn build_default_tree() -> CalendarBlockTree {
        let mut tree = CalendarBlockTree::new();
        for block in default_blocks() {
            let _ = tree.add(block, None);
        }
        tree
    }

    fn flattened_by_label() -> HashMap<String, FlattenedCalendarBlock> {
        build_default_tree()
            .traverse()
            .into_iter()
            .map(|f| (f.block.label.clone(), f))
            .collect()
    }

    #[test]
    fn traverse_default_structure() {
        let by_label = flattened_by_label();

        // (stack_position, subtree_depth, subtree_height) per block. The single synthetic
        // root has stack_position 0; the cluster height is 1 + Available.subtree_depth = 4.
        let expected: &[(&str, usize, usize, usize)] = &[
            ("Available", 1, 3, 4),
            ("Shower", 2, 1, 4),
            ("Shower Thoughts", 3, 0, 4),
            ("Coffee", 2, 2, 4),
            ("Brew", 3, 0, 4),
            ("Contemplation", 3, 1, 4),
            ("Code", 4, 0, 4),
        ];

        for (label, s, sd, height) in expected {
            let f = by_label
                .get(*label)
                .unwrap_or_else(|| panic!("missing block: {label}"));
            assert_eq!(
                (f.stack_position, f.block.subtree_depth, f.subtree_height),
                (*s, *sd, *height),
                "block {label}: (stack_position, subtree_depth, subtree_height)"
            );
        }

        // The wrapper root is the only node at stack_position 0.
        let root = by_label
            .values()
            .find(|f| f.block.block_type == CalendarBlockType::Wrapper)
            .expect("wrapper root present");
        assert_eq!(root.stack_position, 0);
    }

    #[test]
    fn brew_and_shower_thoughts_have_identical_inputs() {
        // Brew (shallow sibling of a deeper branch) and Shower Thoughts (a nested-only
        // leaf) share `(stack_position, subtree_depth, subtree_height)`, so both render to
        // the same span [50%, 100%]. That is the intended result, not a defect: both are
        // leaves filling to the right edge.
        let by_label = flattened_by_label();
        let brew = &by_label["Brew"];
        let st = &by_label["Shower Thoughts"];

        assert_eq!(
            (brew.stack_position, brew.block.subtree_depth, brew.subtree_height),
            (st.stack_position, st.block.subtree_depth, st.subtree_height),
        );
    }
}
