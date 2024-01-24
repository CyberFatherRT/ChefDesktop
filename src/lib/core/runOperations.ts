import { invoke } from '@tauri-apps/api'
import ToBase64 from './operations/ToBase64'

let temp = new ToBase64();

temp.run({input: "hi mom", params: {}})

let request = [
    {
        name: "To Base64",
        request: JSON.stringify({input: input, params: {}}),
        is_disable: false,
        breakpoint: false,
    },
    {
        name: "To Hex",
        request: JSON.stringify({input: input, params: {}}),
        is_disable: false,
        breakpoint: false
    },
];

for (let item of request) {
    if (item.is_disable) {
        continue
    } else if (item.breakpoint) {
        break
    }
    input = await invoke(item.name, {request: item.request})
}
