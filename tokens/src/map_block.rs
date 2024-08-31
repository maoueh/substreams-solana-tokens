use crate::pb::sf::solana::tokens::v1::{event::Type, Event, Events};
use substreams::skip_empty_output;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Events, substreams::errors::Error> {
    skip_empty_output();

    let block_height = block
        .block_height
        .as_ref()
        .map(|h| h.block_height)
        .unwrap_or_default();
    let block_timestamp = block
        .block_time
        .as_ref()
        .map(|t| t.timestamp)
        .unwrap_or_default();

    let mut data: Vec<Event> = Vec::new();
    for confirmed_txn in block.transactions() {
        let tx_id = confirmed_txn.id();
        for (i, instruction) in confirmed_txn.walk_instructions().enumerate() {
            if instruction.program_id() != spl_token::ID {
                continue;
            }

            let token_instruction =
                spl_token::instruction::TokenInstruction::unpack(instruction.data())?;

            match Type::try_from((token_instruction, &instruction)) {
                Ok(event_type) => {
                    data.push(Event {
                        slot: block.slot,
                        txn_id: tx_id.clone(),
                        block_height,
                        block_timestamp,
                        block_hash: block.blockhash.clone(),
                        instruction_index: i as u32,
                        r#type: Some(event_type),
                    });
                }
                Err(_) => continue,
            }
        }
    }

    Ok(Events { data })
}
