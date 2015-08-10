pub struct Context;

impl Context {
    pub unsafe fn current_context() -> Self {
        Context
    }
}
