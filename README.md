
## Mutex Capital Decentralised Liquid Leveraged Tokens

This repo constructs an innovative on-chain protocol designed to facilitate the trading of decentralised liquid leveraged tokens (DLLTs). DLLTs represent a tokenized leveraged position on an underlying asset such as BTC, ETH or SOL. 

### Design

#### Markets

Each market contains 2 pools, a BULL pool and BEAR pool, both at a set leverage. For example `SOLBULLX3` or `SOLBEARX3`. Each market is re-balanced by Mutex Capital every 15 minutes, with the funds moving between the pools as dictated by the difference between the previous price and the current oracle price of the asset.

#### Re-balancing mechanism

The pools are incentivised to stay balanced by the fact that the amount of USDC transferred between pools is defined as `max(BULL pool value, BEAR pool value) * Δ%P`. For example, if we had a SOLBULLX3 pool containing $1 and a SOLBEARX3 pool containing $10000, and the token moves up 10%, then 30% of the BEAR pool ($3000) is shifted to the BULL pool. This means that the BULL pool has returns equal to 299900%! Users who capitalise these pool imbalances have the opportunity for significant reward.

#### Deposits

Users may deposit USDC into a market pool and receive DLLTs in exchange, representing their position. 

#### Withdrawals

Users can exchange their DLLTs for USDC from the associated pool. Users will receive the amount of USDC equivalent to their share of the total pool value. For example, if a user holds 10% of the minted DLLTs for a pool of value $1000, they can withdraw $100.

#### Liquidations

A liquidation event occurs when the value of a pool goes to 0 during a rebalancing. This causes a counter party risk for the profiting pool as not enough funds are able to be moved to cover the increase in value.

Mutex Capital is designed such that users DLLTs will be automatically disgarded if a liquidation occurs. This protects the future depositees into that position. With this in mind, **it is not possible for a single, individual user to be liquidated**- only an entire pool can be liquidated and all users tokens within it will be disgarded.

#### Fees

Mutex Capital takes a 1% fee on all deposits and withdrawals in the markets, with 0.5% going to LP providers and 0.5% going to the protocol fund. In addition, the protocol takes 0.01% of rebalancing funds to cover server fees.