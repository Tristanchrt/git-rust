# RustGit Project

# TODO

// TODO - WIP : https://chatgpt.com/c/67e24596-7ef0-8012-80b9-745c677b5838
- [ ] : Put HashRoot into commit as property
- [ ] : Get commit
- [ ] : Checkout commit / branch ?? restore file  

- [ ] Implement `git checkout` and `git branch`.
- [ ] Add git log
- [ ] Storage of Git Files and Objects 

### Main Features to Implement

#### 1. Storage of Git Files and Objects
- Implement a storage format for blobs, commits, and trees.
- Use SHA-1 (or SHA-256) to identify each object.
- Compress objects with `zlib`.
- Organize objects in a **DAG** (Directed Acyclic Graph) system.

#### 2. Commit and History Management
- Implement a commit system (references to parents).
- Create basic branch management (`HEAD`, refs, tags).
- Add a simple log (`git log`).

#### 3. Branch and Merge Management
- Implement `git checkout` and `git branch`.
- Merge branches with a diff algorithm.

#### 4. Indexing and Staging System
- Manage the index file (`git add` and `git status`).
- Track changes.

#### 5. Networking Protocols and Remote
- Support `git clone` and `git push` via HTTP or SSH.
- Compress and serialize objects for transfer.

## Requirements

- **Rust**: The language in which the project will be implemented.
- **zlib**: For compressing objects.
- **SHA-1 or SHA-256**: For hashing objects.
- **DAG**: For organizing Git objects.
