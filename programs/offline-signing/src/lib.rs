use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod offline_signing {
    use super::*;

    pub fn transfer_sol(ctx: Context<TransferSOL>, lamports: u64) -> Result<()> {
        let tr_instruction = transfer(&ctx.accounts.from.key(), &ctx.accounts.to.key(), lamports);
        invoke(
            &tr_instruction,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferSOL<'info> {
    #[account(mut, signer)]
    ///CHECK: This is not dangerous because we don't read or write from this account
    pub from: AccountInfo<'info>,
    #[account(mut)]
    ///CHECK: This is not dangerous because we don't read or write from this account
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}