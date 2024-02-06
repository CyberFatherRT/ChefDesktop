#include <libakrypt-base.h>
#include <libakrypt.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
*
* kuznechik / magma - cbc, ctr, ofb, cfb
*
*/

typedef enum {
    m_cbc,
    m_ctr,
    m_ofb,
    m_cfb,
} Mode;

int akrypt_encrypt(char* algorithm, char* input, char* output, ak_uint8 *key, ak_uint8 *iv, Mode mode) {

    struct bckey ctx;

    int error = ak_error_ok;
    int exitstatus = EXIT_FAILURE;

    if (!ak_libakrypt_create(ak_function_log_syslog)) {
        return ak_libakrypt_destroy();
    }

    if (strcmp(algorithm, "kuznechik") == 0) {
        ak_bckey_create_kuznechik(&ctx);
    } else if (strcmp(algorithm, "magma") == 0) {
        ak_bckey_create_magma(&ctx);
    } else {
        char* err_msg = (char *)malloc((20 + strlen(algorithm)) * sizeof(char));
        sprintf(err_msg, "Unknown algorithm: %s", algorithm);
        ak_log_set_message(err_msg);
        free(err_msg);
        return -1;
    }

    ak_bckey_set_key(&ctx, key, 32);

    switch (mode) {

        case m_cbc:
            error = ak_bckey_encrypt_cbc(&ctx, input, output, strlen(input), iv, sizeof(iv));
            if (error != ak_error_ok) goto exlab;
            break;
        case m_ctr:
            error = ak_bckey_ctr(&ctx, input, output, strlen(input), iv, sizeof(iv));
            if (error != ak_error_ok) goto exlab;
            break;
        case m_ofb:
            error = ak_bckey_ofb(&ctx, input, output, 13, iv, sizeof(iv));
            if (error != ak_error_ok) goto exlab;
            break;
        case m_cfb:
            error = ak_bckey_encrypt_cfb(&ctx, input, output, strlen(input), iv, sizeof(iv));
            if (error != ak_error_ok) goto exlab;
            break;
    }

    sprintf(output, "%s", ak_ptr_to_hexstr(output, 13, ak_false));

exlab: ak_bckey_destroy(&ctx);

    if (error == ak_error_ok) exitstatus = EXIT_SUCCESS;

    ak_libakrypt_destroy();

    return exitstatus;
}

void foo() {
    printf("Hello from akrypt");
}
