use std::ffi::CString;

fn main() {
    let name = CString::new("ryuta hayashi").unwrap();
    let name_ptr = name.as_ptr();
    let ret = unsafe {cmake_example::test1(name_ptr)};
    println!("ret: {}", ret);

    println!("add => {}", unsafe{cmake_example::adder(35, 45)});
}
