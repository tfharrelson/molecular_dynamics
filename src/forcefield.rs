use std::collections::{HashMap, HashSet};

use candle_core::Tensor;

use crate::molecules::Atom;

pub struct VanderWaalsParameters(f64, f64);

// goal of this package is to provide an interface for allowing atoms to move via a forcefield
pub struct VanderWaalsForceField {
    // this field maps a pair of atoms (e.g. C-C onto a set of parameters
    pub pairwise_potentials: Vec<PairwisePotential>,
}

pub struct PairwisePotential {
    pub atoms: HashSet<Atom>,
    pub parameters: [f64],
}

pub const VDW_12_6: VanderWaalsForceField = construct_force_field(
    vec![(Carbon, Hydrogen), (Carbon, Carbon)],
    vec![(1.0, 2.0), (1.34, 5.3)],
);

// TODO: remove hashset of atom in pairwise potential, change to tuple of 2 atoms
// then add some Comparable trait to PP to make it so the order of the PP doesn't matter
fn construct_force_field(
    atom_pairs: Vec<(Atom, Atom)>,
    parameters: Vec<(f64, f64)>,
) -> VanderWaalsForceField {
    // start by converting atom pairs to hash sets
    let atom_hashes: Vec<HashSet<Atom>> = atom_pairs
        .iter()
        // there must be a better way to convert a tuple to a hash set in rust
        .map(|(a1, a2)| HashSet::from_iter(vec![a1, a2].iter()))
        .collect();

    let potentials: Vec<PairwisePotential> = atom_hashes
        .iter()
        .zip(parameters.iter())
        .map(|(pair, param)| PairwisePotential {
            atoms: pair,
            parameters: param,
        })
        .collect();

    return VanderWaalsForceField {
        pairwise_potentials: potentials,
    };
}

pub trait ForceField {
    pub fn calculate_force(&self, atom_positions: MicroState) -> Tensor {}
}

impl ForceField for VanderWaalsForceField {
    pub fn calculate_force(&self, atom_positions: MicroState) -> Tensor {
        // apply a 12-6 potential each of the microstates
        // first find all the unique atom combinations
        // TODO: apply a cutoff distance/decimation strategy to exclude atom combos that are too
        // far away

    }
}

pub struct MicroState {
    pub positions: Tensor,
    pub atoms: Vec<Atom>,
    pub velocities: Tensor,
    pub time: f64,
}

impl MicroState {
    fn unique_atom_combinations(&self) -> Vec<(Atom, Atom, Tensor, Tensor)> {
        self.atoms.iter().zip(self.positions.to_vec2()?.iter()).map(|(a, p) 
    }
}
