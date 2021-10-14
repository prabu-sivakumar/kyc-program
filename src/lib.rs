use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize,BorshSerialize};

#[derive(Debug, Deserialize, Serialize, BorshDeserialize, BorshSerialize,Clone)]
pub struct CustomerData {
    pub instruction: String,
    pub legal_name: String,
    pub registration_number: String,
    pub incorporation_country: String,
    pub lei_registration_status: String,
    pub lei: String,
    pub incorporation_date: String,
    pub primary_country_operation: String,
    pub primary_isic_code: String,
    pub entity_type: String,
    pub swift_code: String,
    pub kyc_status: bool,
    pub addresses: Vec<AddressData>,
    pub kyc_documents: Vec<KycDocument>
}

#[derive(Debug, Deserialize, Serialize, BorshDeserialize, BorshSerialize,Clone)]
pub struct AddressData {
    pub address_type: String,
    pub address_line1: String,
    pub address_line2: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String
}

#[derive(Debug, Deserialize, Serialize, BorshDeserialize, BorshSerialize,Clone)]
pub struct KycDocument {
    pub document_id: String,
    pub document_type: String,
    pub document_name: String,
    pub document_transaction_id: String
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing Customer Data");
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("Invalid Program Id");
        return Err(ProgramError::IncorrectProgramId);
    }
    
    let data = String::from_utf8(instruction_data.to_vec()).map_err(|_err| {
                    msg!("Invalid UTF-8, from byte {}");
                                ProgramError::InvalidInstructionData
    })?;

    let payload = String::from(data.chars().as_str());
    msg!("Request Payload is {}",payload);
    let customer: CustomerData = serde_json::from_str(&payload).unwrap();
    
    match customer.instruction.as_ref() {
        "POST" => {
            msg!("POST Operation");  
             Ok(())
           },          
          "GET" => {
            msg!("GET Operation");
             Ok(())
           },
           "PUT" => {
            msg!("PUT Operation");
             Ok(())
           } 
           _ => Ok(())
     }
    
}