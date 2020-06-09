#bool_ext
A crate which defines and implements a complete set of Boolean functional combinators.

## FAQ
### Aren't there already crates like this?
Yes, there are, notably Daniel Keep's [boolinator](https://crates.io/crates/boolinator), and 
Rust's [`then()` and `then_some()`](https://doc.rust-lang.org/std/primitive.bool.html#method.then
) methods (currently unstable at the time of this writing).

`boolinator` is a great crate and serves as an inspiration for this crate, as do some other sources.
However, `boolinator`'s provided set of combinators is not as complete as one might wish for (and, 
relatively minor nit, nor are the combinator names as short as they could be.  `boolinator`'s
interface is also stable (i.e. >v1.0.0) so this would be disruptive to address.)  

In Rust's case it has (and is expected to) take a long time to stabilize the names for the
functional combinators, as standardizing on the name has proven to be 
[difficult](https://github.com/rust-lang/rust/issues/64260).  Further, `std` is conservative by
 design, and will only very slowly move toward a complete set of combinators for `bool`.
 
My hope is that this crate can serve as a testing ground for both the naming and scope of
boolean combinators to help inform the Rust lang team's discussion and decisions with usage data
and feedback. 

### It's just a bool--why is the API surface so large?
Because `bool` is a very versatile data type!  For example, adding to a collection only if the
item in question is not already present is a common operation.  `Map` datatypes often give you
this behavior by the nature of their design.  Other containers such as `Vec`, do not.  So instead
of highly stateful, imperative code:
```rust
    // ...
    let mut found = false;
    for needle in haystack {
        if needle == item {
            found = true;
            break;
        }
    }    
    
    if !found {
        haystack.push(item);
    }
```
or imperative/declarative hybrid code:
```rust
    // ...
    if !haystack.contains(&item) {
        haystack.push(item)
    }
```
`bool_ext` enables the following highly expressive, highly cohesive, declarative code:
```rust
    // ...
    haystack.contains(&item)
            .do_false(|| haystack.push(item));
```

### Should I use this? / I'm not sure about method-chaining/functional combinators
You are not alone!  Debuggers have not yet caught up to fluent API design techniques and
debugging fluent interfaces can indeed be objectively more work.  On the other hand, proponents 
(such as me) will tell you that by elevating one's thinking from "micromanaging the CPU" to
expressing one's intent by "shaping the data", far fewer bugs will be written in the first place, 
and the resulting code will be both more expressive and more maintainable (once the maintaining 
party has sufficient experience with this style of coding).

`bool_ext` is implemented according to Bjarne Stroustrup's now classic definition of a zero
-overhead abstraction, where 1) you only pay for what you use and 2) you couldn't implement the
abstraction any better if you coded it yourself by hand.
 
Addressing 1), the `bool_ext` create is very small, takes no dependencies, and most importantly
, *any methods* defined within the crate that *you do not use* are stripped out by the compiler
and *are not a part of your resulting binary*.

Regarding 2), each method is `#[inline]`'d, adheres to the Single Responsibility Principle, 
minimizes register pressure from inlining and when fully optimized (typically in release mode) 
should compile down to exactly the same (or better) code that could be written by hand.

## License
Licensed under either:
    * MIT license (see LICENSE-MIT file)
    * Apache License, Version 2.0 (see LICENSE-APACHE file)
at your option.

## Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the 
work by you shall be dual licensed as above, without any additional terms or conditions.
