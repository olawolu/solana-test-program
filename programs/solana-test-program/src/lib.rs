use anchor_lang::prelude::*;

declare_id!("5jiVgpH21KbLTnnAwTZaLaHUChNWueGUMaRYqbNgKQJn");

#[program]
pub mod solana_test_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
