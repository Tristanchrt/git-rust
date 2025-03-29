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

    // TODO - WIP : https://chatgpt.com/c/67e24596-7ef0-8012-80b9-745c677b5838
    pub fn hash_tree(current: TreeNodeTree) -> TreeNodeTreeHash {
        if current.nodes.is_empty() {
            Self::blob_node(current)
        }else {
            let (final_hash, nodes) = current.nodes.iter()
                .fold((String::new(), vec![]), |(mut final_hash, mut nodes), node| {
                    let node_hashed = Self::hash_tree(node.clone());
                    nodes.push(node_hashed.clone());

                    if(!final_hash.is_empty()) {
                        final_hash.push_str("\n")
                    }
                    final_hash.push_str(&format!("{} {} {} {}", node.mode, node.type_node.to_str(), node_hashed.complete_hash(), node.filename));

                    (final_hash, nodes)
                });

            let hash_blob = Self::hash(&final_hash);
            TreeNodeTreeHash::new((&hash_blob[..2]).parse().unwrap(), (&hash_blob[2..]).parse().unwrap(), final_hash, nodes)
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

