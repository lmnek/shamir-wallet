use bdk::bitcoin::util::bip32::ExtendedPrivKey;
// WARNING: non-tested SSS library, still under development
use sssmc39::{combine_mnemonics, generate_mnemonics};

pub fn split(
    xprv: ExtendedPrivKey,
    treshold: u8,
    count: u8,
) -> Result<Vec<Vec<String>>, sssmc39::Error> {
    generate_mnemonics(
        1, // NOTE: number of groups
        &[(treshold, count)],
        &xprv.encode(),
        "passphrase",
        0,
    )
    .unwrap()[0] // the only group
        .mnemonic_list()
}

pub fn combine(mnemonics: Vec<Vec<String>>) -> ExtendedPrivKey {
    let bytes = combine_mnemonics(mnemonics.as_slice(), "passphrase").unwrap();
    ExtendedPrivKey::decode(bytes.as_slice()).unwrap()
}
