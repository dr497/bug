use solana_program_test::ProgramTest;
use solana_sdk::signer::{keypair::Keypair, Signer};
use spl_associated_token_account::instruction::create_associated_token_account;

pub mod utils;

#[tokio::main]
async fn main() {
    use utils::{mint_bootstrap, sign_send_instructions};
    let mut program_test = ProgramTest::default();

    let buyer = Keypair::new();
    let mint_authority = Keypair::new();

    let (mint, _) = mint_bootstrap(None, 6, &mut program_test, &mint_authority.pubkey());
    let mut prg_test_ctx = program_test.start_with_context().await;
    let ix = create_associated_token_account(
        &prg_test_ctx.payer.pubkey(),
        &buyer.pubkey(),
        &mint,
        &spl_token::ID,
    );
    sign_send_instructions(&mut prg_test_ctx, vec![ix], vec![])
        .await
        .unwrap();
}
