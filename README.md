# impl_twice
Use the same implementation block for two types

Written using rust, https://www.rust-lang.org/

## Examples
When writing rust programs, times come when you need two types,
one immutable and one mutable.

It is possible to remove this duplication with DSTs, such as the
standard library's slice type, where `&[T]` and `&mut [T]` are the
immutable/mutable counterparts. However, DSTs cannot be created
by the programmer, and therefore they are not always applicable.

When making two very similar types that are just immutable/mutable
counterparts to each other, you may have to implement the same
things on both of the types. Here is an example of the duplication;

```rust
struct WrappedSlice<'a, T>(&'a [T]);
struct WrappedSliceMut<'a, T>(&'a mut [T]);

impl<T> WrappedSlice<'_, T> {
    pub fn inner(&self) -> &'_ [T] {
        self.0
    }

    pub fn get(&self, index: usize) -> Option<&'_ T> {
        self.0.get(index)
    }
}

impl<T> WrappedSliceMut<'_, T> {
    pub fn inner(&self) -> &'_ [T] {
        self.0
    }

    pub fn get(&self, index: usize) -> Option<&'_ T> {
        self.0.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&'_ mut T> {
        self.0.get_mut(index)
    }
}
```

This can be solved by having a way to implement the same items on both
types. That's what this crate is designed for!
This is equivalent to the above example but implemented with this
crate;

```rust
# use impl_twice::impl_twice;
struct WrappedSlice<'a, T>(&'a [T]);
struct WrappedSliceMut<'a, T>(&'a mut [T]);

impl_twice! (
    impl<T> WrappedSlice<'_, T>, WrappedSliceMut<'_, T> {
        pub fn inner(&self) -> &'_ [T] {
            self.0
        }

        pub fn get(&self, index: usize) -> Option<&'_ T> {
            self.0.get(index)
        }
    }
);

impl<T> WrappedSliceMut<'_, T> {
    pub fn get_mut(&mut self, index: usize) -> Option<&'_ mut T> {
        self.0.get_mut(index)
    }
}
```

As you can see, the two methods ``inner`` and ``get`` that were duplicated
are now only implemented once.

## Building
Build this like any other rust crate, or add it as
a dependency in your project.

## Contributing
Feel free to post an issue or a pull request if you have any ideas.
