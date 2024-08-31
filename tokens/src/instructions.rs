use anyhow::anyhow;
use spl_token::instruction::TokenInstruction;
use substreams_solana::block_view::InstructionView;

use crate::pb::sf::solana::tokens::v1::{
    event::Type,
    transfer::{TransferAccounts, TransferInstruction},
    Transfer,
};

impl TryFrom<(TokenInstruction<'_>, &InstructionView<'_>)> for Type {
    type Error = substreams::errors::Error;

    fn try_from(value: (TokenInstruction<'_>, &InstructionView<'_>)) -> Result<Self, Self::Error> {
        let (value, instruction_view) = value;
        let accounts = instruction_view.accounts();

        Ok(match value {
            TokenInstruction::Transfer { amount } => Type::Transfer(Transfer {
                instruction: Some(TransferInstruction { amount }),
                accounts: Some(TransferAccounts {
                    source: accounts.get(0).unwrap().to_string(),
                    destination: accounts.get(1).unwrap().to_string(),
                    signer: accounts.get(2).unwrap().to_string(),
                }),
            }),
            // TokenInstruction::InitializeMint {
            //     mint_authority,
            //     freeze_authority,
            //     decimals,
            // } => Type::InitializeMint {
            //     mint_authority,
            //     freeze_authority,
            //     decimals,
            // },
            // TokenInstruction::InitializeAccount => Type::InitializeAccount,
            // TokenInstruction::InitializeMultisig { m } => Type::InitializeMultisig { m },
            // TokenInstruction::Approve { amount } => Type::Approve { amount },
            // TokenInstruction::Revoke => Type::Revoke,
            // TokenInstruction::SetAuthority {
            //     authority_type,
            //     new_authority,
            // } => Type::SetAuthority {
            //     authority_type,
            //     new_authority,
            // },
            // TokenInstruction::MintTo { amount } => Type::MintTo { amount },
            // TokenInstruction::Burn { amount } => Type::Burn { amount },
            // TokenInstruction::CloseAccount => Type::CloseAccount,
            // TokenInstruction::FreezeAccount => Type::FreezeAccount,
            // TokenInstruction::ThawAccount => Type::ThawAccount,
            // TokenInstruction::TransferChecked { amount, decimals } => {
            //     Type::TransferChecked { amount, decimals }
            // }
            // TokenInstruction::ApproveChecked { amount, decimals } => {
            //     Type::ApproveChecked { amount, decimals }
            // }
            // TokenInstruction::MintToChecked { amount, decimals } => {
            //     Type::MintToChecked { amount, decimals }
            // }
            // TokenInstruction::BurnChecked { amount, decimals } => {
            //     Type::BurnChecked { amount, decimals }
            // }
            // TokenInstruction::InitializeAccount2 => Type::InitializeAccount2,
            // TokenInstruction::SyncNative => Type::SyncNative,
            // TokenInstruction::InitializeAccount3 => Type::InitializeAccount3,
            // TokenInstruction::InitializeAccount4 => Type::InitializeAccount4,
            // TokenInstruction::InitializeAccount5 => Type::InitializeAccount5,
            // TokenInstruction::InitializeAccount6 => Type::InitializeAccount6,
            // TokenInstruction::InitializeAccount7 => Type::InitializeAccount7,
            // TokenInstruction::InitializeAccount8 => Type::InitializeAccount8,
            // TokenInstruction::InitializeAccount9 => Type::InitializeAccount9,
            // TokenInstruction::InitializeAccountA => Type::InitializeAccountA,
            // TokenInstruction::InitializeAccountB => Type::InitializeAccountB,
            // TokenInstruction::InitializeAccount => Type::InitializeAccount,
            _ => return Err(anyhow!("Invalid token instruction")),
        })
    }
}
