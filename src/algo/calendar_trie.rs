use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct FlattenedCalendarBlock {
    pub block: CalendarBlock,
    pub stack_position: usize,
    pub subtree_depth: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GraphEdgeType {
    Forward, // towards the root
    Backward,
}
impl Display for GraphEdgeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphEdgeType::Forward => write!(f, "Forward"),
            GraphEdgeType::Backward => write!(f, "Backward"),
        }
    }
}

pub struct CalendarTrie {
    root_idx: NodeIndex,
    adjacency: Graph<Uuid, GraphEdgeType>,
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
        let mut destination = destination.unwrap_or(self.root_idx);

        let mut keep_going = true;
        let mut overlapping_blocks: Vec<NodeIndex> = vec![];

        while keep_going {
            let mut forward_neighbors = self.adjacency.neighbors(destination).filter(|n| {
                let edge = self
                    .adjacency
                    .edges_connecting(destination, *n)
                    .find(|e| *e.weight() == GraphEdgeType::Forward);

                match edge {
                    Some(_) => true,
                    None => false,
                }
            });

            let adjacency_list = &mut self.adjacency.clone();

            match forward_neighbors.next() {
                Some(n) => {
                    let current_block = adjacency_list[n];
                    let current_block = self.id_to_block_map.get(&current_block).unwrap();

                    match block.does_overlap(*current_block) {
                        Some(o) => {
                            match o {
                                CalendarBlockOverlap::Swallows => {
                                    overlapping_blocks.push(n);
                                }
                                CalendarBlockOverlap::GetsSwallowed => {
                                    destination = n;
                                    overlapping_blocks = vec![];
                                }
                            };
                        }
                        None => {
                            let idx = adjacency_list.add_node(block.id);
                            adjacency_list.add_edge(destination, idx, GraphEdgeType::Forward);
                            adjacency_list.add_edge(idx, destination, GraphEdgeType::Backward);

                            keep_going = false;
                        }
                    };
                }
                None => {
                    let idx = adjacency_list.add_node(block.id);
                    adjacency_list.add_edge(destination, idx, GraphEdgeType::Forward);
                    adjacency_list.add_edge(idx, destination, GraphEdgeType::Backward);

                    if overlapping_blocks.is_empty() {
                    } else {
                        //
                        overlapping_blocks.iter().for_each(|overlapping_block| {
                            adjacency_list.add_edge(
                                idx,
                                *overlapping_block,
                                GraphEdgeType::Forward,
                            );
                            adjacency_list.add_edge(
                                *overlapping_block,
                                idx,
                                GraphEdgeType::Backward,
                            );
                            match adjacency_list.find_edge(destination, *overlapping_block) {
                                Some(edge_idx) => {
                                    adjacency_list.remove_edge(edge_idx);
                                }
                                None => {}
                            }
                        });
                    }
                    keep_going = false;
                }
            };

            self.adjacency = adjacency_list.clone();
        }

        self.update_subtree_depth_until_root(destination, 1);
        self.id_to_block_map.insert(block.id, block);
        Ok(())
    }

    fn update_subtree_depth_until_root(&mut self, node_idx: NodeIndex, value: usize) {
        let node_id = self.adjacency[node_idx];
        let node = self.id_to_block_map.get(&node_id);

        if let Some(node) = node {
            if node.subtree_depth >= value {
                return;
            }

            self.id_to_block_map.insert(
                node_id,
                CalendarBlock {
                    id: node_id,
                    end_minute: node.end_minute,
                    start_minute: node.start_minute,
                    block_type: node.block_type,
                    subtree_depth: value,
                },
            );

            let mut parent = self.adjacency.neighbors(node_idx).filter(|_| {
                let edge = self
                    .adjacency
                    .edges(node_idx)
                    .find(|e| *e.weight() == GraphEdgeType::Backward);

                match edge {
                    Some(_) => true,
                    None => false,
                }
            });

            if let Some(p) = parent.next() {
                self.update_subtree_depth_until_root(p, value + 1);
            }
        }
    }

    pub fn display(&self) {
        println!("{}", Dot::new(&self.adjacency));
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

            let forward_neighbors = self.adjacency.neighbors(node_idx).filter(|n| {
                let edge = self
                    .adjacency
                    .edges_connecting(node_idx, *n)
                    .find(|e| *e.weight() == GraphEdgeType::Forward);

                match edge {
                    Some(_) => true,
                    None => false,
                }
            });

            forward_neighbors.into_iter().for_each(|n| {
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
                    subtree_depth: 1,
                };
            })
            .collect()
    }
}
