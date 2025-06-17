use anchor_lang::prelude::*;

declare_id!();

#[program]
pub mod todo_dapp {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializerUser>) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.last_todo = 0;
        user_profile.todo_count = 0;
        Ok(())
    }

    pub fn add_todo(ctx: Context<AddTodo>, content: String) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;
        let user_profile = &mut ctx.accounts.user_profile;

        todo_account.authority = ctx.accounts.authority.key();
        todo_account.idx = user_profile.last_todo;
        todo_account.content = content;
        todo_account.marked = false;

        user_profile.last_todo = user_profile.last_todo.checked_add(1).unwrap();
        user_profile.todo_count = user_profile.todo_count.checked_add(1).unwrap();

        Ok(())
    }

    pub fn mark_todo(ctx: Context<MarkTodo>, todo_idx: u8) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;
        require!(!todo_account.marked, TodoError::AlreadyMarked);
        todo_account.marked = true;
        Ok(())
    }

    pub fn unmark_todo(ctx: Context<UnmarkTodo>, todo_idx: u8) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;
        require!(!todo_account.marked, TodoError::NotMarked);
        todo_account.marked = false;
        Ok(())
    }

    pub fn remove_todo(ctx: Context<RemoveTodo>, todo_idx: u8) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.todo_count = user_profile.todo_count.checked_sub(1).unwrap();
        Ok(())
    }

    pub fn update_todo(ctx: Context<UpdateTodo>, todo_idx: u8, content: String) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;
        todo_account.content = content;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
        seeds = [USER_TAG, authority.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct AddTodo<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Account<'info, UserProfile>,
}
