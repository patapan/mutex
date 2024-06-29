
#[derive(Accounts)]
pub struct CreateMarket<'info> {
    /// Address paying to create the pool. Can be anyone
    #[account(mut)]
    pub market_creator: Signer<'info>,

    /// Which config the pool belongs to.
    pub pool_config: Box<Account<'info, AmmConfig>>,

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
    pub pool_state: AccountLoader<'info, MarketState>,    
}