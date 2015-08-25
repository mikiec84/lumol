/*
 * Cymbalum, Molecular Simulation in Rust
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/
#![allow(non_snake_case)]
//! Testing physical properties of a Lennard-Jones gaz of Helium.

extern crate cymbalum;
use self::cymbalum::*;

use std::path::Path;

fn setup() -> (Simulation, Universe) {
    let data_dir = Path::new(file!()).parent().unwrap();
    let configuration = data_dir.join("data").join("Helium.xyz");
    let mut universe = Universe::from_file(configuration.to_str().unwrap()).unwrap();
    universe.set_cell(UnitCell::cubic(10.0));
    universe.add_pair_interaction("He", "He",
        LennardJones{
            sigma: units::from(2.0, "A").unwrap(),
            epsilon: units::from(0.2, "kJ/mol").unwrap()
        }
    );

    let mut velocities = BoltzmanVelocities::new(units::from(300.0, "K").unwrap());
    velocities.init(&mut universe);

    let simulation = Simulation::new(
        MolecularDynamics::new(units::from(1.0, "fs").unwrap())
    );
    return (simulation, universe);
}

#[test]
fn constant_energy() {
    let (mut simulation, mut universe) = setup();

    let E_initial = universe.total_energy();
    simulation.run(&mut universe, 1000);
    let E_final = universe.total_energy();

    assert!(f64::abs(E_initial - E_final) < 1e-5);
}
