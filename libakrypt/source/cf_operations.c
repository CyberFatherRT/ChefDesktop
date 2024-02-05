#include <libakrypt-base.h>
#include <libakrypt.h>

/*
*
* kuznechik / magma - cbc, ctr, ofb, cfb, acpkm, xts
*
*/

int foo() {
    struct bckey ctx;

    if (!ak_libakrypt_create(ak_function_log_syslog)) {
        return ak_libakrypt_destroy();
    }

    ak_libakrypt_set_openssl_compability(ak_false);

    ak_bckey_create_kuznechik(&ctx);

    ak_bckey_set_key(&ctx, "super_secret_key", 16);

    char data[] = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

    ak_bckey_encrypt_ecb(&ctx, data, data, sizeof(data));

    printf("Encrypted: %s\n", ak_ptr_to_hexstr(data, sizeof(data), ak_false));

    ak_bckey_decrypt_ecb(&ctx, data, data, sizeof(data));

    printf("Decrypted: %s\n", data);

    return 0;
}
