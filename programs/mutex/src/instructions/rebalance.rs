
use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2};

#[derive(Accounts)]
pub struct RebalanceMarket<'info> {
    /// Address paying to rebalance the market. Can be anyone
    #[account(mut)]
    pub market_creator: Signer<'info>,

    /// Which config the market belongs to.
    pub market_config: Box<Account<'info, AmmConfig>>,

    /// Initialize an account to store the market state
    #[account(
        init,
        seeds = [
            MARKET_SEED.as_bytes(),
            amm_config.key().as_ref(),
            token_mint_0.key().as_ref(),
            token_mint_1.key().as_ref(),
        ],
        bump,
        payer = market_creator,
        space = MarketState::LEN
    )]
    pub market_state: AccountLoader<'info, MarketState>,

    // Pyth oracle price feed account
    pub price_update: Account<'info, PriceUpdateV2>,
}

// Get the most recent price from Pyth
pub fn get_price(ctx: Context<RebalanceMarket>) -> Result<()> {
    let price_update = &mut ctx.accounts.price_update;
    // get_price_no_older_than will fail if the price update is more than 30 seconds old
    let maximum_age: u64 = 30;
    // get_price_no_older_than will fail if the price update is for a different price feed.
    // This string is the id of the BTC/USD feed. See https://pyth.network/developers/price-feed-ids for all available IDs.
    let feed_id: [u8; 32] = get_feed_id_from_hex("0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43")?;
    let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;
    // Sample output:
    // The price is (7160106530699 ± 5129162301) * 10^-8
    msg!("The price is ({} ± {}) * 10^{}", price.price, price.conf, price.exponent);
 
    Ok(())
}