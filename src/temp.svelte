<script lang="ts" context="module">
    import { invoke } from "@tauri-apps/api";
    import { bar } from "./temp";

    bar();
    
    export async function foo() {
        let input: string = "input message";

        let request = [
            {
                name: "hmac",
                request: JSON.stringify(
                {
                    input: input,
                    params: {
                        key: "my secret and secure key",
                        key_format: "utf8",
                        hash_function: "sha256",
                        output_format: "hex"
                    }
                }),
                is_disable: false,
                breakpoint: false,
            },
        ];

        for (let item of request) {
            if (item.is_disable) {
                continue
            } else if (item.breakpoint) {
                break
            }
            input = await invoke(item.name, {request: item.request}).catch((err) => err) as string
        }

        console.log(input)
        return input
    }

</script>

