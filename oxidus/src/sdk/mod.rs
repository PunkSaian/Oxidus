pub mod class_id;
pub mod interface;
pub mod client_class;
#[allow(unused)]
pub mod module_names;
#[cfg(not(feature = "dump-netvars"))]
pub mod networkable;
#[cfg(not(feature = "dump-netvars"))]
pub mod unknown;
#[cfg(not(feature = "dump-netvars"))]
pub mod vmts;

#[allow(clippy::all,non_snake_case, non_camel_case_types)]
#[rustfmt::skip]
#[cfg(not(feature = "dump-netvars"))]
pub mod bindings;
