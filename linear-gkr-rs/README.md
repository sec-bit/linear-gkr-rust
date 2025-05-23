# Linear GKR Protocol Implementation in Rust

This repository contains a Rust implementation of the Linear GKR protocol, a zero-knowledge proof system for arithmetic circuits. The implementation is based on the original C++ code and includes both the prover and verifier components.

## Project Structure

The project is organized into several crates:

- `circuit/`: Circuit representation and parsing

  - Defines the `Circuit`, `Layer`, and `Gate` types
  - Provides parsing utilities for circuit files
  - Handles gate type definitions and circuit validation

- `field/`: Finite field arithmetic

  - Implements field operations using the `ark-ff` library
  - Provides the `FieldElement` type used throughout the codebase

- `polynomial/`: Polynomial operations

  - Implements polynomial arithmetic needed for the protocol
  - Includes multilinear extension utilities

- `prover/`: GKR prover implementation

  - Evaluates arithmetic circuits
  - Generates proof components
  - Implements the "slow track" evaluation strategy

- `verifier/`: GKR verifier implementation

  - Verifies proofs generated by the prover
  - Implements sum-check protocol
  - Performs multilinear extension evaluations

- `cli/`: Command-line interface
  - Provides the `slow_track` binary for testing
  - Handles circuit file loading and execution

## Circuit File Format

Circuit files use a simple text format:

```
<depth>
<num_gates_0> <ty_0> <id_0> <u_0> <v_0> ...
<num_gates_1> <ty_1> <id_1> <u_1> <v_1> ...
...
```

Where:

- `depth`: Number of layers in the circuit
- `num_gates_i`: Number of gates in layer i
- `ty`: Gate type (0=Input, 1=Add, 2=Mul, 3=Dummy)
- `id`: Gate ID (must be unique within layer)
- `u`, `v`: Input wire indices

## Building and Running

```bash
# Build all crates
cargo build --workspace

# Run the slow_track binary with a test circuit
cargo run --bin slow_track -- --circuit examples/test_circuit.txt
```

## Testing

```bash
# Run all tests
cargo test --workspace
```

## Dependencies

- `ark-ff`: Finite field arithmetic
- `thiserror`: Error handling
- `rand`: Random number generation

## Implementation Notes

- The implementation uses a "slow track" approach for simplicity and clarity
- Gate IDs must be unique within each layer
- Circuit layers must have at least one gate
- The implementation assumes power-of-two sized layers for efficient multilinear extension evaluation

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and ensure they pass
5. Submit a pull request

