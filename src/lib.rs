#[path = "encode.rs"]
mod encode;

#[test]
fn encoding() {
    let text = "iamhurtverybadlyhelp";
    let key = 4;
    let ciphered = encode::encode(text, key);
    assert_eq!("iryyatbhmvaehedlurlp", ciphered);
}
