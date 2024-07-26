pub mod model;

#[macro_export]
macro_rules! impl_deref {
    ($type:ty, $target:ty, $member:tt) => {
        impl std::ops::Deref for $type {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.$member
            }
        }
    };
}
