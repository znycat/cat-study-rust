trait ArrayTrait<E> {
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

#[allow(dead_code)]
struct ArrayListI32 {
    /// 元素的数量
    size: usize,
    /// 所有的元素
    elements: Vec<i32>,
}

const DEFAULT_CAPACITY: usize = 10;

impl Default for ArrayListI32 {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
        // Self {
        //     size: 0,
        //     elements: vec![0, DEFAULT_CAPACITY],
        // }
    }
}

impl ArrayListI32 {
    #[allow(dead_code)]

    #[allow(dead_code)]
    fn new(capaticy: usize) -> Self {
        Self {
            size: 0,
            elements: vec![0; capaticy],
        }
    }
}

impl std::fmt::Display for ArrayListI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "size={}, [", self.size)?;
        for i in 0..self.size {
            let v = self.elements[i];
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

impl ArrayTrait<i32> for ArrayListI32 {
    fn clear(&mut self) {
        self.size = 0;
    }
    fn len(&self) -> usize {
        self.size
    }
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    fn contains(&self, element: i32) -> bool {
        self.index_of(element) != None
    }
    fn append(&mut self, element: i32) {
        self.elements[self.size] = element;
        self.size += 1;
    }
    fn get(&self, index: usize) -> Option<i32> {
        if index >= self.elements.len() {
            return None;
        }
        Option::Some(self.elements[index])
    }
    fn set(&mut self, index: usize, element: i32) -> Option<i32> {
        if index >= self.elements.len() {
            return None;
        }
        let old = self.elements[index];
        self.elements[index] = element;
        Option::Some(old)
    }
    fn add(&mut self, index: usize, element: i32) {
        self.elements[index] = element;
        self.size += 1;
    }
    fn remove(&mut self, _index: usize) -> Option<i32> {
        None
    }
    fn index_of(&self, element: i32) -> Option<usize> {
        let mut res: usize = 0;
        for i in 0..self.size {
            if self.elements[i] == element {
                return Some(res);
            }
            res += 1;
        }
        None
    }
}

#[test]
fn test_array() {
    let mut list = ArrayListI32::default();
    println!("list: {}", list);
    list.append(11);
    list.append(11);
    list.append(11);
    list.append(11);
    list.append(11);
    println!("list: {}", list);
}
