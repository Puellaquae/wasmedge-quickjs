#![allow(dead_code, unused_imports, unused_must_use)]

use std::borrow::{Borrow, BorrowMut};
use wasmedge_quickjs::*;

fn test_js_file(file_path: &str) {
    use wasmedge_quickjs as q;
    let mut rt = q::Runtime::new();
    rt.run_with_context(|ctx| {
        let code = std::fs::read_to_string(&file_path);
        match code {
            Ok(code) => {
                ctx.put_args(vec![file_path.clone()]);
                ctx.eval_module_str(code, &file_path);
                if let JsValue::Bool(false) = ctx.get_global().get("assertPass") {
                    assert!(false, "js assert fail");
                }
            }
            Err(e) => {
                eprintln!("{}", e.to_string());
                assert!(false, "run js test file fail");
            }
        }
        ctx.js_loop().unwrap();
    });
}

#[test]
#[ignore = "working"]
fn test_crypto_aes_wrap() {
    test_js_file("test/fs/test-crypto-aes-wrap.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_async_sign_verify() {
    test_js_file("test/fs/test-crypto-async-sign-verify.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_authenticated() {
    test_js_file("test/fs/test-crypto-authenticated.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_authenticated_stream() {
    test_js_file("test/fs/test-crypto-authenticated-stream.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_binary_default() {
    test_js_file("test/fs/test-crypto-binary-default.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_certificate() {
    test_js_file("test/fs/test-crypto-certificate.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_cipher_decipher() {
    test_js_file("test/fs/test-crypto-cipher-decipher.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_cipheriv_decipheriv() {
    test_js_file("test/fs/test-crypto-cipheriv-decipheriv.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_classes() {
    test_js_file("test/fs/test-crypto-classes.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_des3_wrap() {
    test_js_file("test/fs/test-crypto-des3-wrap.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_dh_constructor() {
    test_js_file("test/fs/test-crypto-dh-constructor.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_dh_curves() {
    test_js_file("test/fs/test-crypto-dh-curves.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_dh() {
    test_js_file("test/fs/test-crypto-dh.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_dh_leak() {
    test_js_file("test/fs/test-crypto-dh-leak.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_dh_modp2() {
    test_js_file("test/fs/test-crypto-dh-modp2.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_dh_modp2_views() {
    test_js_file("test/fs/test-crypto-dh-modp2-views.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_dh_odd_key() {
    test_js_file("test/fs/test-crypto-dh-odd-key.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_dh_padding() {
    test_js_file("test/fs/test-crypto-dh-padding.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_dh_shared() {
    test_js_file("test/fs/test-crypto-dh-shared.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_dh_stateless() {
    test_js_file("test/fs/test-crypto-dh-stateless.js");
}
#[test]
#[ignore = "unsupported, domain"]
fn test_crypto_domain() {
    test_js_file("test/fs/test-crypto-domain.js");
}
#[test]
#[ignore = "unsupported, domain"]
fn test_crypto_domains() {
    test_js_file("test/fs/test-crypto-domains.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_ecb() {
    test_js_file("test/fs/test-crypto-ecb.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_ecdh_convert_key() {
    test_js_file("test/fs/test-crypto-ecdh-convert-key.js");
}
#[test]
#[ignore = "unsupported"]
fn test_crypto_fips() {
    test_js_file("test/fs/test-crypto-fips.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_from_binary() {
    test_js_file("test/fs/test-crypto-from-binary.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_getcipherinfo() {
    test_js_file("test/fs/test-crypto-getcipherinfo.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_hash() {
    test_js_file("test/fs/test-crypto-hash.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_hash_stream_pipe() {
    test_js_file("test/fs/test-crypto-hash-stream-pipe.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_hkdf() {
    test_js_file("test/fs/test-crypto-hkdf.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_hmac() {
    test_js_file("test/fs/test-crypto-hmac.js");
}
#[test]
#[ignore = "working"]
fn test_crypto() {
    test_js_file("test/fs/test-crypto.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_keygen_deprecation() {
    test_js_file("test/fs/test-crypto-keygen-deprecation.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_keygen() {
    test_js_file("test/fs/test-crypto-keygen.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_key_objects() {
    test_js_file("test/fs/test-crypto-key-objects.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_key_objects_messageport() {
    test_js_file("test/fs/test-crypto-key-objects-messageport.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_lazy_transform_writable() {
    test_js_file("test/fs/test-crypto-lazy-transform-writable.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_modp1_error() {
    test_js_file("test/fs/test-crypto-modp1-error.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_op_during_process_exit() {
    test_js_file("test/fs/test-crypto-op-during-process-exit.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_padding_aes256() {
    test_js_file("test/fs/test-crypto-padding-aes256.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_padding() {
    test_js_file("test/fs/test-crypto-padding.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_pbkdf2() {
    test_js_file("test/fs/test-crypto-pbkdf2.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_prime() {
    test_js_file("test/fs/test-crypto-prime.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_private_decrypt_gh32240() {
    test_js_file("test/fs/test-crypto-private-decrypt-gh32240.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_psychic_signatures() {
    test_js_file("test/fs/test-crypto-psychic-signatures.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_randomfillsync_regression() {
    test_js_file("test/fs/test-crypto-randomfillsync-regression.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_random() {
    test_js_file("test/fs/test-crypto-random.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_randomuuid() {
    test_js_file("test/fs/test-crypto-randomuuid.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_rsa_dsa() {
    test_js_file("test/fs/test-crypto-rsa-dsa.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_scrypt() {
    test_js_file("test/fs/test-crypto-scrypt.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_secret_keygen() {
    test_js_file("test/fs/test-crypto-secret-keygen.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_secure_heap() {
    test_js_file("test/fs/test-crypto-secure-heap.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_sign_verify() {
    test_js_file("test/fs/test-crypto-sign-verify.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_stream() {
    test_js_file("test/fs/test-crypto-stream.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_subtle_zero_length() {
    test_js_file("test/fs/test-crypto-subtle-zero-length.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_update_encoding() {
    test_js_file("test/fs/test-crypto-update-encoding.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_verify_failure() {
    test_js_file("test/fs/test-crypto-verify-failure.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_webcrypto_aes_decrypt_tag_too_small() {
    test_js_file("test/fs/test-crypto-webcrypto-aes-decrypt-tag-too-small.js");
}
#[test]
#[ignore = "unsupport, worker thread"]
fn test_crypto_worker_thread() {
    test_js_file("test/fs/test-crypto-worker-thread.js");
}
#[test]
#[ignore = "working"]
fn test_crypto_x509() {
    test_js_file("test/fs/test-crypto-x509.js");
}