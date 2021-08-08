pub enum TreeType<T: GitObject> {
    Tree(T),
    Blob(T),
}
