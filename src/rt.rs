#[cfg(windows)]
extern "system" {
    fn SetConsoleOutputCP(wCodePageID: u32) -> i32;
}

pub fn init() {
    #[cfg(windows)]
    {
        unsafe {
            SetConsoleOutputCP(65001);
        }
    }
}
