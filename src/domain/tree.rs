use sha1::{Digest, Sha1};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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

    pub fn to_tree_node(&self) -> TreeNodeTree {
        todo!()
    }

    pub fn flatten_nodes(&self) -> Vec<TreeNodeTreeHash> {
        let mut total = vec![self.to_flat()];
        total.extend(self.get_childs());
        total
    }

    fn get_childs(&self) -> Vec<TreeNodeTreeHash> {
        self.nodes.iter().flat_map(|node| {
            let mut childs = vec![node.clone().to_flat()];
            childs.extend(node.get_childs());
            childs
        }).collect()
    }

    pub fn to_flat(&self) -> TreeNodeTreeHash {
        TreeNodeTreeHash::new(self.prefix(), self.hash(), self.content(), vec![])
    }

    pub fn prefix(&self) -> String {
        self.prefix.clone()
    }

    pub fn hash(&self) -> String {
        self.hash.clone()
    }

    pub fn content(&self) -> String {
        self.clone().content
    }
}

impl TreeNodeTree {
    pub fn new(mode: String, filename: String, type_node: TreeNodeType, content: Option<String>, nodes: Vec<TreeNodeTree>) -> Self {
        Self { mode, filename, type_node, content, nodes }
    }

    pub fn hash_tree(current: TreeNodeTree) -> TreeNodeTreeHash {
        if !current.nodes.is_empty() {
            Self::tree_node(current)
        } else {
            Self::blob_node(current)
        }
    }

    fn tree_node(current: TreeNodeTree) -> TreeNodeTreeHash {
        let (hashes, nodes): (Vec<String>, Vec<TreeNodeTreeHash>) = current.nodes.iter()
            .map(|node| {
                let node_hashed = Self::hash_tree(node.clone());
                (Self::content_directory(node, &node_hashed), node_hashed)
            }).unzip();

        let (final_hash, prefix, hash) = Self::get_hash(hashes);
        TreeNodeTreeHash::new(prefix, hash, final_hash, nodes)
    }

    fn get_hash(hashes: Vec<String>) -> (String, String, String) {
        let final_hash = hashes.join("\n");
        let hash_blob = Self::hash(&final_hash);
        let (prefix, hash) = hash_blob.split_at(2);
        (final_hash, prefix.to_string(), hash.to_string())
    }

    fn blob_node(node: TreeNodeTree) -> TreeNodeTreeHash {
        let content = node.content.unwrap();
        let hash_blob = Self::hash(&content);
        let (prefix, hash) = hash_blob.split_at(2);
        TreeNodeTreeHash::new(prefix.to_string(), hash.to_string(), content, vec![])
    }

    fn hash(data: &String) -> String {
        let mut hasher = Sha1::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result).to_string()
    }

    fn content_directory(node: &TreeNodeTree, node_hashed: &TreeNodeTreeHash) -> String {
        format!("{} {} {} {}", node.mode, node.type_node.to_str(), node_hashed.complete_hash(), node.filename)
    }
}

