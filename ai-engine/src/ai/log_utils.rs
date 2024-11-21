use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use super::tree_node::TreeNode;

fn parse_logs(file_path: &str) -> io::Result<TreeNode> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Root of the tree
    let mut root = TreeNode::new("Root".to_string(), 0.0);
    let mut nodes: HashMap<String, &mut TreeNode> = HashMap::new();
    nodes.insert(root.state.clone(), &mut root);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(", ").collect();

        // Extract data from log
        let depth_str = parts[0].strip_prefix("Depth: ").unwrap();
        let move_str = parts[1].strip_prefix("Move: ").unwrap();
        let eval_str = parts[2].strip_prefix("Value: ").unwrap();
        let parent_str = parts[3].strip_prefix("Parent: ").unwrap();

        let state = move_str.to_string();
        let evaluation: f32 = eval_str.parse().unwrap();

        // Create the node
        let mut new_node = TreeNode::new(state.clone(), evaluation);

        // Find parent and add child
        if let Some(parent_node) = nodes.get_mut(parent_str) {
            parent_node.add_child(new_node);
        }

        // Insert new node into the map
        nodes.insert(state, nodes.get_mut(&state).unwrap());
    }

    Ok(root)
}

fn export_tree_to_json(tree: &TreeNode) -> String {
    serde_json::to_string_pretty(tree).unwrap()
}

// Save JSON to file:
fn save_tree_as_json(tree: &TreeNode, file_path: &str) -> io::Result<()> {
    let json = export_tree_to_json(tree);
    std::fs::write(file_path, json)
}
