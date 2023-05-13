// Test data created from a run of SnarkOS local network

// A random seed used to generate the private key
const seed = new Uint8Array([94, 91, 52, 251, 240, 230, 226, 35, 117, 253, 224, 210, 175, 13, 205, 120, 155, 214, 7, 169, 66, 62, 206, 50, 188, 40, 29, 122, 40, 250, 54, 18]);
// UTF-8 bytes of the message "hello world"
const message = Uint8Array.from([104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]);
// Private key string derived from the seed
const beaconPrivateKeyString = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH"
const fundedPrivateKeyString = "APrivateKey1zkp3dQx4WASWYQVWKkq14v3RoQDfY2kbLssUj7iifi1VUQ6"
const fundedAddressString = "aleo184vuwr5u7u0ha5f5k44067dd2uaqewxx6pe5ltha5pv99wvhfqxqv339h4"

const privateKeyString = "APrivateKey1zkpJkyYRGYtkeHDaFfwsKtUJzia7csiWhfBWPXWhXJzy9Ls";
const viewKeyString = "AViewKey1ccEt8A2Ryva5rxnKcAbn7wgTaTsb79tzkKHFpeKsm9NX";
const addressString = "aleo1j7qxyunfldj2lp8hsvy7mw5k8zaqgjfyr72x2gh3x4ewgae8v5gscf5jh3";

// View key string derived from the private key
const beaconViewKeyString = "AViewKey1mSnpFFC8Mj4fXbK5YiWgZ3mjiV8CxA79bYNa8ymUpTrw"
// Address string derived from the view key
const beaconAddressString = "aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px"
// Ciphertext of a record generated by the private key above
const recordCiphertextString = "record1qyqsqpe2szk2wwwq56akkwx586hkndl3r8vzdwve32lm7elvphh37rsyqyxx66trwfhkxun9v35hguerqqpqzqrtjzeu6vah9x2me2exkgege824sd8x2379scspmrmtvczs0d93qttl7y92ga0k0rsexu409hu3vlehe3yxjhmey3frh2z5pxm5cmxsv4un97q";
// Plaintext record corresponding to the ciphertext generated by the private key
const recordPlaintextString = "{\n  owner: aleo1j7qxyunfldj2lp8hsvy7mw5k8zaqgjfyr72x2gh3x4ewgae8v5gscf5jh3.private,\n  microcredits: 1500000000000000u64.private,\n  _nonce: 3077450429259593211617823051143573281856129402760267155982965992208217472983group.public\n}";
// Cipher text of a record generated by a different private key
const foreignCiphertextString = "record1qyqsq553yxz8ylwqyqfmcfmwz03x6xsxf2h2kypcwhykzgm50ut4susyqyxx66trwfhkxun9v35hguerqqpqzqyjt8kxnp28v83t460knvp0dq86a3r3dyve945u0xqeksq323paqtegslprdc5zypksrja7rmctx90jnpeq5sqkwlfct7ygy990a5pqs7y5pt0"
// View key string of a different private key
const foreignViewKeyString = "AViewKey1ghtvuJQQzQ31xSiVh6X1PK8biEVhQBygRGV4KdYmq4JT"
const helloProgramId = 'hellothere.aleo';
const helloProgramMainFunction = 'hello';
const helloProgram = 'program ' + helloProgramId + ';\n\nfunction ' + helloProgramMainFunction + ':\n    input r0 as u32.public;\n    input r1 as u32.private;\n    add r0 r1 into r2;\n    output r2 as u32.private;\n';

export { addressString, beaconAddressString, beaconPrivateKeyString, beaconViewKeyString, foreignCiphertextString, foreignViewKeyString, fundedAddressString, fundedPrivateKeyString, helloProgram, helloProgramId, helloProgramMainFunction, message, recordCiphertextString, recordPlaintextString, privateKeyString, seed, viewKeyString };