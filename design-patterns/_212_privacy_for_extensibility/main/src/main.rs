use student::*;
fn main() {
    //  公共结构体和枚举类型的可扩展性

    // 如何在不破坏向后兼容性的情况下扩展一个结构体，为公共结构体添加公共字段或为枚举添加新的变体
    let alice = StudentInfo1 {
        name: "Alice".to_string(),
        age: 20,
        gender: Gender1::Male,
        number: 1,
    };

    println!("Student info of alice: {:?}", alice);

    // 在升级的时候，字段可能会变化， 我们不希望别人直接这么用
    // 1. #[non_exhausitive]

    let bob = StudentInfo2::set_student_info("Bob".to_string(), 21, Gender2::Male, 2);
    println!("Student info of bob: {:?}", bob);

    let StudentInfo2 {
        name,
        age,
        gender,
        number,
        ..
    } = bob;
    println!(
        "Student info of bob: {:?} {:?} {:?} {:?}",
        name, age, gender, number
    );

    // 2. 添加私有字段
    // bob2 因为无法访问 _b 所以就没有办法直接创建了
    // let bob2 = StudentInfo3 {
    //     name: "Bob".to_string(),
    //     age: 21,
    //     gender: Gender3::Male,
    //     number: 2,
    //     _b: (),
    // };
    // println!("Student info of bob2: {:?}", bob2);

    let bob3 = StudentInfo3::set_student_info("Bob".to_string(), 21, Gender3::Male, 2);
    println!("Student info of bob3: {:?}", bob3);
}
