// Lumol, an extensible molecular simulation engine
// Copyright (C) 2015-2016 G. Fraux — BSD license

//! Data about bonds and angles in the system.
use std::cmp::{min, max};

/// A `Bond` between the particles at indexes `i` and `j`
///
/// This structure ensure an unique representation of a `Bond` by enforcing
/// `i < j`
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bond {
    i: usize,
    j: usize,
}

impl Bond {
    /// Create a new Bond between the particles at indexes `first` and `second`
    pub fn new(first: usize, second: usize) -> Bond {
        assert!(first != second);
        let i = min(first, second);
        let j = max(first, second);
        Bond{i: i, j: j}
    }

    /// Get the first particle in the bond
    #[inline] pub fn i(&self) -> usize {self.i}

    /// Get the second particle in the bond
    #[inline] pub fn j(&self) -> usize {self.j}
}

/// An `Angle` formed by the particles at indexes `i`, `j` and `k`
///
/// This structure ensure uniqueness of the `Angle` representation by enforcing
/// `i < k`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Angle {
    i: usize,
    j: usize,
    k: usize,
}

impl Angle {
    /// Create a new Angle between the particles at indexes `first`, `second` and `third`
    pub fn new(first: usize, second: usize, third: usize) -> Angle {
        assert!(first != second);
        assert!(first != third);
        assert!(second != third);
        let i = min(first, third);
        let k = max(first, third);
        Angle{i: i, j: second, k: k}
    }

    /// Get the first particle in the angle
    #[inline] pub fn i(&self) -> usize {self.i}

    /// Get the second particle in the angle
    #[inline] pub fn j(&self) -> usize {self.j}

    /// Get the third particle in the angle
    #[inline] pub fn k(&self) -> usize {self.k}
}


/// A `Dihedral` angle formed by the particles at indexes `i`, `j`, `k` and `m`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Dihedral {
    i: usize,
    j: usize,
    k: usize,
    m: usize,
}

impl Dihedral {
    /// Create a new Dihedral between the particles at indexes `first`, `second`,
    /// `third` and `fourth`
    pub fn new(first: usize, second: usize, third: usize, fourth: usize) -> Dihedral {
        assert!(first != second);
        assert!(second != third);
        assert!(third != fourth);
        let (i, j, k, m) = if max(first, second) < max(third, fourth) {
            (first, second, third, fourth)
        } else {
            (fourth, third, second, first)
        };
        Dihedral{i: i, j: j, k: k, m: m}
    }

    /// Get the first particle in the dihedral angle
    #[inline] pub fn i(&self) -> usize {self.i}

    /// Get the second particle in the dihedral angle
    #[inline] pub fn j(&self) -> usize {self.j}

    /// Get the third particle in the dihedral angle
    #[inline] pub fn k(&self) -> usize {self.k}

    /// Get the fourth particle in the dihedral angle
    #[inline] pub fn m(&self) -> usize {self.m}
}

mod connect {
    bitflags! {
        /// The `Connectivity` bitflag encode the topological distance between
        /// two particles in the molecule, i.e. the number of bonds between the
        /// particles.
        pub flags Connectivity: u8 {
            /// The particles are separated by one bond
            const CONNECT_12   = 0b0001,
            /// The particles are separated by two bonds
            const CONNECT_13   = 0b0010,
            /// The particles are separated by three bonds
            const CONNECT_14   = 0b0100,
            /// The particles are separated by more than three bonds
            const CONNECT_FAR  = 0b1000,
        }
    }

    impl Default for Connectivity {
        fn default() -> Connectivity {
            CONNECT_FAR
        }
    }
}

pub use self::connect::Connectivity;
pub use self::connect::{CONNECT_12, CONNECT_13, CONNECT_14, CONNECT_FAR};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bond() {
        let bond = Bond::new(2, 1);
        assert_eq!(bond.i, 1);
        assert_eq!(bond.j, 2);
    }

    #[test]
    fn angle() {
        let angle = Angle::new(8, 7, 6);
        assert_eq!(angle.i, 6);
        assert_eq!(angle.j, 7);
        assert_eq!(angle.k, 8);
    }

    #[test]
    fn dihedral() {
        let dihedral = Dihedral::new(8, 7, 6, 0);
        assert_eq!(dihedral.i, 0);
        assert_eq!(dihedral.j, 6);
        assert_eq!(dihedral.k, 7);
        assert_eq!(dihedral.m, 8);

        let dihedral = Dihedral::new(0, 7, 6, 8);
        assert_eq!(dihedral.i, 0);
        assert_eq!(dihedral.j, 7);
        assert_eq!(dihedral.k, 6);
        assert_eq!(dihedral.m, 8);
    }
}
