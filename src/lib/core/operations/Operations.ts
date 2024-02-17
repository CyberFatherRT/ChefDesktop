import type { BaseOperation } from "../baseOperation";
import { Hmac } from "./Hmac";
import { KuznechikDecrypt } from "./KuznechickDecrypt";
import { KuznechikEncrypt } from "./KuznechickEncrypt";

// @ts-ignore
export let operationList = {
    "HMAC": Hmac,
    "Kuznechik Encrypt": KuznechikEncrypt,
    "Kuznechik Decrypt": KuznechikDecrypt,
};

export {
    Hmac,
    KuznechikEncrypt,
    KuznechikDecrypt,
}
