use std::mem::{transmute, ManuallyDrop};

struct MyStruct<'a> {
    text: &'a str,
}

fn create_struct<'a>() -> MyStruct<'a> {
    let s = String::from("hello"); // s 在函数结束后被销毁

    unsafe {
        MyStruct {
            text: transmute(s.as_bytes()),
        } // ❌ text 引用了 s，将导致悬垂引用
    }
}

fn main() {
    let s = String::from("hello"); // s 在函数结束后被销毁
    unsafe {
        let s= ManuallyDrop::new(s);
        let a: &str = transmute(s.as_bytes());
        println!("{:#?}", a);
    }
}
