pub mod platform;

/// Representation of system resources required to register, modify and remove file extension associations
pub(crate) trait Context {
    unsafe fn init() -> Self;
}