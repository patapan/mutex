use anchor_lang::prelude::*;

declare_id!("9yC4hTQ84JaV3mTb9SXNNppMUVy5MNdAoQE7Ka4sieu5");

#[program]
pub mod mutex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
