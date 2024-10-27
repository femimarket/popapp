# Proof of Profit: Simulated Trading Environment

**Proof of Profit** is a proof-of-concept trading simulator for traders to practice and prove profitability. By demonstrating success in this simulated environment, traders may receive capital from investors. The project emphasizes privacy, price accuracy, and account abstraction using advanced tools.

## Key Features
- **Privacy & Security**: Trading data is protected within a Cycles-Quartz Trusted Execution Environment (TEE).
- **Price Feeds**: Slinky Neutron's native oracle is used to ensure reliable price data.
- **Account Abstraction**: Built on Abstract SDK to simplify and enhance account management.

---

## Setup Requirements
To run Proof of Profit, ensure you have:
- **LLVM** (not the default system version)
- **Neutrond**
- **Rust**
- **Additional System Libraries** (required for agnostic SGX mock, check for errors during setup)
- **Just** (CLI runner)

### Instructions

### 1. Smart Contracts
The frontend relies on smart contracts deployed to the testnet.
1. Deploy the smart contracts using:
   ```bash
   just publish-test
   ```
   Ensure you have testnet funds.

### 2. Frontend
1. Navigate to the `web` directory.
2. Start the frontend:
   ```bash
   npm run dev
   ```
   > **Note**: Frontend requires smart contracts to be active.

### 3. Cycles-Quartz (TEE Environment)
A private TEE environment (using Cycles-Quartz) allows local testing:
1. Build the Quartz CLI:
   ```bash
   cargo install --path crates/cli
   ```
2. From the root directory:
   ```bash
   just build-pop2-enclave
   just start-pop2-enclave
   ```
   Ensure a neutron key saved with the alias **admin**.

---

## Technical Highlights

### Abstract SDK Slinky Adapter
The adapter abstracts Slinky oracles, allowing core smart contract updates without oracle modifications. See the code [here](https://github.com/femimarket/popapp/blob/main/contracts/slinky/src/handlers/query.rs#L34-L49).

### Abstract SDK UI
The frontend integrates account creation via Abstract SDK, abstracting blockchain interactions. Implementation details are [here](https://github.com/femimarket/popapp/blob/main/web/apps/webapp/src/app/page.tsx#L88-L111).

### Cycles-Quartz Integration
A secure SGX TEE monitors the state and decrypts/encrypts data for secure communication between UI and blockchain. Examples:
- [State Decryption](https://github.com/femimarket/popapp/blob/main/cycles-quartz/examples/pop2-example/enclave/src/transfers_server.rs#L304-L333)
- [Smart Contract Deployment & Enclave Session](https://github.com/femimarket/popapp/blob/main/deploy-pop2-enclave.sh)

---

## Additional Resources
- [Cycles-Quartz TEE Documentation](https://github.com/informalsystems/cycles-quartz/blob/v0.1.0/docs/getting_started.md)
- [Presentation & Overview](https://www.canva.com/design/DAGUv1bYZxc/I5YzXyy12cMWg-GFGsDdIw/view?utm_content=DAGUv1bYZxc&utm_campaign=designshare&utm_medium=link&utm_source=editor)

---

## Future Vision
This protocol allows traders to prove profitability in a controlled environment, leading to potential investment and on-chain trading. The long-term goal is to create a versatile protocol that interacts with decentralized exchanges (DEXes) for liquidity and cross-chain compatibility. **Currently, the focus is on refining the simulated trading environment.**

