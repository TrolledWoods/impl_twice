#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![no_std]

//! A crate for avoiding code duplication for immutable and mutable types.
//! Check out the [`impl_twice`] macro for more information.
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
//! # use std::fmt::Debug;
//! struct Type;
//! struct TypeMut;
//!
//! impl_twice!(
//!     // The types are separated by commas. There can only be exactly two
//!     // types.
//!     impl Type, TypeMut {
//!         fn hello(&self) {
//!             println!("Hello, World!");
//!         }
//!     }
//!
//!     // Traits work as well
//!     impl Default for Type, TypeMut {
//!         fn default() -> Self {
//!             Self
//!         }
//!     }
//! );
//!
//! struct GenericType<'a, T>(&'a T);
//! struct GenericTypeMut<'a, T>(&'a mut T);
//!
//! trait SomeTrait<T> {
//!     fn get(&self) -> &'_ T;
//! }
//!
//! impl_twice!(
//!     // Generics work as well.
//!     // However, where clauses need parenthesees around them
//!     impl<T> GenericType<'_, T>, GenericTypeMut<'_, T> where (T: Clone) {
//!         pub fn get(&self) -> &'_ T {
//!             self.0
//!         }
//!     }
//!
//!     // Implementing traits with generics works as well.
//!     // You specify the trait for each type that wants one.
//!     impl<T> Debug for GenericType<'_, T>,
//!             Debug for GenericTypeMut<'_, T>
//!         where (T: Debug)
//!     {
//!         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!             write!(f, "{:?}", self.0)
//!         }
//!     }
//!
//!     // You may want different generics or generic bounds on the two different types.
//!     // For that reason you can have two sets of generic parameters.
//!     // This is a bad example because there is actually no reason to do this in this
//!     // case, but it's here if you want it.
//!     impl<T> GenericType<'_, T> where (T: ToString)
//!     impl<T> GenericTypeMut<'_, T> where (T: ToString + Clone) {
//!         fn stuff(&self) -> String {
//!             self.0.to_string()
//!         }
//!     }
//!
//!     // // The above also works with traits. (this is also a bad example).
//!     impl<T> SomeTrait<T> for GenericType<'_, T>
//!     impl<T> SomeTrait<T> for GenericTypeMut<'_, T> where (T: Iterator) {
//!         fn get(&self) -> &'_ T {
//!             &self.0
//!         }
//!     }
//! );
//! ```
//!
//! # Limitations
//! The generics in these macros may look the same as generics on real impl blocks,
//! but they are much more limited. That is simply because there seems to be no good
//! way to do generics like this in macros yet. So for now, the generics you can do
//! are quite limited.
//!

/// A macro for avoiding code duplication for immutable and mutable types.
/// Check out the crate level documentation for more information
#[macro_export]
macro_rules! impl_twice {
    () => {};
    (impl $(<$($gen_args:tt),*>)? $(where ($($where_args:tt)*))? { $($content:item)* }$($extra:tt)*) => {
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
    ({$($x:item)*}) => {};
}
