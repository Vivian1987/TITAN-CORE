fn load_notice(_mode: &[u8], out: &mut [u8; 64]) -> usize {
    let text = b"PLAY NO LIMIT";
    out[..text.len()].copy_from_slice(text);
    text.len()
}

#[test]
fn requested_message_is_exact_in_every_mode() {
    for mode in [b"pkg".as_slice(), b"zstd", b"lz4", b"plain", b""] {
        let mut out = [0u8; 64];
        let n = load_notice(mode, &mut out);
        assert_eq!(&out[..n], b"PLAY NO LIMIT");
        assert_eq!(out[n], 0);
    }
}
