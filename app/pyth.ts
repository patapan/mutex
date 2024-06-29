import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";

// You will need a Connection from @solana/web3.js and a Wallet from @coral-xyz/anchor to create
// the receiver.
const connection: Connection;
const wallet: Wallet;
const pythSolanaReceiver = new PythSolanaReceiver({ connection, wallet });

// There are up to 2^16 different accounts for any given price feed id.
// The 0 value below is the shard id that indicates which of these accounts you would like to use.
// However, you may choose to use a different shard to prevent Solana congestion on another app from affecting your app.
const solUsdPriceFeedAccount = pythSolanaReceiver
  .getPriceFeedAccountAddress(0, SOL_PRICE_FEED_ID)
  .toBase58();
