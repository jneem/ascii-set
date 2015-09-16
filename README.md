ascii-set
=========

A mini-crate for representing sets of ASCII characters with fast membership testing. This crate was inspired by this [reddit discussion](https://www.reddit.com/r/rust/comments/3l3bnn/eliminating_branches_in_rust_for_fun_but_not_much/).

[![Build status](https://travis-ci.org/jneem/ascii-set.svg)](https://travis-ci.org/jneem/ascii-set)

[Documentation](http://jneem.github.io/ascii-set/ascii_set/index.html)

# Benchmarks

The following table lists the results of the benchmarks in `benches/bench.rs`. The first part of the benchmark name states the set of characters that we are testing membership of. For benchmarks beginning with "alnum", the set of characters in question is the set of all alphanumeric characters. Benchmarks beginning with "letter" use the set of all letters, and benchmarks beginning with "lowercase" use the set of lowercase letters.

The second part of the benchmark name shows the method that we are using to test membership. "ascii_set" is using the `AsciiSet` class from this crate; "is_xxx" uses the appropriate method on the `std::char` type; "match" uses a match statement.

The benchmarks show that `ascii_set` is not always the most efficient implementation of character membership testing. When the set is very simple (e.g. just a range, or the union of two ranges) then other methods are slightly better. Once we get to the union of three ranges, however, then `ascii_set` is faster. As the sets get more complicated, the gap should widen.

```
test alnum_ascii_set        ... bench:       2,962 ns/iter (+/- 152) = 345 MB/s
test alnum_is_alnum         ... bench:       3,763 ns/iter (+/- 490) = 272 MB/s
test alnum_match            ... bench:       4,043 ns/iter (+/- 414) = 253 MB/s
test letter_ascii_set       ... bench:       4,132 ns/iter (+/- 381) = 247 MB/s
test letter_is_alphabetic   ... bench:       3,421 ns/iter (+/- 486) = 299 MB/s
test letter_match           ... bench:       3,406 ns/iter (+/- 352) = 300 MB/s
test lowercase_ascii_set    ... bench:       6,077 ns/iter (+/- 428) = 168 MB/s
test lowercase_is_lowercase ... bench:       5,660 ns/iter (+/- 426) = 180 MB/s
test lowercase_match        ... bench:       5,257 ns/iter (+/- 923) = 194 MB/s
```
