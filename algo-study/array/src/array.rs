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
    fn set(&mut self, index: usize, element: E);

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
        Self::new_capaticy(DEFAULT_CAPACITY)
        // Self {
        //     size: 0,
        //     elements: vec![0, DEFAULT_CAPACITY],
        // }
    }
}

impl ArrayListI32 {
    #[allow(dead_code)]
    fn new(size: usize, elements: Vec<i32>) -> Self {
        Self { size, elements }
    }

    #[allow(dead_code)]
    fn new_capaticy(capaticy: usize) -> Self {
        Self {
            size: capaticy,
            elements: vec![0, capaticy as i32],
        }
    }
}

impl ArrayTrait<i32> for ArrayListI32 {
    fn clear(&mut self) {}
    fn len(&self) -> usize {
        0
    }
    fn is_empty(&self) -> bool {
        true
    }
    fn contains(&self, _element: i32) -> bool {
        false
    }
    fn append(&mut self, _element: i32) {}
    fn get(&self, _index: usize) -> Option<i32> {
        None
    }
    fn set(&mut self, _index: usize, _element: i32) {}
    fn add(&mut self, _index: usize, _element: i32) {}
    fn remove(&mut self, _index: usize) -> Option<i32> {
        None
    }
    fn index_of(&self, _element: i32) -> Option<usize> {
        None
    }
}

#[test]
fn test_array() {
    println!("Hello, world!");
}
