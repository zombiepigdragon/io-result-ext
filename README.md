# io::Result Extensions

A collection of helper methods for the standard library's [IO result type](https://doc.rust-lang.org/std/io/type.Result.html).

## Adding new functions

This crate is far from complete!
If there are patterns you frequently use for `std::io::Result` that aren't included in this crate,
please [open an issue](https://github.com/zombiepigdragon/io-result-ext/issues/new) with an example or
[submit a PR](https://github.com/zombiepigdragon/io-result-ext/pulls).

## MSRV

This crate supports every version of Rust, starting with [`1.0.0`](https://doc.rust-lang.org/1.0.0/std/io/type.Result.html).
When it becomes necessary to break support for an old version of Rust,
a feature will be introduced corresponding to the MSRV required to use it.
Each of these features will depend on the prior version's feature,
so it is only necessary to enable the feature corresponding to the highest required MSRV.

## License

This library is licensed under the [CC0 1.0 License](LICENSE).
By submitting a PR to this repository, you agree to relinquish all copyright and other rights as required by CC0.