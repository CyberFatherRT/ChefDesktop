#include <libakrypt-base.h>
#include <libakrypt.h>
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

typedef struct {
    char* algorithm;
    Mode mode;
    ak_uint8* key;
    ak_uint8* iv;
} Config;


int akrypt_encrypt(const Config* config, char* input, char* output) {

    struct bckey ctx;
    ak_uint8* key = config->key;
    ak_uint8* iv = config->iv;
    char* algorithm = config->algorithm;
    Mode mode = config->mode;

    int error = ak_error_ok;
    int exitstatus = EXIT_FAILURE;

    if (!ak_libakrypt_create(ak_function_log_syslog)) {
        return ak_libakrypt_destroy();
    }

    ak_libakrypt_set_openssl_compability(ak_false);

    if (strcmp(algorithm, "kuznechik")) {
        ak_bckey_create_kuznechik(&ctx);
    } else if (strcmp(algorithm, "magma")) {
        ak_bckey_create_magma(&ctx);
    } else {
        char* err_msg = (char *)malloc((20 + strlen(algorithm)) * sizeof(char));
        sprintf(err_msg, "Unknown algorithm: %s", algorithm);
        ak_log_set_message(err_msg);
        free(err_msg);
        return -1;
    }

    ak_bckey_set_key(&ctx, key, sizeof(key));

    switch (mode) {

        case m_cbc:
            error = ak_bckey_encrypt_cbc(&ctx, input, output, sizeof(input), iv, sizeof(iv));
            if (error != ak_error_ok) goto exlab;
            break;
        case m_ctr:
            error = ak_bckey_ctr(&ctx, input, output, sizeof(input), iv, sizeof(iv));
            if (error != ak_error_ok) goto exlab;
            break;
        case m_ofb:
            error = ak_bckey_ofb(&ctx, input, output, sizeof(input), iv, sizeof(iv));
            if (error != ak_error_ok) goto exlab;
            break;
        case m_cfb:
            error = ak_bckey_encrypt_cfb(&ctx, input, output, sizeof(input), iv, sizeof(iv));
            if (error != ak_error_ok) goto exlab;
            break;
    }

exlab: ak_bckey_destroy(&ctx);

    if (error == ak_error_ok) exitstatus = EXIT_SUCCESS;

    ak_libakrypt_destroy();

    return exitstatus;
}

void foo() {
    printf("Hello from akrypt");
}
