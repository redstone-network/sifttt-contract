use anchor_lang::prelude::*;

declare_id!("BU5JMEZ6mwqjSBMWTrh2NF96SMHdjz5JU3nk526LjPdA");

#[program]
pub mod sifttt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let account = &mut ctx.accounts.account;
        account.borrow_utilization = 0;
        msg!("Account initialized with borrow_utilization = 0");
        Ok(())
    }

    pub fn deposit(ctx: Context<Operate>) -> Result<()> {
        msg!("Depositing to: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn withdraw(ctx: Context<Operate>) -> Result<()> {
        msg!("Withdrawing from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn borrow(ctx: Context<Operate>) -> Result<()> {
        msg!("Borrow from: {:?}", ctx.program_id);
        let account = &mut ctx.accounts.account;
        // 这里可以根据实际逻辑更新 borrow_utilization
        account.borrow_utilization += 10; // 示例
        msg!("Borrow utilization: {}", account.borrow_utilization);
        Ok(())
    }

    pub fn repay(ctx: Context<Operate>) -> Result<()> {
        msg!("Repay from: {:?}", ctx.program_id);
        let account = &mut ctx.accounts.account;
        // 这里可以根据实际逻辑减少 borrow_utilization
        account.borrow_utilization -= 5; // 示例
        msg!("Borrow utilization: {}", account.borrow_utilization);
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct AccountState {
    pub borrow_utilization: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8 // discriminator + borrow_utilization
    )]
    pub account: Account<'info, AccountState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Operate<'info> {
    #[account(mut)]
    pub account: Account<'info, AccountState>,
    pub user: Signer<'info>,
}
