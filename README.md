
## Mutex Capital Decentralised Liquid Leveraged Tokens

This repo constructs an innovative on-chain protocol designed to facilitate the trading of decentralised liquid leveraged tokens (DLLTs). DLLTs represent a tokenized leveraged position on an underlying asset such as BTC, ETH or SOL. 

### Design

#### Markets

Each market contains 2 pools, a BULL pool and BEAR pool, both at a set leverage. For example `SOLBULLX3` or `SOLBEARX3`. Each market is re-balanced by Mutex Capital every 15 minutes, with the funds moving between the pools as dictated by the difference between the previous price and the current oracle price of the asset.

The pools are incentivised to stay of equal value by the fact that the value of USDC moved between wallets is defined as % lost by the losing pool. For example, if we had a SOLBULLX3 pool containing $100 and a SOLBEARX3 pool containing $200, and the token moves up 10%, then 30% of the BEAR pool (the losing pool) is shifted to the BULL pool. This means that the BULL pull had asymmetric returns > 3X, despite only being exposed to a 3X bear risk. 

#### Deposits

Users may deposit USDC into a market pool and receive DLLTs in exchange, representing their position. 

#### Withdrawals

Users can exchange their DLLTs for USDC from the associated pool.

#### Liquidations

A liquidation event occurs when the value of a pool goes to 0 during a rebalancing. This causes a counter party risk for the profiting pool as not enough funds are able to be moved to cover the increase in value.

#### Fees

Mutex Capital takes a 1% fee on all volume in the markets.