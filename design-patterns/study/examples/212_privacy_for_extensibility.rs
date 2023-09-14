fn main() {
    //  公共结构体和枚举类型的可扩展性

    // 如何在不破坏向后兼容性的情况下扩展一个结构体，为公共结构体添加公共字段或为枚举添加新的变体
    // 1. #[non_exhausitive]
    // 2. 添加私有字段
    

}

pub struct StudentInfo1 {
    pub name: String,
    pub age: u8,
    pub gender: Gender1,
    pub number: u32,
}

pub enum Gender1 {
    Male,
    Female,
}

