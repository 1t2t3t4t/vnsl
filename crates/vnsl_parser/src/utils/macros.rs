#[macro_export]
macro_rules! debug {
    ($debug_stmt: stmt) => {
        #[cfg(debug_assertions)]
        $debug_stmt
    };
}
