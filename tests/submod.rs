pub struct MyStruct {
    num: i32,
}

impl MyStruct {

    pub fn new() -> MyStruct {
        MyStruct { num: 2 }
    }

    pub fn add_num(&self) {
        let _ = self.num + 2;
    }
}
