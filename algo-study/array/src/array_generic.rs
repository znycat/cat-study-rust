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

use std::fmt::Display;

#[allow(dead_code)]
struct ArrayList<E>
where
    E: Default + Display + Eq + Copy,
{
    /// 元素的数量
    size: usize,
    /// 所有的元素
    elements: Vec<E>,
}

impl<E> ArrayList<E>
where
    E: Default + Display + Eq + Copy,
{
    #[allow(dead_code)]
    fn new(capaticy: usize) -> Self {
        Self {
            size: 0,
            elements: vec![E::default(); capaticy],
        }
    }
}

const DEFAULT_CAPACITY: usize = 10;

impl<E> Default for ArrayList<E>
where
    E: Default + Display + Eq + Copy,
{
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }
}

impl<E> Display for ArrayList<E>
where
    E: Default + Display + Eq + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "size={}, [", self.size)?;
        for i in 0..self.size {
            let v = &self.elements[i];
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

impl<E> ArrayTrait<E> for ArrayList<E>
where
    E: Default + Display + Eq + Copy,
{
    fn clear(&mut self) {
        self.size = 0;
    }
    fn len(&self) -> usize {
        self.size
    }
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    fn contains(&self, element: E) -> bool {
        self.index_of(element) != None
    }
    fn append(&mut self, element: E) {
        self.ensure_capacity(self.size + 1);

        self.elements[self.size] = element;
        self.size += 1;
    }

    fn get(&self, index: usize) -> Option<E> {
        if index >= self.elements.len() {
            return None;
        }
        let e = self.elements[index];
        Option::Some(e)
    }
    fn set(&mut self, index: usize, element: E) -> Option<E> {
        if index >= self.elements.len() {
            return None;
        }
        let old = self.elements[index];
        self.elements[index] = element;
        Option::Some(old)
    }

    fn add(&mut self, index: usize, element: E) {
        if index > self.elements.len() {
            return;
        }

        self.ensure_capacity(self.size + 1);

        self.size += 1;

        let mut i: usize = self.size;
        while i > index {
            self.elements[i] = self.elements[i - 1];
            i -= 1;
        }

        self.elements[index] = element;
    }

    fn remove(&mut self, index: usize) -> Option<E> {
        if index >= self.elements.len() {
            return None;
        }
        let old = self.elements[index];
        self.size -= 1;

        for i in index..self.size {
            self.elements[i] = self.elements[i + 1];
        }

        return Option::Some(old);
    }

    fn index_of(&self, element: E) -> Option<usize> {
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

impl<E> ArrayList<E>
where
    E: Default + Display + Eq + Copy,
{
    /// 扩容
    /// @param capacity
    fn ensure_capacity(&mut self, capacity: usize) {
        let old_capacity = self.elements.capacity();
        if old_capacity > capacity {
            return;
        }
        // 右移1 相当于除2
        let new_capacity = old_capacity + (old_capacity >> 1);
        let mut new_elements = vec![E::default(); new_capacity];
        for i in 0..self.size {
            new_elements[i] = self.elements[i];
        }
        self.elements = new_elements;
    }
}

#[test]
fn test_array_generic() {
    let mut list: ArrayList<i32> = ArrayList::default();
    println!("list: {}", list);
    list.append(11);
    list.append(21);
    list.append(31);
    list.append(41);
    list.append(51);
    list.append(61);
    list.append(71);
    list.append(71);
    list.append(71);
    println!("list: {}", list);
    list.add(0, 1);
    println!("list: {}", list);
    list.add(2, 22);
    let a = list.remove(0);
    println!("a: {:?}", a);
    println!("list: {}", list);
}
