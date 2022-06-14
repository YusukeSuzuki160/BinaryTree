use std::cmp::Ordering;

struct BinaryTree<T: Ord> {
    first_node: Node<T>,
}

struct Node<T: Ord> {
    item: T,
    left_node: Box<Node<T>>,
    right_node: Box<Node<T>>,
}

impl<T> BinaryTree<T> 
where
    T: Ord
{
    fn new(item: T) -> Self {
        BinaryTree<T> {
            first_node: Node::new(item),
        }
    }
    fn insert(item: &T) {
        if first_node.is_empty() {
            first_node.insert(item);
        } else {
            match item.cmp(first_node.item) {
                Ordering::Equal | Ordering::Less => first_node.insert_left(item),
                Ordering::Greater => first_node.insert_right(item).
                _ => (),
            }
        }
    }
    fn DFS_preorder() {
        first_node.preorder();
    }
    fn DFS_inorder() {
        first_node.inorder();
    }
    fn DFS_postorder() {
        first_node.postorder();
    }
    fn BFS {
        first_node.bfs();
    }
}

impl<T> Node<T>
where 
    T: Ord
{

}
