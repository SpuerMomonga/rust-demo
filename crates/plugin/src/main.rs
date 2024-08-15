type Activate = dyn FnMut(i32, i32) -> Option<i32> + Send;

// #[repr(C)]
pub struct Plugin {
    pub name: &'static str,
    pub initialize: Box<Activate>,
}

pub trait PluginManager {}

fn main() {
    unsafe {
        let lib = libloading::Library::new("../../../target/release/shop.dll").unwrap();
        let init: libloading::Symbol<unsafe extern "C" fn() -> Plugin> = lib.get(b"init").unwrap();
        print!("{:?}", (init().initialize)(4, 4));
    }
}
