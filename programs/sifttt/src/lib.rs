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
        account.health_factor = account.target_health_factor;
        
        msg!("Auto repay executed: health factor restored to {}", account.health_factor);
        Ok(())
    }

    // 设置定投参数
    pub fn set_dca(
        ctx: Context<Operate>,
        interval: u64,
        token_address: Pubkey,
        token_amount: u64,
    ) -> Result<()> {
        let account = &mut ctx.accounts.account;
        
        // 验证参数
        require!(interval > 0, ErrorCode::InvalidInterval);
        require!(token_amount > 0, ErrorCode::InvalidAmount);
        
        // 更新账户状态
        account.dca_interval = interval;
        account.token_address = token_address;
        account.token_amount = token_amount;
        account.dca_enabled = true;
        
        msg!(
            "DCA set: interval={}, token={}, amount={}",
            interval,
            token_address,
            token_amount
        );
        Ok(())
    }

    // Mock买入函数
    pub fn mock_buy(
        ctx: Context<Operate>,
        token_address: Pubkey,
        token_amount: u64,
    ) -> Result<()> {
        let account = &mut ctx.accounts.account;
        
        // 验证定投是否启用
        require!(account.dca_enabled, ErrorCode::DCANotEnabled);
        // 验证token地址是否匹配
        require!(account.token_address == token_address, ErrorCode::TokenMismatch);
        
        msg!(
            "Mock buying {} tokens from contract {}",
            token_amount,
            token_address
        );
        
        Ok(())
    }

    // 设置价格交易参数
    pub fn set_price_trading(
        ctx: Context<Operate>,
        target_price: u64,
        token_address: Pubkey,
        token_amount: u64,
    ) -> Result<()> {
        let account = &mut ctx.accounts.account;
        
        // 验证参数
        require!(target_price > 0, ErrorCode::InvalidPrice);
        require!(token_amount > 0, ErrorCode::InvalidAmount);
        
        // 更新账户状态
        account.target_price = target_price;
        account.price_token_address = token_address;
        account.price_token_amount = token_amount;
        account.price_trading_enabled = true;
        
        msg!(
            "Price trading set: target_price={}, token={}, amount={}",
            target_price,
            token_address,
            token_amount
        );
        Ok(())
    }

    // 执行价格触发交易
    pub fn execute_price_trade(
        ctx: Context<Operate>,
        current_price: u64,
    ) -> Result<()> {
        // 将account的借用限制在这个作用域内
        {
            let account = &ctx.accounts.account;
            
            // 验证价格交易是否启用
            require!(account.price_trading_enabled, ErrorCode::PriceTradingNotEnabled);
            
            // 验证当前价格是否达到目标价格
            require!(
                current_price <= account.target_price,
                ErrorCode::PriceNotMet
            );
            
            msg!(
                "Price condition met: current_price={}, target_price={}",
                current_price,
                account.target_price
            );
        } // account的借用在这里结束
        
        // 获取需要的值
        let token_address = ctx.accounts.account.price_token_address;
        let token_amount = ctx.accounts.account.price_token_amount;
        
        // 现在可以安全地移动ctx
        mock_buy(
            ctx,
            token_address,
            token_amount,
        )?;
        
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
    // 新增定投相关字段
    pub dca_interval: u64,        // 定投周期(秒)
    pub token_address: Pubkey,    // token合约地址
    pub token_amount: u64,        // 定投数量
    pub dca_enabled: bool,        // 定投是否启用
    // 价格交易相关字段
    pub target_price: u64,           // 目标价格
    pub price_token_address: Pubkey, // 目标token地址
    pub price_token_amount: u64,     // 交易数量
    pub price_trading_enabled: bool, // 价格交易是否启用
}

impl AccountState {
    pub fn check_automation(&mut self) -> Result<()> {
        if self.automation_enabled && 
           self.health_factor <= self.trigger_health_factor &&
           self.trigger_health_factor > 0 {
            self.health_factor = self.target_health_factor;
            msg!("Automation triggered! Health factor restored to {}", self.health_factor);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8 + 8 + 8 + 1 + 8 + 32 + 8 + 1 + 8 + 32 + 8 + 1 // discriminator + health_factor + trigger + target + bool + dca_interval + token_address + token_amount + dca_enabled + target_price + price_token_address + price_token_amount + price_trading_enabled
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
    #[msg("Invalid DCA interval")]
    InvalidInterval,
    #[msg("Invalid token amount")]
    InvalidAmount,
    #[msg("DCA is not enabled")]
    DCANotEnabled,
    #[msg("Token address mismatch")]
    TokenMismatch,
    #[msg("Invalid target price")]
    InvalidPrice,
    #[msg("Price trading is not enabled")]
    PriceTradingNotEnabled,
    #[msg("Current price is higher than target price")]
    PriceNotMet,
}
