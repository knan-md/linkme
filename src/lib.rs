//! **A library for safe cross-platform linker shenanigans.**
//!
//! <br>
//!
//! # Platform support
//!
//! | Component | Linux | macOS | Windows | Other...<sup>†</sup> |
//! |:---|:---:|:---:|:---:|:---:|
//! | Distributed slice | ✅ | ✅ | ✅ | |
//!
//! <br>***<sup>†</sup>*** We welcome PRs adding support for any platforms not
//! listed here.
//!
//! <br>
//!
//! # Distributed slice
//!
//! A distributed slice is a collection of static elements that are gathered
//! into a contiguous section of the binary by the linker. Slice elements may be
//! defined individually from anywhere in the dependency graph of the final
//! binary.
//!
//! Refer to [`linkme::DistributedSlice`][DistributedSlice] for complete details
//! of the API. The basic idea is as follows.
//!
//! A static distributed slice is declared by writing `#[distributed_slice]` on
//! a static item whose type is `[T]` for some type `T`. The initializer
//! expression must be `[..]` to indicate that elements come from elsewhere.
//!
//! ```
//! # struct Bencher;
//! #
//! use linkme::distributed_slice;
//!
//! #[distributed_slice]
//! pub static BENCHMARKS: [fn(&mut Bencher)] = [..];
//! ```
//!
//! Slice elements may be registered into a distributed slice by a
//! `#[distributed_slice(...)]` attribute in which the path to the distributed
//! slice is given in the parentheses. The initializer is required to be a const
//! expression.
//!
//! ```
//! # mod other_crate {
//! #     use linkme::distributed_slice;
//! #
//! #     pub struct Bencher;
//! #
//! #     #[distributed_slice]
//! #     pub static BENCHMARKS: [fn(&mut Bencher)] = [..];
//! # }
//! #
//! # use other_crate::Bencher;
//! #
//! use linkme::distributed_slice;
//! use other_crate::BENCHMARKS;
//!
//! #[distributed_slice(BENCHMARKS)]
//! static BENCH_DESERIALIZE: fn(&mut Bencher) = bench_deserialize;
//!
//! fn bench_deserialize(b: &mut Bencher) {
//!     /* ... */
//! }
//! ```
//!
//! The distributed slice behaves in all ways like `&'static [T]`.
//!
//! ```no_run
//! # use linkme::distributed_slice;
//! #
//! # struct Bencher;
//! #
//! # #[distributed_slice]
//! # static BENCHMARKS: [fn(&mut Bencher)] = [..];
//! #
//! fn main() {
//!     // Iterate the elements.
//!     for bench in BENCHMARKS {
//!         /* ... */
//!     }
//!
//!     // Index into the elements.
//!     let first = BENCHMARKS[0];
//!
//!     // Slice the elements.
//!     let except_first = &BENCHMARKS[1..];
//!
//!     // Invoke methods on the underlying slice.
//!     let len = BENCHMARKS.len();
//! }
//! ```

#![no_std]
#![doc(html_root_url = "https://docs.rs/linkme/0.2.1")]

mod distributed_slice;

#[doc(hidden)]
pub mod private;

pub use linkme_impl::*;

pub use crate::distributed_slice::DistributedSlice;
