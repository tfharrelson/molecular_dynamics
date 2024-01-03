use std::collections::HashSet;

pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
    pub positions: [(f64, f64, f64)],
    pub atom_clusters: Vec<AtomCluster>,
}

pub enum Atom {
    // make sure to get all the biologically relevant ones plus some interesting ones
    H,
    N,
    C,
    O,
    F,
    P,
    S,
    Cl,
    Na,
    Mg,
    Se,
    K,
    Fe,
    Ca,
}

pub struct Bond {
    pub indices: (usize, usize),
    pub bond_type: BondType,
}

pub enum BondType {
    // focus on covalent bonding for now
    Single,
    Double,
    Triple,
    Aromatic,
}

// TODO: consider making this a tuple struct for brevity
pub struct IndexedAtom {
    pub atom: Atom,
    pub index: isize,
}

pub struct AtomCluster {
    // defined by an atom with its nearest neighbors
    pub current: IndexedAtom,
    pub neighbors: Vec<IndexedAtom>,
}

impl AtomCluster {
    fn triplets(&self) -> Option<Vec<HashSet<usize>>> {
        match self.neighbors.len() {
            3 => {
                let hs1 = self.build_hash_set(
                    self.current.index,
                    self.neighbors.0.index,
                    self.neighbors.1.index,
                );
                let hs2 = self.build_hash_set(
                    self.current.index,
                    self.neighbors.1.index,
                    self.neighbors.2.index,
                );
                let hs3 = self.build_hash_set(
                    self.current.index,
                    self.neighbors.0.index,
                    self.neighbors.2.index,
                );

                return Some(vec![hs1, hs2, hs3]);
            }
            2 => {
                let hs = self.build_hash_set(
                    self.current.index,
                    self.neighbors.0.index,
                    self.neighbors.1.index,
                );
                return Some(vec![hs]);
            }
            _ => return None,
        }
    }
    fn build_hash_set(&self, elem1: usize, elem2: usize, elem3: usize) -> HashSet<usize> {
        let mut hs = HashSet::new();
        hs.insert(elem1);
        hs.insert(elem2);
        hs.insert(elem3);
        return hs;
    }
}

impl Molecule {
    fn angle_triplets(&self) -> HashSet<HashSet<usize>> {
        // map over each atom cluster and get triplets for each
        let triplet_list = self
            .atom_clusters
            .map(|clus| clus.triplets())
            .filter(|val| val != Null)
            .map(|Some(val)| val);
        return HashSet::from_iter(triplet_list);
    }
    /*
    fn angles(&self) -> Vec<f64> {
        // find all the triplets in the bonds
    }
    fn dihedrals(&self) -> Vec<f64> {
        // find all the dihedrals in the molecule
    }
    */
}
