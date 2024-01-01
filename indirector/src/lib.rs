pub extern crate container as container;
pub use container::target;

/// Forces insertion of target's code during compilation of indirector. This is
/// not actually a solution, rather it is a sanity check. It's not a solution
/// because: we need to be able to call the proc_macros from within `user`.
pub fn indirector_forced_target() {
    target!()
}
