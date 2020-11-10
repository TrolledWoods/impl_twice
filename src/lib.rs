#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![no_std]

//! A crate for avoiding code duplication for immutable and mutable types.
//!
//! # Reason for existance
//! When writing rust programs, times come when you need two types,
//! one immutable and one mutable.
//!
//! It is possible to remove this duplication with DSTs, such as the
//! standard library's slice type, where `&[T]` and `&mut [T]` are the
//! immutable/mutable counterparts. However, DSTs cannot be created
//! by the programmer, and therefore they are not always applicable.
//!
//! When making two very similar types that are just immutable/mutable
//! counterparts to each other, you may have to implement the same
//! things on both of the types. Here is an example of the duplication;
//!
//! ```
//! struct WrappedSlice<'a, T>(&'a [T]);
//! struct WrappedSliceMut<'a, T>(&'a mut [T]);
//!
//! impl<T> WrappedSlice<'_, T> {
//!     pub fn inner(&self) -> &'_ [T] {
//!         self.0
//!     }
//!
//!     pub fn get(&self, index: usize) -> Option<&'_ T> {
//!         self.0.get(index)
//!     }
//! }
//!
//! impl<T> WrappedSliceMut<'_, T> {
//!     pub fn inner(&self) -> &'_ [T] {
//!         self.0
//!     }
//!
//!     pub fn get(&self, index: usize) -> Option<&'_ T> {
//!         self.0.get(index)
//!     }
//!
//!     pub fn get_mut(&mut self, index: usize) -> Option<&'_ mut T> {
//!         self.0.get_mut(index)
//!     }
//! }
//! ```
//!
//! This can be solved by having a way to implement the same items on both
//! types. That's what this crate is designed for!
//! This is equivalent to the above example but implemented with this
//! crate;
//!
//! ```
//! # use impl_twice::impl_twice;
//! struct WrappedSlice<'a, T>(&'a [T]);
//! struct WrappedSliceMut<'a, T>(&'a mut [T]);
//!
//! impl_twice! (
//!     impl<T> WrappedSlice<'_, T>, WrappedSliceMut<'_, T> {
//!         pub fn inner(&self) -> &'_ [T] {
//!             self.0
//!         }
//!
//!         pub fn get(&self, index: usize) -> Option<&'_ T> {
//!             self.0.get(index)
//!         }
//!     }
//! );
//!
//! impl<T> WrappedSliceMut<'_, T> {
//!     pub fn get_mut(&mut self, index: usize) -> Option<&'_ mut T> {
//!         self.0.get_mut(index)
//!     }
//! }
//! ```
//!
//! As you can see, the two methods ``inner`` and ``get`` that were duplicated
//! are now only implemented once.
//!
//! # Usage
//! There are quite a few different ways to use the macro based on what you want.
//! ```
//! # use impl_twice::impl_twice;
//!
//! # #[allow(unused)]
//! struct Owned<T>(T);
//! # #[allow(unused)]
//! struct Borrowed<'a, T>(&'a T);
//! # #[allow(unused)]
//! struct BorrowedMut<'a, T>(&'a mut T);
//!
//! trait SomeTrait<T> {
//!     fn get(&self) -> &'_ T;
//! }
//!
//! impl_twice!(
//!     // Normal impl blocks work just fine
//!     impl<T> Owned<T> {
//!         fn get(&self) -> &'_ T {
//!             &self.0
//!         }
//!     }
//! );
//! ```
//!
//! If you want to implement the same items on several types,
//! just separate the types with commas;
//!
//! ```
//! # use impl_twice::impl_twice;
//! # #[allow(unused)]
//! # struct Owned<T>(T);
//! # #[allow(unused)]
//! # struct Borrowed<'a, T>(&'a T);
//! # #[allow(unused)]
//! # struct BorrowedMut<'a, T>(&'a mut T);
//! impl_twice!(
//!     impl<T>
//!         Borrowed<'_, T>,
//!         BorrowedMut<'_, T>
//!     {
//!         fn get(&self) -> &'_ T {
//!             self.0
//!         }
//!     }
//! );
//! ```
//! Adding type bounds is done with 'where'.
//! Here this library differs from normal impls in that you have
//! to add parenthesees after the where.
//! ```
//! # use impl_twice::impl_twice;
//! # #[allow(unused)]
//! # struct Owned<T>(T);
//! # #[allow(unused)]
//! # struct Borrowed<'a, T>(&'a T);
//! # #[allow(unused)]
//! # struct BorrowedMut<'a, T>(&'a mut T);
//! impl_twice!(
//!     impl<T>
//!         Borrowed<'_, T>,
//!         BorrowedMut<'_, T>
//!     where (T: Clone) {
//!         fn to_owned(&self) -> Owned<T> {
//!             Owned(self.0.clone())
//!         }
//!     }
//! );
//! ```
//! Traits work as well.
//! ```
//! # use impl_twice::impl_twice;
//! # #[allow(unused)]
//! # struct Owned<T>(T);
//! # #[allow(unused)]
//! # struct Borrowed<'a, T>(&'a T);
//! # #[allow(unused)]
//! # struct BorrowedMut<'a, T>(&'a mut T);
//! use std::fmt::{Debug, Formatter, Result};
//! impl_twice!(
//!     impl<T>
//!         Debug for Borrowed<'_, T>,
//!         Debug for BorrowedMut<'_, T>,
//!         Debug for Owned<T>
//!     where (T: Debug) {
//!         fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//!             write!(f, "[{:?}]", self.0)
//!         }
//!     }
//! );
//! ```
//!
//! Sometimes you may want different generics
//! for each type, maybe because they have a different number
//! of generic parameters.
//!
//! In that case, you can simply add another `impl`, and the
//! types specified after that impl will use its generics.
//!
//! You have a separate `where` block for each `impl`.
//!
//! ```
//! # use impl_twice::impl_twice;
//! #[derive(Clone)]
//! struct Complex<A, B>(A, B);
//! #[derive(Clone)]
//! struct Simple<T>(T);
//!
//! impl_twice!(
//!     impl<A, B> Complex<A, B> where (A: Clone, B: Clone)
//!     impl<T> Simple<T> where (T: Clone) {
//!         fn redundant_clone_method_for_example_purposes(&self) -> Self {
//!             self.clone()
//!         }
//!     }
//! );
//! ```
//!
//! # Limitations
//! * Trait, type names and generic parameters are simply tokens. That means, you cannot specify a
//! path with ``::``, so you have to ``use`` the items first before implementing them. This also
//! means that the generic parameters cannot depend on other generic parameters. This
//! might get implemented eventually however.
//!

/// A macro for avoiding code duplication for immutable and mutable types.
/// Check out the crate level documentation for more information
#[macro_export]
macro_rules! impl_twice {
    () => {};
    (impl $(<$($gen_args:tt),*>)? $(where ($($where_args:tt)*))? { $($content:item)* }$($extra:tt)*) => {
        impl_twice!($($extra)*);
    };
    ({ $($content:item)* }$($extra:tt)*) => {
        impl_twice!($($extra)*);
    };
    (
        impl $(<$($gen_args:tt),*>)?
            $name:ident$(<$($name_param:tt),*>)?
            $(for $ename:ident$(<$($ename_param:tt),*>)?)?
            $(,
                $more_name:ident$(<$($more_name_param:tt),*>)?
                $(for $more_ename:ident$(<$($more_ename_param:tt),*>)?)?
            )*
        $(where ($($where_args:tt)*))?
        $(
            impl $(<$($gen_args2:tt),*>)?
                $name2:ident$(<$($name_param2:tt),*>)?
                $(for $ename2:ident$(<$($ename_param2:tt),*>)?)?
                $(,
                    $more_name2:ident$(<$($more_name_param2:tt),*>)?
                    $(for $more_ename2:ident$(<$($more_ename_param2:tt),*>)?)?
                )*
            $(where ($($where_args2:tt)*))?
        )*
        {
            $($content:item)*
        }
        $($extra:tt)*
    ) => {
        impl$(<$($gen_args),*>)? $name $(<$($name_param),*>)? $(for $ename$(<$($ename_param),*>)?)? $(where $($where_args)*)? {
            $($content)*
        }
        impl_twice!(
            impl $(<$($gen_args),*>)? $(
                $more_name$(<$($more_name_param),*>)?
                $(for $more_ename$(<$($more_ename_param),*>)?)?
            ),*
            $(where ($($where_args)*))?
            {
                $($content)*
            }
        );
        impl_twice!(
            $(
                impl $(<$($gen_args2),*>)?
                    $name2$(<$($name_param2),*>)?
                    $(for $ename2$(<$($ename_param2),*>)?)?
                    $(,
                        $more_name2$(<$($more_name_param2),*>)?
                        $(for $more_ename2$(<$($more_ename_param2),*>)?)?
                    )*
                $(where ($($where_args2)*))?
            )*
            {
                $($content)*
            }
        );
        impl_twice!($($extra)*);
    };
}
