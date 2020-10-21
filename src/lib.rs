#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used
)]
#![warn()]
#![allow(
    clippy::implicit_return,
    clippy::iter_nth_zero,
    clippy::match_bool,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
#![forbid(bare_trait_objects)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing
// license files and more
#![allow(clippy::blanket_clippy_restriction_lints)]
#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
#![allow(clippy::implicit_return)]
//! `bool_ext` is a crate which defines and implements a complete set of Boolean functional
//! combinators.  See this crate's `README.md` for more background.

/// `BoolExt` trait defines and implements a complete set of Boolean functional combinators.
pub trait BoolExt {
    /// # Boolean to `Option` (`bool` => `Option<T>`) adapters

    /// ## Transforms `true` => `Some(())`, `false` => `None`
    /// ### Examples:
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&2).as_option() == Some(()));
    /// ```
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&4).as_option() == None);
    /// ```
    fn as_option(self) -> Option<()>;

    /// ## Transforms `true` => `Some(T)`, `false` => `None`
    /// ### Examples:
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&2).some(Foo) == Some(Foo));
    /// ```
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&4).some(Foo) == None);
    /// ```
    fn some<T>(self, some: T) -> Option<T>;

    /// ## Transforms `true` => `Some(T)`, `false` => `None`, lazily evaluated
    /// ### Examples:
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// fn expensive_computation() -> Foo {
    ///     // ...expensive computation
    ///     Foo
    /// }
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&2).some_with(|| expensive_computation()) == Some(Foo));
    /// ```
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// fn expensive_computation() -> Foo {
    ///     // ...some expensive computation
    ///     Foo
    /// }
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// // elide `expensive_computation()`
    /// assert!(vec.contains(&4).some_with(|| expensive_computation()) == None);
    /// ```
    fn some_with<F: FnOnce() -> T, T>(self, some: F) -> Option<T>;

    /// `bool` => `Result<T, E>`
    /// ## Transforms `true` => `Ok(())`, `false` => `Err(())`
    /// ### Examples:
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&2).as_result() == Ok(()));
    /// ```
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&4).as_result() == Err(()));
    /// ```
    fn as_result(self) -> Result<(), ()>;

    /// ## Transforms `true` => `Ok(T)`, `false` => `Err(())`
    /// ### Examples:
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&2).ok(Foo) == Ok(Foo));
    /// ```
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&4).ok(Foo) == Err(()));
    /// ```
    fn ok<T>(self, ok: T) -> Result<T, ()>;

    /// ## Transforms `true` => `Ok(T)`, `false` => `Err(())`, lazily evaluated
    /// ### Examples:
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// fn expensive_computation() -> Foo {
    ///     // ...expensive computation
    ///     Foo
    /// }
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&2).ok_with(|| expensive_computation()) == Ok(Foo));
    /// ```
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// fn expensive_computation() -> Foo {
    ///     // ...some expensive computation
    ///     Foo
    /// }
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// // elide `expensive_computation()`
    /// assert!(vec.contains(&4).ok_with(|| expensive_computation()) == Err(()));
    /// ```
    fn ok_with<F: FnOnce() -> T, T>(self, ok: F) -> Result<T, ()>;

    /// ## Transforms `true` => `Ok(T)`, `false` => `Err(E)`
    /// ### Examples:
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    /// use std::fmt::Formatter;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Error;
    ///
    /// impl std::error::Error for Error {}
    ///
    /// impl std::fmt::Display for Error {
    ///     fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ///         write!(f, "BoolExt Example Error")
    ///     }
    /// }
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&2).ok_or_err(Foo, Error) == Ok(Foo));
    /// ```
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Error;
    ///
    /// impl std::error::Error for Error {}
    ///
    /// impl std::fmt::Display for Error {
    ///     fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ///         write!(f, "BoolExt Example Error")
    ///     }
    /// }
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&4).ok_or_err(Foo, Error) == Err(Error));
    /// ```
    fn ok_or_err<T, E>(self, ok: T, err: E) -> Result<T, E>;

    /// ## Transforms `true` => `Ok(T)`, `false` => `Err(E)`, lazily evaluated
    /// ### Examples:
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// fn expensive_ok_computation() -> Foo {
    ///     // ...expensive computation
    ///     Foo
    /// }
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Bar;
    ///
    /// fn expensive_err_computation() -> Bar {
    ///     // ...expensive computation
    ///     Bar
    /// }
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&2).ok_or_err_with(
    ///     || expensive_ok_computation(),
    ///     || expensive_err_computation()) == Ok(Foo));
    /// ```
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// fn expensive_ok_computation() -> Foo {
    ///     // ...some expensive computation
    ///     Foo
    /// }
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Bar;
    ///
    /// fn expensive_err_computation() -> Bar {
    ///     // ...expensive computation
    ///     Bar
    /// }
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// // elide `expensive_computation()`
    /// assert!(vec.contains(&4).ok_or_err_with(
    ///     || expensive_ok_computation(),
    ///     || expensive_err_computation()) == Err(Bar));
    /// ```
    fn ok_or_err_with<F: FnOnce() -> T, G: FnOnce() -> E, T, E>(
        self,
        ok: F,
        err: G,
    ) -> Result<T, E>;

    /// `bool` => `T`
    /// ## Transforms `true` => `T`, `false` => `T`
    /// ### Examples:
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    /// use std::fmt::Formatter;
    ///
    /// #[derive(Debug, PartialEq)]
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&2).map_or(0, || {
    ///     //... some computation
    ///     42
    /// }) == 42);
    /// ```
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    /// use std::fmt::Formatter;
    ///
    /// #[derive(Debug, PartialEq)]
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&4).map_or(None, || {
    ///     //... some computation
    ///     Some(42)
    /// }) == None);
    /// ```
    fn map_or<F: FnOnce() -> T, T>(self, default: T, t: F) -> T;

    /// `bool` => `T`
    /// ## Transforms `true` => `T`, `false` => `T::default()`
    /// ### Examples:
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    /// use std::fmt::Formatter;
    ///
    /// #[derive(Debug, PartialEq)]
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&2).map_or_default(|| {
    ///     //... some computation
    ///     42
    /// }) == 42);
    /// ```
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    /// use std::fmt::Formatter;
    ///
    /// #[derive(Debug, PartialEq)]
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&4).map_or_default(|| {
    ///     //... some computation
    ///     Some(42)
    /// }) == None);
    /// ```
    fn map_or_default<F: FnOnce() -> T, T: Default>(self, t: F) -> T;

    /// `bool` => `T`
    /// ## Transforms `true` => `T`, `false` => `T`
    /// ### Examples:
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    /// use std::fmt::Formatter;
    ///
    /// #[derive(Debug, PartialEq)]
    /// enum SetPresence {
    ///     In,
    ///     Out,
    /// }
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&2).map_or_else(
    ///     || SetPresence::In,
    ///     || SetPresence::Out) == SetPresence::In);
    /// ```
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    /// use std::fmt::Formatter;
    ///
    /// #[derive(Debug, PartialEq)]
    /// enum SetPresence {
    ///     In,
    ///     Out,
    /// }
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert!(vec.contains(&4).map_or_else(
    ///     || SetPresence::In,
    ///     || SetPresence::Out) == SetPresence::Out);
    /// ```
    fn map_or_else<F: FnOnce() -> T, G: FnOnce() -> T, T>(self, t: F, f: G) -> T;

    /// ## Perform side-effect if `true`, otherwise do nothing
    /// ### Examples:
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let mut vec = vec![1, 2, 3];
    /// vec.contains(&2).and_do(|| vec.iter_mut().for_each(|el| *el = -*el));
    /// assert!(vec.eq(&[-1, -2, -3]));
    /// ```
    fn and_do<F: FnOnce()>(self, t: F) -> bool;

    /// ## Perform side-effect if `false`, otherwise do nothing
    /// ### Examples:    
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let mut vec = vec![1, 2, 3];
    /// vec.contains(&4).or_do(|| vec.push(4));
    /// assert!(vec.eq(&[1, 2, 3, 4]));
    /// ```
    fn or_do<F: FnOnce()>(self, f: F) -> bool;

    /// ## Perform fallible side-effect if `true`, otherwise do nothing
    /// ### Examples:
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Clone, Debug, PartialEq)]
    /// enum SomeError {}
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let mut vec = vec![1, 2, 3];
    /// vec.contains(&2).and_try_do(|| {
    ///     vec.iter_mut().for_each(|el| *el = -*el);
    ///     Ok::<_, SomeError>(())
    /// });
    /// assert!(vec.eq(&[-1, -2, -3]));
    /// ```
    fn and_try_do<F: FnOnce() -> Result<(), E>, E>(self, t: F) -> Result<bool, E>;

    /// ## Perform fallible side-effect if `false`, otherwise do nothing
    /// ### Examples:    
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Clone, Debug, PartialEq)]
    /// enum SomeError {}
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let mut vec = vec![1, 2, 3];
    /// vec.contains(&4).or_try_do(|| {
    ///     vec.push(4);
    ///     Ok::<_, SomeError>(())
    /// });
    /// assert!(vec.eq(&[1, 2, 3, 4]));
    /// ```
    fn or_try_do<F: FnOnce() -> Result<(), E>, E>(self, f: F) -> Result<bool, E>;

    /// ## Transforms `false` => `panic!()`
    /// ## panic with message if `false`, otherwise do nothing
    /// ### Examples:    
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let mut vec = vec![1, 2, 3];
    ///
    /// let res = std::panic::catch_unwind(|| {
    ///     vec.contains(&2).expect("Test expected `true`, but found `false`");
    /// });
    /// // Did not panic
    /// assert!(res.is_ok());
    /// ```
    /// ```
    /// use assert2::assert;
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let mut vec = vec![1, 2, 3];
    /// let res = std::panic::catch_unwind(|| {
    ///     vec.contains(&4).expect("Test expected `true`, but found `false`");
    /// });
    /// // Panicked
    /// assert!(res.is_err());
    /// ```
    fn expect(self, msg: &str);
}

// Suppress clippy::use_self warning arising from use of `panic!()`
#[allow(clippy::use_self)]
impl BoolExt for bool {
    #[inline]
    fn as_option(self) -> Option<()> {
        match self {
            true => Some(()),
            false => None,
        }
    }

    #[inline]
    fn some<T>(self, some: T) -> Option<T> {
        match self {
            true => Some(some),
            false => None,
        }
    }

    #[inline]
    fn some_with<F: FnOnce() -> T, T>(self, some: F) -> Option<T> {
        match self {
            true => Some(some()),
            false => None,
        }
    }

    #[inline]
    fn as_result(self) -> Result<(), ()> {
        match self {
            true => Ok(()),
            false => Err(()),
        }
    }

    #[inline]
    fn ok<T>(self, ok: T) -> Result<T, ()> {
        match self {
            true => Ok(ok),
            false => Err(()),
        }
    }

    #[inline]
    fn ok_with<F: FnOnce() -> T, T>(self, ok: F) -> Result<T, ()> {
        match self {
            true => Ok(ok()),
            false => Err(()),
        }
    }

    #[inline]
    fn ok_or_err<T, E>(self, ok: T, err: E) -> Result<T, E> {
        match self {
            true => Ok(ok),
            false => Err(err),
        }
    }

    #[inline]
    fn ok_or_err_with<F: FnOnce() -> T, G: FnOnce() -> E, T, E>(
        self,
        ok: F,
        err: G,
    ) -> Result<T, E> {
        match self {
            true => Ok(ok()),
            false => Err(err()),
        }
    }

    #[inline]
    fn map_or<F: FnOnce() -> T, T>(self, default: T, t: F) -> T {
        match self {
            true => t(),
            false => default,
        }
    }

    #[inline]
    fn map_or_default<F: FnOnce() -> T, T: Default>(self, t: F) -> T {
        self.map_or(T::default(), t)
    }

    #[inline]
    fn map_or_else<F: FnOnce() -> T, G: FnOnce() -> T, T>(self, t: F, f: G) -> T {
        match self {
            true => t(),
            false => f(),
        }
    }

    #[inline]
    fn and_do<F: FnOnce()>(self, t: F) -> bool {
        match self {
            true => t(),
            false => (),
        }
        self
    }

    #[inline]
    fn or_do<F: FnOnce()>(self, f: F) -> bool {
        !(!self).and_do(f)
    }

    #[inline]
    fn and_try_do<F: FnOnce() -> Result<(), E>, E>(self, t: F) -> Result<bool, E> {
        match self {
            true => {
                t()?;
                Ok(self)
            }
            false => Ok(self),
        }
    }

    #[inline]
    fn or_try_do<F: FnOnce() -> Result<(), E>, E>(self, f: F) -> Result<bool, E> {
        (!self).and_try_do(f).map(|_| self)
    }

    #[inline]
    fn expect(self, msg: &str) {
        #[allow(clippy::panic)]
        match self {
            true => (),
            false => panic!("{}", msg),
        }
    }
}
