//! A derive for hashmoji.
//!
//! Implements hashmoji::Debug and hashmoji::Display for structs and enums.
//!
//! By default [hashmoji::Debug] calls `hashmoji::fixed(value, 3)` which gives around 5B
//! combinations with the default emoji set.
//!
//! You can specify options to change this fixed value, use one(), or variable():
//! e.g. to use one():
//!
//! #[derive(hashmoji::Debug)]
//! #[Hashmoji(one)]
//!
//! or to use variable:
//! #[derive(hashmoji::Debug)]
//! #[Hashmoji(variable(2..5))]
