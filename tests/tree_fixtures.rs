use git_rust::domain::tree::TreeNodeTreeHash;

pub fn root_hash() -> TreeNodeTreeHash {
    TreeNodeTreeHash::new("2b".to_string(), "903ff50c055a7e06a7bb5de7210eb5d691b38d".to_string(), content_hash_root(), vec![subdir_hash(), file1_hash()])
}

pub fn file1_hash()  -> TreeNodeTreeHash {
    TreeNodeTreeHash::new("da".to_string(), "4e04a1877a7b6a5363ed4fc407a3885010c4de".to_string(), "I'm the file 1".to_string(), vec![])
}

pub fn subdir_hash() -> TreeNodeTreeHash  {
    TreeNodeTreeHash::new("96".to_string(), "764a513c0973ec5f6737a5bcdb5f95c30006f9".to_string(), content_hash_sub_dir(), vec![file2_hash(), file3_hash()])
}

pub fn file2_hash() -> TreeNodeTreeHash {
    TreeNodeTreeHash::new("e3".to_string(), "09fe7afbb26194fbade3c9266f2cef65b7a613".to_string(), "I'm the sub file 2".to_string(), vec![])
}

pub fn file3_hash() -> TreeNodeTreeHash {
    TreeNodeTreeHash::new("cb".to_string(), "b472bd5a547c092365cee7bec4022f09e4b8cc".to_string(),"I'm the sub file 3".to_string(), vec![])
}

pub fn content_hash_sub_dir() -> String {
    "100644 blob e309fe7afbb26194fbade3c9266f2cef65b7a613 subdir/file2.txt\n100644 blob cbb472bd5a547c092365cee7bec4022f09e4b8cc subdir/file3.txt".to_string()
}

pub fn content_hash_root() -> String {
    "100755 tree 96764a513c0973ec5f6737a5bcdb5f95c30006f9 subdir\n100755 blob da4e04a1877a7b6a5363ed4fc407a3885010c4de file1.txt".to_string()
}