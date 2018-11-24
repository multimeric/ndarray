//! Parallelization features for ndarray.
//!
//! The array views and references to owned arrays all implement
//! `IntoParallelIterator`; the default parallel iterators (each element
//! by reference or mutable reference) have no ordering guarantee in their
//! parallel implementations.
//!
//! `.axis_iter()` and `.axis_iter_mut()` also have parallel counterparts,
//! and their parallel iterators are indexed (and thus ordered) and exact length.
//!
//! `Zip` also implements `IntoParallelIterator`, and it has direct methods
//! called `.par_apply()` that one can use as direct parallelized replacements
//! for `.apply()`.
//!
//! # Examples
//!
//!
//! ## Arrays and array views
//!
//! Compute the exponential of each element in an array, parallelized.
//!
//! ```
//! extern crate ndarray;
//!
//! use ndarray::Array2;
//! use ndarray::parallel::prelude::*;
//!
//! fn main() {
//!     let mut a = Array2::<f64>::zeros((128, 128));
//!
//!     // Parallel versions of regular array methods
//!     a.par_map_inplace(|x| *x = x.exp());
//!     a.par_mapv_inplace(f64::exp);
//!
//!     // You can also use the parallel iterator directly
//!     a.par_iter_mut().for_each(|x| *x = x.exp());
//! }
//! ```
//!
//! ## Axis iterators
//!
//! Use the parallel `.axis_iter()` to compute the sum of each row.
//!
//! ```
//! extern crate ndarray;
//!
//! use ndarray::Array;
//! use ndarray::Axis;
//! use ndarray::parallel::prelude::*;
//!
//! fn main() {
//!     let a = Array::linspace(0., 63., 64).into_shape((4, 16)).unwrap();
//!     let mut sums = Vec::new();
//!     a.axis_iter(Axis(0))
//!      .into_par_iter()
//!      .map(|row| row.sum())
//!      .collect_into_vec(&mut sums);
//!
//!     assert_eq!(sums, [120., 376., 632., 888.]);
//! }
//! ```
//!
//! ## Zip
//!
//! Use zip for lock step function application across several arrays
//!
//! ```
//! extern crate ndarray;
//!
//! use ndarray::Array3;
//! use ndarray::Zip;
//!
//! type Array3f64 = Array3<f64>;
//!
//! fn main() {
//!     const N: usize = 128;
//!     let a = Array3f64::from_elem((N, N, N), 1.);
//!     let b = Array3f64::from_elem(a.dim(), 2.);
//!     let mut c = Array3f64::zeros(a.dim());
//!
//!     Zip::from(&mut c)
//!         .and(&a)
//!         .and(&b)
//!         .par_apply(|c, &a, &b| {
//!             *c += a - b;
//!         });
//! }
//! ```


/// Into- traits for creating parallelized iterators.
pub mod prelude {
    #[doc(no_inline)]
    pub use rayon::prelude::{ParallelIterator, IndexedParallelIterator,
    IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator};
}

pub use self::par::Parallel;

mod par;
mod ext_traits;
mod into_impls;
mod zipmacro;
