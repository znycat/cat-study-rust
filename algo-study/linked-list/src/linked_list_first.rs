///
///  链表
///  （英语：Linked list）
///  1. 是一种常见的数据结构
///  2. 是一种线性表,但并不会按线性的顺序存储数据
///  3. 每个节点都包含一个数据和下一个节点的指针
///
///  链表的特点
///  1. 插入复杂度:O(1),比数组要快
///  2. 查找和访问特定索引元素:O(n), 数组:O(1),O(logn)
///  3. 优缺点
///      - 数组: 可随机访问
///      - 链表: 不需要知道数据容量(capacity)
///      - 链表: 有指针和节点结构,空间开销会大
///
///  每一个元素指向下一个元素
///  A1 -> A2 -> A3 -> Null
///
///  List a = Empty | Elem a (List a)
///


#[cfg(test)]
mod test {
    // use crate::linked_list_first::List;
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn long_list() {
        let mut list = List::new();
        for i in 0..100000 {
            list.push(i);
        }
        // drop(list);
    }
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}

impl List {
    /// A -> B -> C -> Null
    /// push D
    /// D -> A -> B -> C -> Null
    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(Box::new(new_node));
    }

    /// D -> A -> B -> C -> Null
    /// pop() -> D
    /// A -> B -> C -> Null
    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
        // unimplemented!()
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut node) = cur_link {
            cur_link = std::mem::replace(&mut node.next, Link::Empty);
            // boxed_node 在这里超出作用域并被 drop,
            // 由于它的 `next` 字段拥有的 `Node` 被设置为 Link::Empty,
            // 因此这里并不会有无边界的递归发生
            
        }
    }
}

#[derive(Clone)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Clone)]
pub struct Node {
    elem: i32,
    next: Link,
}
