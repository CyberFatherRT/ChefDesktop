import { Hmac } from "./Hmac";
import { KuznechikDecrypt } from "./KuznechickDecrypt";
import { KuznechikEncrypt } from "./KuznechickEncrypt";
import { ToBase64 } from "./ToBase64";

// @ts-ignore
export let operationList = {
    "To Base64": ToBase64,
    "HMAC": Hmac,
    "Kuznechik Encrypt": KuznechikEncrypt,
    "Kuznechik Decrypt": KuznechikDecrypt,
};

export {
    Hmac,
    KuznechikEncrypt,
    KuznechikDecrypt,
}
