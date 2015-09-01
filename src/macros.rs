// Used to specify checks that shouldn't fail (but might in unsafe)
macro_rules! dbg_gl_error {
    ($($pat:pat => $msg:expr),*) => {
        if cfg!(debug_assertions) {
            let err = $crate::Context::get_error();
            match err {
                $(Some($pat) => {
                    panic!("OpenGL error {:?} - {}", err, $msg)
                }),*
                None => { }
            }
        }
    }
}

// Used to specify checks that should *never* be able to fail (even in unsafe!)
macro_rules! dbg_gl_sanity_check {
    ($($pat:pat => $msg:expr),*) => {
        dbg_gl_error! { $($pat => concat!("Sanity check failed: ", $msg)),* }
    }
}
