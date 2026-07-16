use std::collections::{HashMap, HashSet};
use techscript_ir::{BlockId, Function};

/// Dominator analysis result containing dominators list and dominance frontier.
#[derive(Debug, Clone)]
pub struct DominatorAnalysis {
    pub dominators: HashMap<BlockId, HashSet<BlockId>>,
    pub idoms: HashMap<BlockId, BlockId>,
}

impl DominatorAnalysis {
    /// Computes the dominator tree map for a function.
    pub fn analyze(func: &Function) -> Self {
        let mut dominators = HashMap::new();
        let mut idoms = HashMap::new();

        if func.blocks.is_empty() {
            return Self { dominators, idoms };
        }

        let all_blocks: HashSet<BlockId> = func.blocks.iter().map(|b| b.id).collect();
        let entry_id = func.blocks[0].id;

        // Initialize dominator sets
        for block in &func.blocks {
            if block.id == entry_id {
                let mut set = HashSet::new();
                set.insert(entry_id);
                dominators.insert(entry_id, set);
            } else {
                dominators.insert(block.id, all_blocks.clone());
            }
        }

        // Iterative algorithm to compute dominators
        let mut changed = true;
        while changed {
            changed = false;
            for block in &func.blocks {
                if block.id == entry_id {
                    continue;
                }

                let mut new_dom = all_blocks.clone();
                for &pred in &block.predecessors {
                    if let Some(pred_doms) = dominators.get(&pred) {
                        new_dom = new_dom.intersection(pred_doms).cloned().collect();
                    }
                }
                new_dom.insert(block.id);

                if let Some(old_doms) = dominators.get_mut(&block.id) {
                    if *old_doms != new_dom {
                        *old_doms = new_dom;
                        changed = true;
                    }
                }
            }
        }

        // Compute immediate dominators (idoms)
        for block in &func.blocks {
            if block.id == entry_id {
                continue;
            }
            if let Some(doms) = dominators.get(&block.id) {
                // The idom is the dominator that dominates block, other than block itself,
                // and does not dominate any other dominator of block.
                for &dom in doms {
                    if dom == block.id {
                        continue;
                    }
                    // Check if dom is dominated by all other dominators (longest path)
                    let mut is_idom = true;
                    for &other in doms {
                        if other == block.id || other == dom {
                            continue;
                        }
                        if let Some(other_doms) = dominators.get(&other) {
                            if other_doms.contains(&dom) {
                                is_idom = false;
                                break;
                            }
                        }
                    }
                    if is_idom {
                        idoms.insert(block.id, dom);
                        break;
                    }
                }
            }
        }

        Self { dominators, idoms }
    }
}
