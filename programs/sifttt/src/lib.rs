use anchor_lang::prelude::*;

declare_id!("BU5JMEZ6mwqjSBMWTrh2NF96SMHdjz5JU3nk526LjPdA");

#[program]
pub mod sifttt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let account = &mut ctx.accounts.account;
        account.health_factor = 100; // 初始健康因子为100
        account.trigger_health_factor = 0; // 默认不触发
        account.target_health_factor = 0; // 默认不设目标
        account.automation_enabled = false;
        msg!("Account initialized with health_factor = 100");
        Ok(())
    }

    pub fn set_automation(
        ctx: Context<Operate>, 
        trigger_health_factor: u64, 
        target_health_factor: u64
    ) -> Result<()> {
        let account = &mut ctx.accounts.account;
        require!(target_health_factor > trigger_health_factor, ErrorCode::InvalidHealthFactors);
        
        account.trigger_health_factor = trigger_health_factor;
        account.target_health_factor = target_health_factor;
        account.automation_enabled = true;
        
        msg!("Automation set: trigger={}, target={}", trigger_health_factor, target_health_factor);
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
        // 借贷增加健康因子风险，所以减少健康因子值
        account.health_factor = account.health_factor.saturating_sub(10);
        msg!("Health factor after borrow: {}", account.health_factor);
        
        // 检查是否需要自动化保护
        Self::check_automation(account)?;
        Ok(())
    }

    pub fn repay(ctx: Context<Operate>) -> Result<()> {
        msg!("Repay from: {:?}", ctx.program_id);
        let account = &mut ctx.accounts.account;
        // 还款改善健康因子，所以增加健康因子值
        account.health_factor += 5;
        msg!("Health factor after repay: {}", account.health_factor);
        Ok(())
    }

    pub fn auto_repay(ctx: Context<Operate>) -> Result<()> {
        let account = &mut ctx.accounts.account;
        require!(account.automation_enabled, ErrorCode::AutomationNotEnabled);
        require!(
            account.health_factor <= account.trigger_health_factor,
            ErrorCode::NoTriggerNeeded
        );
        
        // 自动还款到目标健康因子
        let repay_amount = account.target_health_factor.saturating_sub(account.health_factor);
        account.health_factor = account.target_health_factor;
        
        msg!("Auto repay executed: health factor restored to {}", account.health_factor);
        Ok(())
    }

    // 内部函数检查自动化条件
    fn check_automation(account: &mut AccountState) -> Result<()> {
        if account.automation_enabled && 
           account.health_factor <= account.trigger_health_factor &&
           account.trigger_health_factor > 0 {
            // 自动还款逻辑
            let repay_amount = account.target_health_factor.saturating_sub(account.health_factor);
            account.health_factor = account.target_health_factor;
            msg!("Automation triggered! Health factor restored to {}", account.health_factor);
        }
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct AccountState {
    pub health_factor: u64,
    pub trigger_health_factor: u64,
    pub target_health_factor: u64,
    pub automation_enabled: bool,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8 + 8 + 8 + 1 // discriminator + health_factor + trigger + target + bool
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

#[error_code]
pub enum ErrorCode {
    #[msg("Target health factor must be greater than trigger health factor")]
    InvalidHealthFactors,
    #[msg("Automation is not enabled")]
    AutomationNotEnabled,
    #[msg("Health factor is above trigger threshold")]
    NoTriggerNeeded,
}
