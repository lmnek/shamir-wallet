use bdk::bitcoin::util::bip32::ExtendedPrivKey;
// WARNING: non-tested SSS library, still under development
use anyhow::Result;
use sssmc39::{combine_mnemonics, generate_mnemonics};

pub fn split(xprv: ExtendedPrivKey, treshold: u8, count: u8) -> Result<Vec<String>> {
    let shares = generate_mnemonics(
        1, // NOTE: number of groups
        &[(treshold, count)],
        &xprv.encode(),
        "passphrase",
        0,
    )
    .map_err(convert)?[0] // the only group
        .mnemonic_list_flat()
        .map_err(convert)?;
    Ok(shares)
}

pub fn combine(mnemonics: Vec<Vec<String>>) -> Result<ExtendedPrivKey> {
    let bytes = combine_mnemonics(mnemonics.as_slice(), "passphrase").map_err(convert)?;
    let xprv = ExtendedPrivKey::decode(bytes.as_slice())?;
    Ok(xprv)
}

// error handling -> ssscm39::Error does not implement std::Error_error
fn convert(sssmc_error: sssmc39::Error) -> anyhow::Error {
    anyhow::anyhow!("Error in ssscm39 library occurred: {}", sssmc_error)
}
