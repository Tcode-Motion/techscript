use std::collections::{HashMap, HashSet};
use techscript_ir::{BlockId, Function};

/// CFG connectivity and loop boundary analysis result.
#[derive(Debug, Clone)]
pub struct CFGAnalysis {
    pub predecessors: HashMap<BlockId, Vec<BlockId>>,
    pub successors: HashMap<BlockId, Vec<BlockId>>,
    pub back_edges: HashSet<(BlockId, BlockId)>,
}

impl CFGAnalysis {
    /// Performs control-flow graph validation and loop back-edge discovery.
    pub fn analyze(func: &Function) -> Self {
        let mut predecessors = HashMap::new();
        let mut successors = HashMap::new();

        for block in &func.blocks {
            successors.insert(block.id, block.successors.clone());
            predecessors.insert(block.id, block.predecessors.clone());
        }

        // Back-edge discovery via simple DFS traversal
        let mut back_edges = HashSet::new();
        let mut visited = HashSet::new();
        let mut stack = HashSet::new();

        if !func.blocks.is_empty() {
            let entry_id = func.blocks[0].id;
            Self::dfs(
                entry_id,
                &successors,
                &mut visited,
                &mut stack,
                &mut back_edges,
            );
        }

        Self {
            predecessors,
            successors,
            back_edges,
        }
    }

    fn dfs(
        curr: BlockId,
        succs: &HashMap<BlockId, Vec<BlockId>>,
        visited: &mut HashSet<BlockId>,
        stack: &mut HashSet<BlockId>,
        back_edges: &mut HashSet<(BlockId, BlockId)>,
    ) {
        visited.insert(curr);
        stack.insert(curr);

        if let Some(targets) = succs.get(&curr) {
            for &target in targets {
                if stack.contains(&target) {
                    // Loop back-edge detected
                    back_edges.insert((curr, target));
                } else if !visited.contains(&target) {
                    Self::dfs(target, succs, visited, stack, back_edges);
                }
            }
        }

        stack.remove(&curr);
    }
}
