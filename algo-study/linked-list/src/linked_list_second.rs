#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Link::None }
    }

    pub fn push(&mut self, elem: T) {
        // Option 有一个take方法, 可以替代mem::replace
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Link::Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        // 让 map 作用在引用上，而不是直接作用在 self.head 上，为此我们可以使用 Option 的 as_ref 方法
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // 重写drop
        // 系统的drop 会递归的调用drop,
        // list -> A -> B -> C
        // 当list被drop时, 会调用A.drop, 然后递归B, 然后C,这个方法的递归可能会使栈溢出
        //
        // 为了防止这种情况，我们需要重写drop
        // 直接拿到当前的head 同时replace会将他替换为None
        let mut cur_link = self.head.take();
        // 遍历next, 将每一个元素都替换为None
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // println!("cur_link {:?}", boxed_node.elem);
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
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

    #[test]
    fn test_01() {
        let mut list = List::new();
        list.push(3);
        println!("list: {:?}", list);
    }

    #[test]
    fn test_peek() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value: &mut i32| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
}
