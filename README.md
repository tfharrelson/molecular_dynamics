# molecular_dynamics
Rust project to build an MD solver that naturally mixes with deep learning crates, which allows users to easily define novel potentials with artificial intelligence and/or plain old physics

## Overview

Molecular dynamics is a powerful technique for understand complex biological and material systems. Most molecular dynamics tools, while extremely performant, are not very developer friendly.
This project aims to create a developer-friendly SDK-like experience for molecular dynamics simulations. It will be somewhat hackable, in a way that allows developers to flexibly experiment
with the physics in the simulation. The primary feature is a first-class treatment of ML/AI forcefields (using candle.rs); any arbitrary forcefield can be used as long as it has the correct
traits.

## Roadmap

- Finish up van der waals fluid simulator
- Implement deep learning forcefield framework utilizing candle
- Build framework for training deep forcefield from data
- Come up with simple test case
- Create python bindings using pyo3
- Build tutorial notebook in python
- Integrate forcefield evaluation with onnx runtime to interface with externally trained models
