use crate::pb::sf::solana::tokens::v1::{event::Type, Events};
use substreams::skip_empty_output;
use substreams_database_change::{pb::database::DatabaseChanges, tables::Tables};

#[substreams::handlers::map]
fn db_out(events: Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    skip_empty_output();

    let mut tables = Tables::new();

    for event in events.data.iter() {
        let row = tables
            .create_row(
                "transfer",
                [
                    ("evt_tx", event.txn_id.clone()),
                    ("evt_instruction_index", event.instruction_index.to_string()),
                ],
            )
            .set("evt_slot", event.slot)
            .set("evt_block_timestamp", event.block_timestamp)
            .set("evt_block_height", event.block_height)
            .set("evt_block_hash", &event.block_hash);

        match &event.r#type {
            Some(Type::Transfer(transfer)) => {
                let accounts = transfer.accounts.as_ref().unwrap();

                row.set("amount", transfer.instruction.as_ref().unwrap().amount)
                    .set("source", &accounts.source)
                    .set("destination", &accounts.destination)
                    .set("signer", &accounts.signer);
            }
            _ => continue,
        }
    }

    Ok(tables.to_database_changes())
}
