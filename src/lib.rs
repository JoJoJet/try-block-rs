#![no_std]
//! # Notice
//!
//! This crate has been moved/renamed to [tryvial](https://github.com/JoJoJet/tryvial).
//! You should use that instead.
//!
//! ---
//!
//! Macro for stable try blocks that performs Ok-wrapping, and otherwise tries to
//! achieve feature parity with RFC 1859. The macro is compatible with any type
//! that implements the unstable `Try` trait through the use of type magic.
//!
//! This crate is `no_std` compatible.

/// Macro for ok-wrapping any `Try` type. This works on stable through dark type magic.
///
/// Note that type inference is very finicky; you should give this a type ascription ASAP.
/// ```
/// # use try_blocks::wrap_ok;
/// let r: Result<_, ()> = wrap_ok!(1);
/// assert_eq!(r, Ok(1));
/// ```
#[macro_export]
macro_rules! wrap_ok {
    ($e:expr) => {{
        ::core::iter::empty().try_fold($e, |_, x: core::convert::Infallible| match x {})
    }};
}

/// Macro for the recieving end of a `?` operation.
/// Right now, type inference is quite finicky so you usually have to declare a concrete type somewhere.
///
/// ```
/// # use try_blocks::try_block;
/// // Note: this fails without explicitly specifying the error type.
/// let y: Result<_, std::num::ParseIntError> = try_block! {
///     "1".parse::<i32>()? + "2".parse::<i32>()?
/// };
/// # assert_eq!(y, Ok(3));
/// ```
/// If you know that a block will have a specific type, it may aid type inference to use
/// the macros [`try_opt`], [`try_res`], and [`try_cf`].
///
/// ## Alternative
/// The only other way to emulate try blocks is with closures, which is very ugly.
///
/// #### Before:
/// ```ignore
/// let result: Result<T, E> = (|| {
///    let a = do_one(x)?;
///    let b = do_two(a)?;
///    Ok(b)
/// })();
/// ```
///
/// #### After:
/// ```
/// # use try_blocks::try_block;
/// # type T = (); type E = ();
/// # fn do_one((): T) -> Result<T, E> { Ok(()) }
/// # fn do_two((): T) -> Result<T, E> { Ok(()) }
/// # let x = ();
/// let result: Result<T, E> = try_block! {
///    let a = do_one(x)?;
///    let b = do_two(a)?;
///    b
/// };
/// ```
#[macro_export]
macro_rules! try_block {
    { $($token:tt)* } => {{
        #[allow(unused_mut)]
        let mut f = || $crate::wrap_ok!({ $($token)* });
        f()
    }}
}

/// Like [`try_block`], but specificially for [`Option`]. This aids type inference.
#[macro_export]
macro_rules! try_opt {
    { $($token:tt)* } => {{
        #[allow(unused_mut)]
        let mut f = || ::core::option::Option::Some({ $($token)* });
        f()
    }};
}

/// Like [`try_block`], but specificially for [`Result`]]. This aids type inference.
#[macro_export]
macro_rules! try_res {
    { $($token:tt)* } => {{
        #[allow(unused_mut)]
        let mut f = || ::core::result::Result::Ok({ $($token)* });
        f()
    }};
}

/// Like [`try_block`], but specificially for [`ControlFlow`]. This aids type inference.
#[macro_export]
macro_rules! try_cf {
    { $($token:tt)* } => {
        #[allow(unused_mut)]
        let mut f = || ::core::ops::ControlFlow::Continue({ $($token)* });
        f()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_sum() {
        let result: Result<_, core::num::ParseIntError> = try_block! {
            let x = "1".parse::<i32>()?;
            let x = "2".parse::<i32>()? + x * 10;
            "3".parse::<i32>()? + x * 10
        };
        assert_eq!(result, Ok(123));
    }

    #[test]
    fn option() {
        assert_eq!(
            Some(520),
            try_block! {
                "400".parse::<i32>().ok()? + "20".parse::<i32>().ok()? * "6".parse::<i32>().ok()?
            },
        );
    }

    #[test]
    fn named() {
        try_opt! {
            let x = "400".parse::<i32>().ok()?;
            let x = x.checked_add(6_900_000)?;
            assert_eq!(x, 6_900_400);
        }
        .unwrap();

        let res: Result<_, ()> = try_res! {
            1
        };
        assert_eq!(res, Ok(1));
    }
}
