# Proof of Profit - README

**Proof of Profit** is a simulated trading platform designed to help traders prove their profitability and earn capital from investors. This proof-of-concept project prioritizes privacy and security using Cycles-Quartz for a trusted execution environment (TEE), integrates price feeds from Slinky Neutron's native oracle, and utilizes account abstraction through the Abstract SDK.

## Features
- **Privacy-First Trading**: Protected trading environment leveraging Cycles-Quartz TEE.
- **Real-Time Price Feeds**: Slinky Neutron’s oracle for accurate, live price data.
- **Account Abstraction**: Simplified and secure account interactions using Abstract SDK.

## Getting Started

### Prerequisites
Ensure the following dependencies are installed on your system:
- LLVM (custom install, not default system)
- Neutrond
- Rust
- Required system libraries (for agnostic SGX mock support)
- `just` (CLI task runner)

### Installation

1. **Smart Contracts**:
  - Deploy contracts to the testnet:
    ```bash
    just publish-test
    ```
  - This builds and deploys contracts, and generates schemas needed for the frontend. Ensure you have testnet funds available.

2. **Frontend**:
  - Navigate to the `web` directory and start the development server:
    ```bash
    npm run dev
    ```
  - Note: The frontend requires deployed contracts to function properly.

3. **Cycles-Quartz (Blackbox TEE)**:
  - To set up a local TEE mock environment:
    - Build the Quartz CLI:
      ```bash
      cargo install --path crates/cli
      ```
    - Refer to [Cycles-Quartz documentation](https://github.com/informalsystems/cycles-quartz/blob/v0.1.0/docs/getting_started.md) for further details.
    - From the root directory, build and start the local SGX enclave:
      ```bash
      just build-pop2-enclave
      just start-pop2-enclave
      ```
  - **Note**: Ensure a Neutron key with alias `admin` is configured.

### Important Notes
This project uses separate smart contracts for the frontend and TEE to establish a foundation for core features: privacy, account abstraction, and oracle integration. It is intended as an initial step toward a fully functional trading application.

---

