// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_std::UniformRand;

use rayon::prelude::*;

pub fn generate_points_scalars<G: AffineCurve>(
    len: usize,
) -> (Vec<G>, Vec<G::ScalarField>) {
    generate_points_scalars_cond(len, true)
}

pub fn generate_points_scalars_cond<G: AffineCurve>(
    len: usize,
    is_with_p: bool,
) -> (Vec<G>, Vec<G::ScalarField>) {
    let scalars_len = len;
    let mut len = if is_with_p { len } else { 3 };
    let rand_gen: usize = std::cmp::min(1usize << 11, len);
    let mut rng = ChaCha20Rng::from_entropy();

    let mut points =
        <G::Projective as ProjectiveCurve>::batch_normalization_into_affine(
            &(0..rand_gen)
                .map(|_| G::Projective::rand(&mut rng))
                .collect::<Vec<_>>(),
        );
    // Sprinkle in some infinity points
    if len > 3 {
        points[3] = G::zero();
    }

    while points.len() < len {
        points.append(&mut points.clone());
    }

    points.truncate(len);

    let scalars = (0..scalars_len)
        .into_par_iter()
        .map(|_| {
            let mut rng = ChaCha20Rng::from_entropy();
            G::ScalarField::rand(&mut rng)
        })
        .collect::<Vec<_>>();

    (points, scalars)
}
