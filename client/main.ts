import {
    establishConnection,
    establishPayer,
    checkProgram,
    createCustomer,
    updateKycStatus
} from './kyc_client';

async function main() {
    await establishConnection();

    await establishPayer();

    await checkProgram();

    console.log("Creating Customer");
    await createCustomer('"instruction" : "CreateCustomer", "customer_id" : "1", "legal_name" : "ABC Corporation", "registration_number" : "1002343-AXZSDF", "incorporation_country" : "Singapore", "lei_registration_status" :"Registered", "lei" : "ABC4097092374092BDJ3", "incorporation_date" : "12 May 2012", "primary_country_operation" : "Singapore", "primary_isic_code" : "1122 - Food and Beverages", "entity_type" : "Subsidiary", "swift_code" : "ABCFXX", "kyc_status" : false, "is_active" : true');

    console.log("Updating KycStatus for the Customer")
    await updateKycStatus('"instruction" : "updateKycStatus", "customer_id" : "1", "legal_name" : "ABC Corporation", "registration_number" : "1002343-AXZSDF", "incorporation_country" : "Singapore", "lei_registration_status" :"Registered", "lei" : "ABC4097092374092BDJ3", "incorporation_date" : "12 May 2012", "primary_country_operation" : "Singapore", "primary_isic_code" : "1122 - Food and Beverages", "entity_type" : "Subsidiary", "swift_code" : "ABCFXX", "kyc_status" : true, "is_active" : true');

    console.log('Success');
}

main().then(
    () => process.exit(),
    err => {
        console.error(err);
        process.exit(-1);
    },
);