use std::sync::Arc;
use backtrace::Backtrace;

struct PanicWrapper {
    data: Vec<u8>
}

impl AsRef<[u8]> for PanicWrapper {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

static mut BT: Option<Backtrace> = None;

impl Drop for PanicWrapper {
    fn drop(&mut self) {
        unsafe { BT = Some(Backtrace::new()); }
    }
}

#[test]
fn macos() {
    let p = Arc::new(PanicWrapper { data: vec![0, 3, 4] });
    let d = core_graphics::data_provider::CGDataProvider::from_buffer(p);
    drop(d);

    let bt = unsafe { BT.take().unwrap() };
    println!("{:#?}", bt);
    let sym = bt.frames().iter().flat_map(|frame| frame.symbols()).find(|sym| {
        match sym.name().and_then(|name| name.as_str()) {
            Some(name) => name.contains("_data_release_info"),
            None => false,
       }
    });
    println!("{:?}", sym);
    assert!(sym.is_some());
    //panic!();
}
