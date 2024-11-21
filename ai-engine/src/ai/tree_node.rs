use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct TreeNode {
    pub depth: usize,
    pub state: String,
    pub evaluation: f32,
    pub best_move: String,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    // pub fn new(depth: usize, best_move: Move, state: String, evaluation: f32) -> Self {
    //     TreeNode {
    //         depth,
    //         state,
    //         evaluation,
    //         best_move,
    //         children: Vec::new(),
    //     }
    // }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }
}
