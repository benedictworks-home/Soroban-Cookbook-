# Soroban Fungible Token dApp Template

A production-ready full-stack template for building and managing SEP-41 compliant fungible tokens on Stellar using Soroban and Freighter Wallet.

---

## 🌟 Features

- **SEP-41 Compliant Token Contract:** Includes `initialize`, `balance`, `transfer`, `transfer_from`, `approve`, `allowance`, `mint`, and `burn`.
- **Safe Storage & TTL Management:** Automated instance and balance TTL extensions preventing data archival.
- **Modern Responsive Web Client:** Built with modern CSS and Vanilla JS for instant setup without heavy framework overhead.
- **Freighter Wallet Integration:** Seamless connection, signing, and live transaction feedback.
- **Automated Deployment:** One-click deployment script with CLI integration.

---

## 📁 Project Structure

```
token-dapp/
├── contracts/
│   └── token/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs       # Token smart contract logic
│           └── test.rs      # Unit test suite
├── frontend/
│   ├── index.html           # Main UI dashboard
│   ├── package.json         # Web dependencies
│   └── src/
│       ├── app.js           # Wallet connector & contract client
│       └── styles.css       # Modern dark-mode styling
├── scripts/
│   └── deploy.sh            # Build, deploy & initialization script
├── Cargo.toml               # Rust workspace config
└── README.md                # Project documentation
```

---

## 🚀 Getting Started

### 1. Test the Smart Contract
```bash
cd contracts/token
cargo test
```

### 2. Deploy to Testnet
```bash
# Configure deployer identity
stellar keys generate deployer --network testnet
stellar keys fund deployer --network testnet

# Run automated deployment
chmod +x scripts/deploy.sh
./scripts/deploy.sh
```

### 3. Run the Frontend
```bash
cd frontend
npm install
npm run dev
```

Open `http://localhost:3000` to view your live token portal!
