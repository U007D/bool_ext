// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
#![forbid(bare_trait_objects)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
#![warn(clippy::unused_self)]
#![allow(
    clippy::implicit_return,
    clippy::iter_nth_zero,
    clippy::match_bool,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing
// license files and more
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
    /// use bool_ext::BoolExt;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&2).as_option(), Some(()));
    /// ```
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&4).as_option(), None);
    /// ```
    fn as_option(&self) -> Option<()>;

    /// ## Transforms `true` => `None`, `false` => `Some(())`
    /// ### Examples:
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&2).as_option_false(), None);
    /// ```
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&4).as_option_false(), Some(()));
    /// ```
    fn as_option_false(&self) -> Option<()>;

    /// ## Transforms `true` => `Some(T)`, `false` => `None`
    /// ### Examples:
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&2).some(Foo), Some(Foo));
    /// ```
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&4).some(Foo), None);
    /// ```
    fn some<T>(&self, some: T) -> Option<T>;

    /// ## Transforms `true` => `None`, `false` => `Some(T)`
    /// ### Examples:
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&2).some_false(Foo), None);
    /// ```
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&4).some_false(Foo), Some(Foo));
    /// ```
    fn some_false<T>(&self, some: T) -> Option<T>;

    /// ## Transforms `true` => `Some(T)`, `false` => `None`, lazily evaluated
    /// ### Examples:
    /// ```
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
    /// assert_eq!(vec.contains(&2).some_with(|| expensive_computation()), Some(Foo));
    /// ```
    /// ```
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
    /// assert_eq!(vec.contains(&4).some_with(|| expensive_computation()), None);
    /// ```
    fn some_with<F: FnOnce() -> T, T>(&self, some: F) -> Option<T>;

    /// ## Transforms `true` => `None`, `false` => `Some(T)`, lazily evaluated
    /// ```
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
    /// // elide `expensive_computation()`
    /// assert_eq!(vec.contains(&2).some_with_false(|| expensive_computation()), None);
    /// ```
    /// ```
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
    /// assert_eq!(vec.contains(&4).some_with_false(|| expensive_computation()), Some(Foo));
    /// ```
    fn some_with_false<F: FnOnce() -> T, T>(&self, some: F) -> Option<T>;

    /// `bool` => `Result<T, E>`
    /// ## Transforms `true` => `Ok(())`, `false` => `Err(())`
    /// ### Examples:
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&2).as_result(), Ok(()));
    /// ```
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&4).as_result(), Err(()));
    /// ```
    fn as_result(&self) -> Result<(), ()>;

    /// ## Transforms `true` => `Err(())`, `false` => `Ok(())`
    /// ### Examples:
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&2).as_result_false(), Err(()));
    /// ```
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&4).as_result_false(), Ok(()));
    /// ```
    fn as_result_false(&self) -> Result<(), ()>;

    /// ## Transforms `true` => `Ok(T)`, `false` => `Err(())`
    /// ### Examples:
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&2).ok(Foo), Ok(Foo));
    /// ```
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&4).ok(Foo), Err(()));
    /// ```
    fn ok<T>(&self, ok: T) -> Result<T, ()>;

    /// ## Transforms `true` => `Ok(T)`, `false` => `Err(())`, lazily evaluated
    /// ### Examples:
    /// ```
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
    /// assert_eq!(vec.contains(&2).ok_with(|| expensive_computation()), Ok(Foo));
    /// ```
    /// ```
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
    /// assert_eq!(vec.contains(&4).ok_with(|| expensive_computation()), Err(()));
    /// ```
    fn ok_with<F: FnOnce() -> T, T>(&self, ok: F) -> Result<T, ()>;

    /// ## Transforms `true` => `Ok(())`, `false` => `Err(E)`
    /// ### Examples:
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&2).ok_false(Foo), Err(Foo));
    /// ```
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let vec = vec![1, 2, 3];
    ///
    /// assert_eq!(vec.contains(&4).ok_false(Foo), Ok(()));
    /// ```
    fn ok_false<E>(&self, err: E) -> Result<(), E>;

    /// ## Transforms `true` => `Ok(())`, `false` => `Err(E)`, lazily evaluated
    /// ### Examples:
    /// ```
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
    /// assert_eq!(vec.contains(&2).ok_with(|| expensive_computation()), Ok(Foo));
    /// ```
    /// ```
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
    /// assert_eq!(vec.contains(&4).ok_with(|| expensive_computation()), Err(()));
    /// ```
    fn ok_false_with<F: FnOnce() -> E, E>(&self, err: F) -> Result<(), E>;

    /// ## Transforms `true` => `Ok(T)`, `false` => `Err(E)`
    /// ### Examples:
    /// ```
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
    /// assert_eq!(vec.contains(&2).ok_or_err(Foo, Error), Ok(Foo));
    /// ```
    /// ```
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
    /// assert_eq!(vec.contains(&4).ok_or_err(Foo, Error), Err(Error));
    /// ```
    fn ok_or_err<T, E>(&self, ok: T, err: E) -> Result<T, E>;

    /// ## Transforms `true` => `Ok(T)`, `false` => `Err(E)`, lazily evaluated
    /// ### Examples:
    /// ```
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
    /// assert_eq!(vec.contains(&2).ok_or_err_with(
    ///     || expensive_ok_computation(),
    ///     || expensive_err_computation()), Ok(Foo));
    /// ```
    /// ```
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
    /// assert_eq!(vec.contains(&4).ok_or_err_with(
    ///     || expensive_ok_computation(),
    ///     || expensive_err_computation()), Err(Bar));
    /// ```
    fn ok_or_err_with<F: FnOnce() -> T, G: FnOnce() -> E, T, E>(
        &self,
        ok: F,
        err: G,
    ) -> Result<T, E>;

    /// ## Transforms `true` => `Err(E)`, `false` => `Ok(T)`
    /// ### Examples:
    /// ```
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
    /// assert_eq!(vec.contains(&2).ok_or_err_false(Foo, Error), Err(Error));
    /// ```
    /// ```
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
    /// assert_eq!(vec.contains(&4).ok_or_err_false(Foo, Error), Ok(Foo));
    /// ```
    fn ok_or_err_false<T, E>(&self, ok: T, err: E) -> Result<T, E>;

    /// ## Transforms `true` => `Ok(T)`, `false` => `Err(E)`, lazily evaluated
    /// ### Examples:
    /// ```
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
    /// assert_eq!(vec.contains(&2).ok_or_err_false_with(
    ///     || expensive_ok_computation(),
    ///     || expensive_err_computation()), Err(Bar));
    /// ```
    /// ```
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
    /// assert_eq!(vec.contains(&4).ok_or_err_false_with(
    ///     || expensive_ok_computation(),
    ///     || expensive_err_computation()), Ok(Foo));
    /// ```
    fn ok_or_err_false_with<F: FnOnce() -> T, G: FnOnce() -> E, T, E>(
        &self,
        ok: F,
        err: G,
    ) -> Result<T, E>;

    /// `bool` => `T`
    /// ## Transforms `true` => `T`, `false` => `T`
    /// ### Examples:
    /// ```
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
    /// assert_eq!(vec.contains(&2).map(|| SetPresence::In, || SetPresence::Out), SetPresence::In);
    /// ```
    /// ```
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
    /// assert_eq!(vec.contains(&4).map(|| SetPresence::In, || SetPresence::Out), SetPresence::Out);
    /// ```
    fn map<F: FnOnce() -> T, G: FnOnce() -> T, T>(&self, t: F, f: G) -> T;

    /// ## Perform side-effect if `true`, otherwise do nothing
    /// ### Examples:
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let mut vec = vec![1, 2, 3];
    /// vec.contains(&2).do_true(|| vec.iter_mut().for_each(|el| *el = -*el));
    /// assert!(vec.eq(&[-1, -2, -3]));
    /// ```
    fn do_true<F: FnOnce()>(&self, t: F) -> bool;

    /// ## Perform side-effect if `false`, otherwise do nothing
    /// ### Examples:    
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let mut vec = vec![1, 2, 3];
    /// vec.contains(&4).do_false(|| vec.push(4));
    /// assert!(vec.eq(&[1, 2, 3, 4]));
    /// ```
    fn do_false<F: FnOnce()>(&self, f: F) -> bool;

    /// ## Transforms `false` => `panic!()`
    /// ## panic with message if `false`, otherwise do nothing
    /// ### Examples:    
    /// ```
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
    fn expect(&self, msg: &str);

    /// ## Transforms `true` => `panic!()`
    /// ## panic with message if `true`, otherwise do nothing
    /// ### Examples:    
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let mut vec = vec![1, 2, 3];
    ///
    /// let res = std::panic::catch_unwind(|| {
    ///     vec.contains(&4).expect_false("Test expected `false`, but found `true`");
    /// });
    /// // Did not panic
    /// assert!(res.is_ok());
    /// ```
    /// ```
    /// use bool_ext::BoolExt;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo;
    ///
    /// let mut vec = vec![1, 2, 3];
    /// let res = std::panic::catch_unwind(|| {
    ///     vec.contains(&2).expect_false("Test expected `false`, but found `true`");
    /// });
    /// // Panicked
    /// assert!(res.is_err());
    /// ```
    fn expect_false(&self, msg: &str);
}

// Suppress clippy::use_self warning arising from use of `panic!()`
#[allow(clippy::use_self)]
impl BoolExt for bool {
    #[inline]
    fn as_option(&self) -> Option<()> {
        match self {
            true => Some(()),
            false => None,
        }
    }

    #[inline]
    fn as_option_false(&self) -> Option<()> {
        (!self).as_option()
    }

    #[inline]
    fn some<T>(&self, some: T) -> Option<T> {
        match self {
            true => Some(some),
            false => None,
        }
    }

    #[inline]
    fn some_false<T>(&self, some: T) -> Option<T> {
        (!self).some(some)
    }

    #[inline]
    fn some_with<F: FnOnce() -> T, T>(&self, some: F) -> Option<T> {
        match self {
            true => Some(some()),
            false => None,
        }
    }

    #[inline]
    fn some_with_false<F: FnOnce() -> T, T>(&self, some: F) -> Option<T> {
        (!self).some_with(some)
    }

    #[inline]
    fn as_result(&self) -> Result<(), ()> {
        match self {
            true => Ok(()),
            false => Err(()),
        }
    }

    #[inline]
    fn as_result_false(&self) -> Result<(), ()> {
        (!self).as_result()
    }

    #[inline]
    fn ok<T>(&self, ok: T) -> Result<T, ()> {
        match self {
            true => Ok(ok),
            false => Err(()),
        }
    }

    #[inline]
    fn ok_with<F: FnOnce() -> T, T>(&self, ok: F) -> Result<T, ()> {
        match self {
            true => Ok(ok()),
            false => Err(()),
        }
    }

    #[inline]
    fn ok_false<E>(&self, err: E) -> Result<(), E> {
        match self {
            true => Err(err),
            false => Ok(()),
        }
    }

    #[inline]
    fn ok_false_with<F: FnOnce() -> E, E>(&self, err: F) -> Result<(), E> {
        match self {
            true => Ok(()),
            false => Err(err()),
        }
    }

    #[inline]
    fn ok_or_err<T, E>(&self, ok: T, err: E) -> Result<T, E> {
        match self {
            true => Ok(ok),
            false => Err(err),
        }
    }

    #[inline]
    fn ok_or_err_with<F: FnOnce() -> T, G: FnOnce() -> E, T, E>(
        &self,
        ok: F,
        err: G,
    ) -> Result<T, E> {
        match self {
            true => Ok(ok()),
            false => Err(err()),
        }
    }

    #[inline]
    fn ok_or_err_false<T, E>(&self, ok: T, err: E) -> Result<T, E> {
        (!self).ok_or_err(ok, err)
    }

    #[inline]
    fn ok_or_err_false_with<F: FnOnce() -> T, G: FnOnce() -> E, T, E>(
        &self,
        ok: F,
        err: G,
    ) -> Result<T, E> {
        (!self).ok_or_err_with(ok, err)
    }

    #[inline]
    fn map<F: FnOnce() -> T, G: FnOnce() -> T, T>(&self, t: F, f: G) -> T {
        match self {
            true => t(),
            false => f(),
        }
    }

    #[inline]
    fn do_true<F: FnOnce() -> ()>(&self, t: F) -> bool {
        match self {
            true => t(),
            false => (),
        }
        *self
    }

    #[inline]
    fn do_false<F: FnOnce()>(&self, f: F) -> bool {
        (!self).do_true(f)
    }

    #[inline]
    fn expect(&self, msg: &str) {
        match self {
            true => (),
            false => panic!("{}", msg),
        }
    }

    #[inline]
    fn expect_false(&self, msg: &str) {
        (!self).expect(msg)
    }
}
