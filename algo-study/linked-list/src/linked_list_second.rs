pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::None),
        });
        self.head = Link::Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // 重写drop
        // 系统的drop 会递归的调用drop,
        // list -> A -> B -> C
        // 当list被drop时, 会调用A.drop, 然后递归B, 然后C,这个方法的递归可能会使栈溢出
        //
        // 为了防止这种情况，我们需要重写drop
        // 直接拿到当前的head 同时replace会将他替换为None
        let mut cur_link = std::mem::replace(&mut self.head, None);
        // 遍历next, 将每一个元素都替换为None
        while let Some(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, None);
            // println!("cur_link {:?}", boxed_node.elem);
        }
    }
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

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
        println!("i32::MAX = {:?}", i32::MAX);
        for i in 0..1000000000 {
            list.push(i);
        }
        // drop(list);
    }
}
