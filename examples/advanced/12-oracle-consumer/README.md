# Oracle Consumer Patterns

Oracle *providers* publish data. Oracle *consumers* have to decide whether that data is safe to act on — and that decision is where most oracle incidents actually happen.

This example contains three deployable consumer contracts, each showing a different way to consume a price feed, plus the shared interface and validation helpers they all use.

## What You'll Learn

- Validating a feed read before using it: freshness, sanity bounds, and clock skew
- Separating the expensive write path from the cheap read path with a validated cache
- Surviving a broken provider with `try_*` cross-contract calls
- Reducing several disagreeing feeds to one number a quorum agrees on
- Gating a value transfer behind a deviation circuit breaker
- Why a contract cannot both return an error and remember that it did

## Layout

| Crate | Contract | Data usage pattern |
|-------|----------|--------------------|
| [`common/`](./common/) | *(interface only — not deployed)* | `Quote`, the `PriceFeed` trait, `ConsumerError`, and the three shared checks |
| [`guarded/`](./guarded/) | `GuardedConsumer` | Pull, validate, cache, serve from cache |
| [`aggregating/`](./aggregating/) | `AggregatingConsumer` | Fan out to N feeds, drop bad ones, require a quorum, take the median |
| [`settlement/`](./settlement/) | `SettlementConsumer` | Gate a state change behind a deviation circuit breaker |

Each consumer is its own crate because each is its own deployed contract: `#[contractimpl]` exports one Wasm symbol per function, so two contracts with an `initialize` in the same crate will not link.

## The Feed Interface

All three consumers talk to feeds through one trait, so any contract exposing `quote(asset) -> Quote` can be plugged in:

```rust
#[contractclient(name = "PriceFeedClient")]
pub trait PriceFeed {
    fn quote(env: Env, asset: Symbol) -> Quote;
}

pub struct Quote {
    pub price: i128,   // scaled by PRICE_SCALE (1.0 == 10_000_000)
    pub timestamp: u64,
}
```

Feeds with a different shape — such as the separate `get_value` / `get_timestamp` calls of [`03-oracle-pattern`](../03-oracle-pattern/) — fit behind a small adapter contract that implements this trait and forwards the two reads.

## Consumer 1 — Validate Once, Serve From Cache

```
refresh()  ──▶  feed.quote()  ──▶  freshness + bounds  ──▶  cache
                                          │
                                          ├── price()               (cache ≤ max_age)
                                          └── price_or_last_known() (cache ≤ fallback_max_age)
```

`refresh` is permissionless — anybody may pay to keep the cache warm, and a caller cannot influence *what* gets cached, only whether a validated value is written. Business logic then calls `price`, which makes no cross-contract call at all.

The degraded read is a separate entry point on purpose. Callers opt into stale data; nothing falls back implicitly.

```rust
let config = read_config(&env)?;
let quote = PriceFeedClient::new(&env, &config.feed).quote(&config.asset);
check_freshness(&env, &quote, config.max_age)?;
check_bounds(quote.price, config.min_price, config.max_price)?;
env.storage().instance().set(&GuardKey::Cached, &quote);
```

Rotating the feed with `set_feed` clears the cache: a value validated against the old feed says nothing about the new one.

## Consumer 2 — Quorum Across Redundant Feeds

```
feeds ──▶ try_quote each ──▶ drop unreachable ──▶ drop stale ──▶ drop out-of-bounds
                                                                        │
                                                    survivors ≥ min_responses ? median : QuorumNotMet
```

The `try_quote` call is what makes this resilient. A feed that traps, whose instance entry has expired, or that no longer exposes `quote` is skipped instead of reverting the whole read:

```rust
let Ok(Ok(quote)) = PriceFeedClient::new(&env, &feed).try_quote(&config.asset) else {
    continue;
};
```

The median — not the mean — is the reduction: one compromised feed cannot move a median, but it can move a mean arbitrarily far.

`usable_prices` exposes the survivors so monitoring can see which providers are healthy before the quorum is missed.

## Consumer 3 — Circuit Breaker Around a State Change

The first two consumers only *read* a price. This one lets a price move value, so a single bad tick mints credit that cannot be un-minted.

```
settle() ──▶ breaker open?  ──▶ CircuitOpen
         └─▶ freshness + bounds
         └─▶ deviation from last accepted price > max_deviation_bps ? DeviationTooLarge
         └─▶ credit = amount × price / PRICE_SCALE     (checked)

trip_if_deviated()  ──▶ keeper call; trips the breaker and *keeps* the write
reset_breaker(price) ──▶ admin re-anchors explicitly
```

### The pitfall this example exists to show

An invocation that returns an error has **all of its storage writes rolled back**. So `settle` cannot reject a bad tick *and* record that it saw one — the breaker flag would vanish along with the error.

Tripping therefore lives in `trip_if_deviated`, a permissionless keeper call that returns `Ok` and so keeps its write. `settle` rejects; the keeper remembers. Getting this backwards produces a breaker that silently never trips, and the tests would still pass if they only asserted on the error.

Recovery is deliberately manual: `reset_breaker` takes the price the admin is re-anchoring to, so resuming is an auditable decision rather than a side effect of whatever the feed reports next.

## Error Codes

| Code | Name | Meaning |
|------|------|---------|
| 1 | `AlreadyInitialized` | `initialize` called more than once |
| 2 | `NotInitialized` | Contract not yet initialized |
| 3 | `Unauthorized` | Caller is not the stored admin |
| 4 | `InvalidConfig` | Unusable configuration, or a non-positive settlement amount |
| 5 | `StaleData` | Quote is past the window, or dated in the future |
| 6 | `PriceOutOfBounds` | Quote outside the sanity bounds, or not positive |
| 7 | `QuorumNotMet` | Fewer usable quotes than `min_responses` |
| 8 | `NoCachedValue` | No validated value has ever been cached |
| 9 | `DeviationTooLarge` | Price moved further than `max_deviation_bps` |
| 10 | `CircuitOpen` | Breaker is tripped; admin must reset |
| 11 | `ArithmeticOverflow` | A price calculation would have wrapped |
| 12 | `FeedNotFound` | Feed is not registered |

## Best Practices

- **Never trust a raw read.** Check freshness, sanity bounds, and that the timestamp is not in the future. A future timestamp means the feed's clock disagrees with the ledger, and trusting it lets a misconfigured feed keep a stale value alive forever.
- **Bound the price, not just the age.** A fresh quote of `0` or `10^30` is still a bad quote.
- **Use `try_*` when a feed may legitimately fail** so one broken provider cannot take the consumer down with it — but only where skipping is genuinely safe. `usable_prices` skips; `settle` does not.
- **Use checked arithmetic everywhere a price appears.** Prices are attacker-influenced inputs.
- **Prefer a median over a mean** when aggregating independent sources.
- **Make degradation explicit.** A separate entry point for stale data beats a silent fallback.
- **Remember that error paths roll back.** Anything the contract must remember about a rejection has to happen in a call that succeeds.
- **Re-anchor manually after a breaker trips.** Automatic recovery re-arms exactly the risk the breaker was there to catch.

## Running Tests

```bash
cargo test -p oracle-consumer-common
cargo test -p guarded-oracle-consumer
cargo test -p aggregating-oracle-consumer
cargo test -p settlement-oracle-consumer
```

## Building

```bash
cargo build --target wasm32v1-none --release \
  -p guarded-oracle-consumer \
  -p aggregating-oracle-consumer \
  -p settlement-oracle-consumer
```

## Related Examples

- [`03-oracle-pattern`](../03-oracle-pattern/) — the single-source provider these consumers read from
- [`06-price-oracle`](../06-price-oracle/) — multi-updater provider with median aggregation and TWAP
- [`03-data-aggregation-oracle`](../03-data-aggregation-oracle/) — provider-side outlier filtering and manipulation detection
- [`04-circuit-breaker`](../04-circuit-breaker/) — the pause-and-recover pattern generalised
- [`docs/cross-contract-patterns.md`](../../../docs/cross-contract-patterns.md) — integration tips for multi-contract systems
