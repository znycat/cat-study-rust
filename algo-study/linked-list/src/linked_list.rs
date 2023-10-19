trait ListTrait<E> {
    /// 清空数组
    fn clear(&mut self);

    /// 获取数组长度
    /// @return
    fn len(&self) -> usize;

    /// 判断数组是否为空
    /// @return
    fn is_empty(&self) -> bool;

    /// 判断是否包含element
    /// @param element
    /// @return
    fn contains(&self, element: E) -> bool;

    /// 添加元素到尾部
    /// @param element
    fn append(&mut self, element: E);

    /// 获取index位置的元素
    /// @param index
    /// @return
    fn get(&self, index: usize) -> Option<E>;

    /// 设置index位置的元素
    /// @param index
    /// @param element
    fn set(&mut self, index: usize, element: E) -> Option<E>;

    /// 在index位置插入element
    /// @param index
    /// @param element
    fn add(&mut self, index: usize, element: E);

    /// 删除index位置的元素
    /// @param index
    /// @return
    fn remove(&mut self, index: usize) -> Option<E>;

    /// 获取index位置的元素
    /// @param index
    /// @return
    fn index_of(&self, element: E) -> Option<usize>;
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

#[allow(dead_code)]
struct LinkedListI32 {
    /// 元素的数量
    size: usize,
    first_node: Option<Box<ListNode>>,
}

const DEFAULT_CAPACITY: usize = 10;

impl Default for LinkedListI32 {
    fn default() -> Self {
        Self::new()
    }
}

impl LinkedListI32 {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            size: 0,
            first_node: None,
        }
    }
}

// impl std::fmt::Display for LinkedListI32 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "size={}, [", self.size)?;
//         for i in 0..self.size {
//             let v = self.elements[i];
//             if i != 0 {
//                 write!(f, ", ")?;
//             }
//             write!(f, "{}", v)?;
//         }
//         write!(f, "]")
//     }
// }

// impl ListTrait<i32> for LinkedListI32 {
//     fn clear(&mut self) {
//         self.size = 0;
//         self.first_node = None;
//     }
//
//     fn len(&self) -> usize {
//         self.size
//     }
//
//     fn is_empty(&self) -> bool {
//         self.size == 0
//     }
//
//     fn contains(&self, element: i32) -> bool {
//         self.index_of(element) != None
//     }
//
//     fn append(&mut self, element: i32) {
//         let node = Box::new(ListNode::new(element));
//
//         let mut current_node = self.first_node.as_mut();
//         if current_node == None {
//             self.first_node = Some(node);
//         }else {
//             while current_node.next != None {
//                 current_node = current_node.unwrap().next.as_mut();
//                 
//             }
//
//         }
//         
//         if self.first_node == None {
//         }else {
//             while current_node.next != None {
//                 current_node = current_node.next.as_mut().unwrap();
//             }
//             current_node.next = Some(node);
//         }
//         self.size += 1;
//     }
//     fn get(&self, index: usize) -> Option<i32> {
//
//         if index >= self.elements.len() {
//             return None;
//         }
//         Option::Some(self.elements[index])
//     }
//     fn set(&mut self, index: usize, element: i32) -> Option<i32> {
//         if index >= self.elements.len() {
//             return None;
//         }
//         let old = self.elements[index];
//         self.elements[index] = element;
//         Option::Some(old)
//     }
//
//     fn add(&mut self, index: usize, element: i32) {
//         if index > self.elements.len() {
//             return;
//         }
//
//         self.ensure_capacity(self.size + 1);
//
//         self.size += 1;
//
//         let mut i: usize = self.size;
//         while i > index {
//             self.elements[i] = self.elements[i - 1];
//             i -= 1;
//         }
//
//         self.elements[index] = element;
//     }
//
//     fn remove(&mut self, index: usize) -> Option<i32> {
//         if index >= self.elements.len() {
//             return None;
//         }
//         let old = self.elements[index];
//         self.size -= 1;
//
//         for i in index..self.size {
//             self.elements[i] = self.elements[i + 1];
//         }
//
//         return Option::Some(old);
//     }
//
//     fn index_of(&self, element: i32) -> Option<usize> {
//         let mut res: usize = 0;
//         for i in 0..self.size {
//             if self.elements[i] == element {
//                 return Some(res);
//             }
//             res += 1;
//         }
//         None
//     }
// }
//
//
// #[test]
// fn test_array() {
//     let mut list = LinkedListI32::default();
//     println!("list: {}", list);
//     list.append(11);
//     list.append(21);
//     list.append(31);
//     list.append(41);
//     list.append(51);
//     list.append(61);
//     list.append(71);
//     list.append(71);
//     list.append(71);
//     println!("list: {}", list);
//     list.add(0, 1);
//     println!("list: {}", list);
//     list.add(2, 22);
//     // let a = list.remove(0);
//     // println!("a: {:?}", a);
//     println!("list: {}", list);
// }
