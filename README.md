# counting: Fast counting into a Rust slice
Bart Massey 2023

This is a repo of code linked from [this Reddit
thread](https://www.reddit.com/r/rust/comments/12hj0yq/how_fast_can_you_count_to_16_in_rust/). I
am not the original author of most of the code; it is
reproduced without permission, and I do not hold any
license.

The `main` branch contains the original "naïve" code. The
`fast-naive` branch contains a version of the "idiomatic
Rust" speedup given in the article. The `fast-simd` version
is the final "wide SIMD" code given in the article.

I found the following on my machine (AMD Ryzen 9 3900X,
2023-04-09, "nightly" is `1.70.0-nightly (478cbb42b 2023-03-28)`:

* "Naïve" version, without `--target-cpu`: ~570ns
* "Naïve" version, with `--target-cpu=native`: ~570ns
* Rust idiomatic (non-SIMD) version, without `--target-cpu`: ~310ns
* Rust idiomatic (non-SIMD) version, with `--target-cpu=native`: ~250ns
* Rust idiomatic (non-SIMD) version on nightly, with `--target-cpu=native`: ~250ns
* Final optimized wide SIMD version on nightly, without `--target-cpu`: ~250ns
* Final optimized wide SIMD version on nightly, with `--target-cpu=native`: ~180ns
