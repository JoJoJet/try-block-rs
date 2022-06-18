# try-blocks

Macro for stable try blocks that performs Ok-wrapping, and otherwise tries to
achieve feature parity with RFC 1859. The macro is compatible with any type
that implements the unstable `Any` trait through the use of type magic.

This crate is a fork of `try-block`, which has not been updated in four years at
the time of writing this. This fork adds Ok-wrapping and the promise of future
updates.

This crate is `no_std` compatible.

License: MIT
