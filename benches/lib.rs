#![feature(test)]

extern crate test;
extern crate labyrinth;

use test::Bencher;

use labyrinth::*;

#[bench]
fn bench_10_10(ben: &mut Bencher) {
    let mut wall = Wall::new(10, 10);
    ben.iter(|| {
        wall.close_all().carve();
        wall.to_string()
    });
}

#[bench]
fn bench_20_20(ben: &mut Bencher) {
    let mut wall = Wall::new(20, 20);
    ben.iter(|| {
        wall.close_all().carve();
        wall.to_string()
    });
}

#[bench]
fn bench_50_50(ben: &mut Bencher) {
    let mut wall = Wall::new(50, 50);
    ben.iter(|| {
        wall.close_all().carve();
        wall.to_string()
    });
}

#[bench]
fn bench_100_100(ben: &mut Bencher) {
    let mut wall = Wall::new(100, 100);
    ben.iter(|| {
        wall.close_all().carve();
        wall.to_string()
    });
}
