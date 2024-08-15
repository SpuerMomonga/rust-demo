type Activate = dyn FnMut(i32, i32) -> Option<i32> + Send;

// #[repr(C)]
pub struct Plugin {
    pub name: &'static str,
    pub initialize: Box<Activate>,
}

// #[repr(C)]
pub struct Builder {
    pub name: &'static str,
    pub initialize: Box<Activate>,
}

impl Builder {
    pub fn new(name: &'static str) -> Self {
        Builder {
            name,
            initialize: Box::new(|_, _| None),
        }
    }

    #[must_use]
    pub fn initialize<F>(mut self, initialize: F) -> Self
    where
        F: FnMut(i32, i32) -> Option<i32> + Send + 'static,
    {
        self.initialize = Box::new(initialize);
        self
    }

    pub fn build(self) -> Plugin {
        Plugin {
            name: self.name,
            initialize: self.initialize,
        }
    }
}

#[no_mangle]
pub fn init() -> Plugin {
    Builder::new("shop.kit-shop")
        .initialize(|a, b| Some((a + b) / 2))
        .build()
}
