# Governance Examples

On-chain governance patterns: voting systems, delegation, multisig control, and DAO treasury management.

## 📂 Examples

### [01-delegation-marketplace](./01-delegation-marketplace/)
A marketplace for listing and renting voting power with incentive mechanisms.
- **Concepts:** `require_auth()`, persistent storage, typed errors, event topics, fee transfer.

## Quick Start

| # | Example | Concepts |
|---|---------|----------|
| 01 | [simple-voting](./01-simple-voting/) | Proposal creation, one-address-one-vote, time-based deadlines, vote tallying, execution |
| 02 | [voting-time-constraints](./01-voting-time-constraints/) | Voting periods, proposal deadlines, grace periods, early closure |
| 06 | [timelock-governance](./06-timelock-governance/) | Proposal queue, mandatory delays, veto, emergency execution |
| 07 | [automatic-snapshot-triggers](./07-automatic-snapshot-triggers/) | Time-based & event-based snapshots, snapshot pruning, gas-efficient storage |
```bash
cd examples/governance/01-simple-voting
cargo test && cargo build --target wasm32-unknown-unknown --release
```

## Examples by Pattern

### Voting Fundamentals

| # | Example | Focus | Concepts |
|---|---------|-------|----------|
| 01 | [simple-voting](./01-simple-voting/) | 🟢 Beginner | Proposal creation, one-address-one-vote, time-based deadlines, tally, execution |
| 02 | [token-voting](./02-token-voting/) | 🟡 Intermediate | Balance snapshots, token-weighted voting, flash-loan resistance |
| 02 | [voting-time-constraints](./02-voting-time-constraints/) | 🟡 Intermediate | Voting periods, grace periods, quorum thresholds, early closure |

### Delegation & Authority

| # | Example | Focus | Concepts |
|---|---------|-------|----------|
| 01 | [vote-delegation](./01-vote-delegation/) | 🟡 Intermediate | Liquid delegation, chain traversal, cycle detection, recursion limits |
| 01 | [proposal-validation](./01-proposal-validation/) | 🟡 Intermediate | Proposal creation gates, validator patterns, pre-voting checks |

### Advanced Governance

| # | Example | Focus | Concepts |
|---|---------|-------|----------|
| 03 | [proposal-lifecycle](./03-proposal-lifecycle/) | 🟠 Advanced | Full state machine: Draft → Active → Queued → Executed/Defeated, veto paths |
| 03 | [dao-treasury](./03-dao-treasury/) | 🟠 Advanced | Multisig fund management, timelock on withdrawals, role-based access |
| 05 | [delegation](./05-delegation/) | 🟠 Advanced | Enhanced delegation with revocation, time-bounds, and re-delegation |
| 06 | [timelock-governance](./06-timelock-governance/) | 🟠 Advanced | Mandatory delays, veto windows, emergency bypass, queue management |

## Pattern Progression

**Beginner → Production:**

1. **Start here** — `01-simple-voting` — Learn vote lifecycle and auth patterns
2. **Add governance token** — `02-token-voting` — Balance-weighted voting
3. **Formal phases** — `02-voting-time-constraints` — Voting periods and quorum
4. **Complex workflows** — `03-proposal-lifecycle` — Full state machine
5. **Production security** — `06-timelock-governance` + `03-dao-treasury` — Multisig + timelock + treasury

## Documentation

See [`docs/governance-patterns.md`](../../docs/governance-patterns.md) for:
- Pattern explanations with code examples
- When to use each pattern
- Security considerations and threat models
- Voting system comparison table
- Deployment checklist

See [`docs/governance-rbac-multisig-timelock.md`](../../docs/governance-rbac-multisig-timelock.md) for:
- RBAC, multisig, and timelock foundations
- Combined governance flows
- Role hierarchy design
- Signer and threshold recommendations

## Next Steps

- Read [`docs/governance-patterns.md`](../../docs/governance-patterns.md) for pattern overview
- Start with `01-simple-voting` for basic voting flow
- Combine `02-token-voting` + `06-timelock-governance` for production DAO
- See [`examples/basics/03-authentication/`](../basics/03-authentication/) for RBAC foundation
- See [`examples/basics/04-events/`](../basics/04-events/) for event patterns used in governance contracts
