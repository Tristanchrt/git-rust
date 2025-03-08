# RustGit Project

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
