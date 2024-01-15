// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use sppark::{NTTInputOutputOrder, NTTDirection, NTTType};

extern "C" {
    fn compute_ntt(
        device_id: usize,
        inout: *mut core::ffi::c_void,
        lg_domain_size: u32,
        ntt_order: NTTInputOutputOrder,
        ntt_direction: NTTDirection,
        ntt_type: NTTType,
    ) -> sppark::Error;
}

/// Compute an in-place NTT on the input data.
#[allow(non_snake_case)]
pub fn NTT<T>(device_id: usize, inout: &mut [T], order: NTTInputOutputOrder) {
    let len = inout.len();
    if (len & (len - 1)) != 0 {
        panic!("inout.len() is not power of 2");
    }

    let err = unsafe {
        compute_ntt(
            device_id,
            inout.as_mut_ptr() as *mut core::ffi::c_void,
            len.trailing_zeros(),
            order,
            NTTDirection::Forward,
            NTTType::Standard,
        )
    };

    if err.code != 0 {
        panic!("{}", String::from(err));
    }
}

/// Compute an in-place iNTT on the input data.
#[allow(non_snake_case)]
pub fn iNTT<T>(device_id: usize, inout: &mut [T], order: NTTInputOutputOrder) {
    let len = inout.len();
    if (len & (len - 1)) != 0 {
        panic!("inout.len() is not power of 2");
    }

    let err = unsafe {
        compute_ntt(
            device_id,
            inout.as_mut_ptr() as *mut core::ffi::c_void,
            len.trailing_zeros(),
            order,
            NTTDirection::Inverse,
            NTTType::Standard,
        )
    };

    if err.code != 0 {
        panic!("{}", String::from(err));
    }
}

#[allow(non_snake_case)]
pub fn coset_NTT<T>(
    device_id: usize,
    inout: &mut [T],
    order: NTTInputOutputOrder,
) {
    let len = inout.len();
    if (len & (len - 1)) != 0 {
        panic!("inout.len() is not power of 2");
    }

    let err = unsafe {
        compute_ntt(
            device_id,
            inout.as_mut_ptr() as *mut core::ffi::c_void,
            len.trailing_zeros(),
            order,
            NTTDirection::Forward,
            NTTType::Coset,
        )
    };

    if err.code != 0 {
        panic!("{}", String::from(err));
    }
}

#[allow(non_snake_case)]
pub fn coset_iNTT<T>(
    device_id: usize,
    inout: &mut [T],
    order: NTTInputOutputOrder,
) {
    let len = inout.len();
    if (len & (len - 1)) != 0 {
        panic!("inout.len() is not power of 2");
    }

    let err = unsafe {
        compute_ntt(
            device_id,
            inout.as_mut_ptr() as *mut core::ffi::c_void,
            len.trailing_zeros(),
            order,
            NTTDirection::Inverse,
            NTTType::Coset,
        )
    };

    if err.code != 0 {
        panic!("{}", String::from(err));
    }
}

use std::time::Instant;

extern "C" {    
    fn bench_fr_add_cuda(device_id: usize, samples: usize, blocks: usize, threads: usize) -> i32;
    fn bench_fr_sub_cuda(device_id: usize, samples: usize, blocks: usize, threads: usize) -> i32;
    fn bench_fr_mul_cuda(device_id: usize, samples: usize, blocks: usize, threads: usize) -> i32;
}

pub fn bench_add_fr(samples: usize, blocks: usize, threads: usize) {
    unsafe {
        bench_fr_add_cuda(0, samples, blocks, threads);
    }
}

pub fn bench_sub_fr(samples: usize, blocks: usize, threads: usize) {
    unsafe {
        bench_fr_sub_cuda(0, samples, blocks, threads);
    }
}

pub fn bench_mul_fr(samples: usize, blocks: usize, threads: usize) {
    unsafe {
        bench_fr_mul_cuda(0, samples, blocks, threads);
    }
}

pub fn arith_run() {
    use std::str::FromStr;
    let bench_npow = std::env::var("ARITH_BENCH_NPOW").unwrap_or("6".to_string());
    let npoints_npow = usize::from_str(&bench_npow).unwrap();

    for blocks in [128, 256, 1024] {
        for threads in [128, 256, 1024] {
            for lg_domain_size in 2..=npoints_npow {
                let domain_size = 10_usize.pow(lg_domain_size as u32) as usize;
                let count =  domain_size * blocks * threads;
                let name = format!("FR ADD 10**{}*{}*{}", lg_domain_size, blocks, threads);
                let start = Instant::now();
                bench_add_fr(domain_size, blocks, threads);
                let elapsed = start.elapsed();
                println!(
                    "{} = {:?} o/us",
                    name,
                    (count as f32) / elapsed.as_micros() as f32,
                );

                let name = format!("FR MUL 10**{}*{}*{}", lg_domain_size, blocks, threads);
                let start = Instant::now();
                bench_mul_fr(domain_size, blocks, threads);
                let elapsed = start.elapsed();
                println!(
                    "{} = {:?} o/us",
                    name,
                    (count as f32) / elapsed.as_micros() as f32,
                );
            }
        }
    }
}
