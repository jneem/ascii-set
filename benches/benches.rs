#![feature(test)]

extern crate ascii_set;
extern crate rand;
extern crate test;

use ascii_set::AsciiSet;
use rand::{Rng, thread_rng};
use std::char;
use test::Bencher;

fn rand_str(n: usize) -> String {
    thread_rng().gen_ascii_chars().take(n).collect()
}

fn bench_fn<F>(b: &mut Bencher, f: F) where F: Fn(&u8) -> bool {
    let input = rand_str(1024);
    b.bytes = 1024;
    b.iter(|| input.bytes().filter(&f).count());
}

#[bench]
fn lowercase_ascii_set(b: &mut Bencher) {
    let set = AsciiSet::lower_case_letters();
    bench_fn(b, |c| set.contains_byte(*c));
}

#[bench]
fn lowercase_match(b: &mut Bencher) {
    let f = |c: &u8| match *c {
        97...122 => true,
        _ => false,
    };
    bench_fn(b, f);
}

#[bench]
fn lowercase_is_lowercase(b: &mut Bencher) {
    let f = |c: &u8| char::from_u32((*c) as u32).unwrap().is_lowercase();
    bench_fn(b, f);
}

#[bench]
fn letter_ascii_set(b: &mut Bencher) {
    let set = AsciiSet::letters();
    bench_fn(b, |c| set.contains_byte(*c));
}

#[bench]
fn letter_is_alphabetic(b: &mut Bencher) {
    let f = |c: &u8| char::from_u32((*c) as u32).unwrap().is_alphabetic();
    bench_fn(b, f);
}

#[bench]
fn letter_match(b: &mut Bencher) {
    let f = |c: &u8| match *c {
        97...122 => true,
        65...90 => true,
        _ => false,
    };
    bench_fn(b, f);
}

#[bench]
fn alnum_ascii_set(b: &mut Bencher) {
    let set = AsciiSet::letters().union(&AsciiSet::digits());
    bench_fn(b, |c| set.contains_byte(*c));
}

#[bench]
fn alnum_is_alnum(b: &mut Bencher) {
    let f = |c: &u8| char::from_u32((*c) as u32).unwrap().is_alphanumeric();
    bench_fn(b, f);
}

#[bench]
fn alnum_match(b: &mut Bencher) {
    let f = |c: &u8| match *c {
        97...122 => true,
        65...90 => true,
        48...57 => true,
        _ => false,
    };
    bench_fn(b, f);
}


