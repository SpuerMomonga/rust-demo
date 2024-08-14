fn main() {
  unsafe {
    let lib = libloading::Library::new("../../../target/release/shop.dll").unwrap();
    let func: libloading::Symbol<unsafe extern fn(a: i32, b: i32) -> i32> = lib.get(b"sum").unwrap();
    print!("{}", func(3,4))
  }
}