use sha1::{Digest, Sha1};

#[derive(Debug, Clone)]
pub enum TreeNodeType {
    BLOB,
    TREE,
}

impl TreeNodeType {
    pub fn to_str(&self) -> String {
        match self {
            TreeNodeType::BLOB => "blob".to_string(),
            TreeNodeType::TREE => "tree".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct TreeNodeTree {
    mode: String,
    filename: String,
    type_node: TreeNodeType,
    content: Option<String>,
    nodes: Vec<TreeNodeTree>
}

#[derive(Debug, PartialEq, Clone)]
pub struct TreeNodeTreeHash {
    prefix: String,
    hash: String,
    content: String,
    nodes: Vec<TreeNodeTreeHash>
}

impl TreeNodeTreeHash {
    pub fn new(prefix: String, hash: String, content: String, nodes: Vec<TreeNodeTreeHash>) -> Self {
        Self { prefix, hash, content, nodes }
    }

    pub fn complete_hash(&self) -> String {
        format!("{}{}", self.prefix, self.hash)
    }

    pub fn to_hashes(&self) -> Vec<TreeNodeTreeHash> {
        todo!()
    }
}

impl TreeNodeTree {
    pub fn new(mode: String, filename: String, type_node: TreeNodeType, content: Option<String>, nodes: Vec<TreeNodeTree>) -> Self {
        Self { mode, filename, type_node, content, nodes }
    }

    // TODO - WIP
    pub fn hash_tree(current: TreeNodeTree) -> TreeNodeTreeHash {
        if current.nodes.is_empty() {
            Self::blob_node(current)
        }else {
            let mut nodes_hash: Vec<TreeNodeTreeHash> = vec![];
            let mut current_data: Vec<String> = vec![];
            for node in current.nodes {
                let n = Self::hash_tree(node.clone());
                let content = format!("{} {} {} {}", node.mode, node.type_node.to_str(), n.complete_hash(), node.filename);
                current_data.push(content);
                nodes_hash.push(n);
            }

            let final_hash = current_data.join("\n");
            let hash_blob = Self::hash(&final_hash);
            TreeNodeTreeHash::new((&hash_blob[..2]).parse().unwrap(), (&hash_blob[2..]).parse().unwrap(), final_hash, nodes_hash)
        }
    }

    fn blob_node(node: TreeNodeTree) -> TreeNodeTreeHash {
        let content = node.content.unwrap();
        let hash_blob = Self::hash(&content);
        TreeNodeTreeHash::new((&hash_blob[..2]).parse().unwrap(), (&hash_blob[2..]).parse().unwrap(), content, vec![])
    }

    fn hash(data: &String) -> String {
        let mut hasher = Sha1::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result).to_string()
    }
}

