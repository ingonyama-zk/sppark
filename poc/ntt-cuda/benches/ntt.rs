// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(feature = "bls12_377")]
use ark_bls12_377::{G1Affine, G2Affine};
#[cfg(feature = "bls12_381")]
use ark_bls12_381::{G1Affine, G2Affine};
#[cfg(feature = "bn254")]
use ark_bn254::G1Affine;
use ark_ff::BigInteger256;

use std::str::FromStr;

use ntt_cuda::*;
use sppark::*;

use msm_cuda::util;

fn criterion_benchmark(c: &mut Criterion) {
    let bench_npow = std::env::var("BENCH_NPOW").unwrap_or("28".to_string());
    let npoints_npow = i32::from_str(&bench_npow).unwrap();

    let mut group = c.benchmark_group("CUDA");
    group.sample_size(20);

    let name = format!("2**{}", npoints_npow);
    group.bench_function(name, |b| {
        let (mut points, mut scalars) =
        util::generate_points_scalars_cond::<G1Affine>(1usize << npoints_npow, false);

        b.iter(|| {
            // let domain_size = 1usize << lg_domain_size;

            // let domain = D::new(domain_size).unwrap();

            // let mut v = vec![];
            // for _ in 0..domain_size {
            //     v.push(T::rand(rng));
            // }

            // v.resize(domain.size(), T::zero());
            // let mut vtest = v.clone();

            // domain.fft_in_place(&mut v);
            ntt_cuda::NTT(0, &mut scalars.as_mut_slice(), NTTInputOutputOrder::RN);
            // assert!(vtest == v);

            // domain.ifft_in_place(&mut v);
            // ntt_cuda::iNTT(DEFAULT_GPU, &mut vtest, NTTInputOutputOrder::NN);
            // assert!(vtest == v);

            // ntt_cuda::NTT(DEFAULT_GPU, &mut vtest, NTTInputOutputOrder::NR);
            // ntt_cuda::iNTT(DEFAULT_GPU, &mut vtest, NTTInputOutputOrder::RN);
            // assert!(vtest == v);

            // domain.coset_fft_in_place(&mut v);
            // ntt_cuda::coset_NTT(DEFAULT_GPU, &mut vtest, NTTInputOutputOrder::NN);
            // assert!(vtest == v);

            // domain.coset_ifft_in_place(&mut v);
            // ntt_cuda::coset_iNTT(DEFAULT_GPU, &mut vtest, NTTInputOutputOrder::NN);
            // assert!(vtest == v);
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);
