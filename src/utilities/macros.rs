#[allow(unused_macros)]
macro_rules! dth_version_clash {
    ($e:expr, $c:expr) => {
        format!(
            "Incompatible serial versions! Expected version {} but was {}.",
            $e, $c
        )
    };
}
pub(crate) use dth_version_clash;

// #[warn(unused_macros)]
