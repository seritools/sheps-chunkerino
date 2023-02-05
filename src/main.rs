#![feature(slice_split_at_unchecked)]

use std::hint::black_box;

mod double_chunk;
mod on_demand;
mod precomputed;
mod seri;
mod seri2;
mod seri2_flipped;
mod seri2_slice;
mod seri2_slice_flipped;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let points = [
        // (1, 1),
        (100_000, 1),
        (100_000, 2),
        (100_000, 3),
        (100_000, 4),
        (100_000, 5),
        (100_000, 6),
        (100_000, 7),
        (100_000, 8),
        (100_000, 9),
        (100_000, 10),
        (100_000, 11),
        (100_000, 12),
        (100_000, 13),
        (100_000, 14),
        (100_000, 15),
        (100_000, 16),
        (100_000, 17),
        (100_000, 18),
        (100_000, 19),
        (100_000, 20),
        (100_000, 31),
        (100_000, 32),
        (100_000, 33),
        (100_000, 63),
        (100_000, 64),
        (100_000, 65),
        (100_000, 127),
        (100_000, 128),
        (100_000, 129),
        // (100_000, 100_000),
        // (10, 32),
        // (100, 32),
        // (1_000, 32),
        // (10_000, 32),
        // (1_000_000, 32),
        // (10_000_000, 32),
        // (100_000, 3),
        // (100_000, 32),
        // (100_000, 320),
        // (100_000, 3200),
        // (100_000, 32000),
        // (100_000, 320000),
    ];

    let data = points.map(|(l, n)| (vec![0u8; l], n, format!("len={l},n_chunks={n:06}")));

    let mut group = c.benchmark_group("Parts");

    for (data, n, label) in data {
        // group.bench_with_input(
        //     BenchmarkId::new("on_demand", &label),
        //     &(&data, n),
        //     |b, (data, n)| b.iter(|| on_demand::Parts::new(data, *n).map(black_box).count()),
        // );

        group.bench_with_input(
            BenchmarkId::new("precomputed", &label),
            &(&data, n),
            |b, (data, n)| b.iter(|| precomputed::Parts::new(data, *n).map(black_box).count()),
        );
        group.bench_with_input(
            BenchmarkId::new("double_chunk", &label),
            &(&data, n),
            |b, (data, n)| b.iter(|| double_chunk::Parts::new(data, *n).map(black_box).count()),
        );
        group.bench_with_input(
            BenchmarkId::new("seri", &label),
            &(&data, n),
            |b, (data, n)| b.iter(|| seri::Parts::new(data, *n).map(black_box).count()),
        );
        group.bench_with_input(
            BenchmarkId::new("seri2", &label),
            &(&data, n),
            |b, (data, n)| b.iter(|| seri2::Parts::new(data, *n).map(black_box).count()),
        );
        group.bench_with_input(
            BenchmarkId::new("seri2_flipped", &label),
            &(&data, n),
            |b, (data, n)| b.iter(|| seri2_flipped::Parts::new(data, *n).map(black_box).count()),
        );
        group.bench_with_input(
            BenchmarkId::new("seri2_slice", &label),
            &(&data, n),
            |b, (data, n)| b.iter(|| seri2_slice::Parts::new(data, *n).map(black_box).count()),
        );
        group.bench_with_input(
            BenchmarkId::new("seri2_slice_flipped", &label),
            &(&data, n),
            |b, (data, n)| {
                b.iter(|| {
                    seri2_slice_flipped::Parts::new(data, *n)
                        .map(black_box)
                        .count()
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

macro_rules! ext {
    () => {
        pub trait PartsExt<T> {
            /// Returns an iterator that divides the slice into a
            /// maximum of `n_chunks` chunks, starting at the
            /// beginning of the slice.
            ///
            /// The chunks are slices and do not overlap. If the slice
            /// cannot be evenly divided into `n_chunks` parts, some
            /// chunks will be one element longer than others. It is
            /// not guaranteed which chunks will be longer.
            ///
            /// See [`chunks`] for an iterator that returns chunks of
            /// a specified length, instead of a specified number of
            /// chunks.
            ///
            /// # Panics
            ///
            /// Panics if `n_chunks` is 0.
            ///
            /// # Examples
            ///
            /// ```
            /// let slice = ['l', 'o', 'r', 'e', 'm'];
            /// let mut iter = slice.parts(2);
            /// assert_eq!(iter.next().unwrap(), &['l', 'o', 'r']);
            /// assert_eq!(iter.next().unwrap(), &['e', 'm']);
            /// assert!(iter.next().is_none());
            /// ```
            ///
            /// [`chunks_exact`]: slice::chunks_exact
            /// [`rchunks`]: slice::rchunks

            fn parts(&self, n_chunks: usize) -> Parts<'_, T>;
        }

        impl<T> PartsExt<T> for [T] {
            fn parts(&self, n_chunks: usize) -> Parts<'_, T> {
                Parts::new(self, n_chunks)
            }
        }
    };
}
pub(crate) use ext;
