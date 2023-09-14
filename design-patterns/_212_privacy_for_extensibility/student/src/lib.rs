#[derive(Debug)]
pub struct StudentInfo1 {
    pub name: String,
    pub age: u8,
    pub gender: Gender1,
    pub number: u32,
}

#[derive(Debug)]
pub enum Gender1 {
    Male,
    Female,
}

#[non_exhaustive] // 不是最终版本
#[derive(Debug)]
pub struct StudentInfo2 {
    pub name: String,
    pub age: u8,
    pub gender: Gender2,
    pub number: u32,
}

impl StudentInfo2 {
    pub fn set_student_info(name: String, age: u8, gender: Gender2, number: u32) -> Self {
        StudentInfo2 {
            name,
            age,
            gender,
            number,
        }
    }
}

#[derive(Debug)]
pub enum Gender2 {
    Male,
    Female,
}

#[derive(Debug)]
pub struct StudentInfo3 {
    pub name: String,
    pub age: u8,
    pub gender: Gender3,
    pub number: u32,
    _b: (),
}

impl StudentInfo3 {
    pub fn set_student_info(name: String, age: u8, gender: Gender3, number: u32) -> Self {
        StudentInfo3 {
            name,
            age,
            gender,
            number,
            _b: (), 
        }
    }
}

#[derive(Debug)]
pub enum Gender3 {
    Male,
    Female,
}
