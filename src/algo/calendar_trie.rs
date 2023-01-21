use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct FlattenedCalendarBlock {
    pub block: CalendarBlock,
    pub stack_position: usize,
}

pub struct CalendarTrie {
    root_idx: NodeIndex,
    adjacency: Graph<Uuid, usize>,
    id_to_block_map: HashMap<Uuid, CalendarBlock>,
}

impl CalendarTrie {
    pub fn new() -> Self {
        let mut id_to_block_map = HashMap::new();

        let root_node = CalendarBlock {
            id: Uuid::new_v4(),
            start_minute: 0,
            end_minute: 1440,
            block_type: CalendarBlockType::Wrapper,
            subtree_depth: 0,
        };

        id_to_block_map.insert(root_node.id, root_node);

        let mut adjacency_map = Graph::new();
        let root = adjacency_map.add_node(root_node.id);

        Self {
            root_idx: root,
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

            match block.does_overlap(*current_block) {
                Some(o) => Some((o, forward_n_idx)),
                None => None,
            }
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
        let mut traversal_queue: VecDeque<(NodeIndex, usize)> =
            VecDeque::with_capacity(self.id_to_block_map.iter().len());

        let mut buffer: Vec<(NodeIndex, usize)> =
            Vec::with_capacity(self.id_to_block_map.iter().len());

        traversal_queue.push_back((self.root_idx, 0));

        while !traversal_queue.is_empty() {
            let (node_idx, stack_position) = traversal_queue.pop_front().unwrap();
            buffer.push((node_idx, stack_position));

            let forward_neighbors = self
                .adjacency
                .edges_directed(node_idx, petgraph::Direction::Outgoing)
                .map(|e| e.target());

            forward_neighbors.for_each(|n| {
                traversal_queue.push_back((n, stack_position + 1));
            });
        }

        buffer
            .iter()
            .map(|(node_idx, stack_position)| {
                let current_block_id = self.adjacency[*node_idx];
                let current_block = self.id_to_block_map.get(&current_block_id).unwrap();
                return FlattenedCalendarBlock {
                    block: *current_block,
                    stack_position: *stack_position,
                };
            })
            .collect()
    }
}
