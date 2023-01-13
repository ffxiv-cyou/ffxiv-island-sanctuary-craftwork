import { gzipSync, gunzipSync } from "fflate";
import { fromUint8Array, toUint8Array } from "js-base64";

export function ToShareCode(raw: Uint8Array): string {
    return fromUint8Array(raw, true);
}

export function FromShareCode(input: string): Uint8Array {
    return toUint8Array(input);
}

export function Compress(raw: Uint8Array): string {
    if (raw.length == 0)
        return "";

    const data = gzipSync(raw);
    return fromUint8Array(removeGzipHeader(data), true);
}

export function Decompress(input: string): Uint8Array {
    if (input == "")
        return new Uint8Array(0);

    const data = toUint8Array(input);
    return gunzipSync(addGzipHeader(data));
}

function removeGzipHeader(data: Uint8Array): Uint8Array {
    const dstLen = data.length - 10;
    const bytes = new Uint8Array(dstLen);
    bytes[0] = 0x31;
    bytes[1] = 0x34;
    bytes[2] = 0x01;
    bytes[3] = data[data.length - 4];
    bytes.set(data.slice(10, data.length - 4), 4);
    return bytes;
}

function addGzipHeader(data: Uint8Array): Uint8Array {
    const bytes = new Uint8Array(data.length + 10);
    let i = 0;
    bytes[i++] = 0x1F; // header, 2 bytes
    bytes[i++] = 0x8B;
    bytes[i++] = 0x08; // CM, 8 = Deflate
    bytes[i++] = 0x00; // FLAG, disable all feature

    bytes[i++] = 0; // MTIME, 0
    bytes[i++] = 0;
    bytes[i++] = 0;
    bytes[i++] = 0;

    bytes[i++] = 2; // XFL, 2 = maximum compression
    bytes[i++] = 0; // OS, 3 = Unix system.

    bytes.set(data.slice(4), 10);
    i += data.length - 4;

    bytes[i++] = data[3]; // Length
    bytes[i++] = 0;
    bytes[i++] = 0;
    bytes[i++] = 0;
    return bytes;
}