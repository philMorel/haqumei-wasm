import { createHash } from "node:crypto";
import { createReadStream, readFileSync, statSync } from "node:fs";
import { createGunzip } from "node:zlib";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const manifest = JSON.parse(readFileSync(path.join(root, "runtime/manifest.json"), "utf8"));
const runtimeDir = path.join(root, "runtime", `v${manifest.portVersion}`);

function sha256File(filename) {
  const bytes = readFileSync(filename);
  return createHash("sha256").update(bytes).digest("hex");
}

function verifyFile(filename, size, sha256) {
  const full = path.join(runtimeDir, filename);
  const actualSize = statSync(full).size;
  if (actualSize !== size) throw new Error(`${filename}: size ${actualSize} != ${size}`);
  const actualHash = sha256File(full);
  if (actualHash !== sha256) throw new Error(`${filename}: SHA-256 mismatch`);
}

verifyFile(manifest.wasm.filename, manifest.wasm.sizeBytes, manifest.wasm.sha256);
verifyFile(
  manifest.dictionary.filename,
  manifest.dictionary.gzipSizeBytes,
  manifest.dictionary.gzipSha256,
);

const wasm = readFileSync(path.join(runtimeDir, manifest.wasm.filename));
if (!WebAssembly.validate(wasm)) throw new Error("committed WASM is invalid");

const sums = readFileSync(path.join(runtimeDir, "SHA256SUMS"), "utf8").trim().split("\n");
for (const line of sums) {
  const match = /^([a-f0-9]{64})  (.+)$/.exec(line);
  if (!match) throw new Error(`invalid SHA256SUMS line: ${line}`);
  if (sha256File(path.join(runtimeDir, match[2])) !== match[1]) {
    throw new Error(`${match[2]}: SHA256SUMS mismatch`);
  }
}

const parts = manifest.dictionary.parts.map((part) => ({
  ...part,
  received: 0,
  hash: createHash("sha256"),
}));
let partIndex = 0;
let rawBytes = 0;
const gunzip = createReadStream(path.join(runtimeDir, manifest.dictionary.filename)).pipe(createGunzip());
for await (const chunk of gunzip) {
  let offset = 0;
  rawBytes += chunk.length;
  while (offset < chunk.length) {
    const part = parts[partIndex];
    if (!part) throw new Error("dictionary has trailing decompressed bytes");
    const take = Math.min(chunk.length - offset, part.sizeBytes - part.received);
    part.hash.update(chunk.subarray(offset, offset + take));
    part.received += take;
    offset += take;
    if (part.received === part.sizeBytes) partIndex += 1;
  }
}

if (rawBytes !== manifest.dictionary.rawSizeBytes) {
  throw new Error(`dictionary raw size ${rawBytes} != ${manifest.dictionary.rawSizeBytes}`);
}
for (const part of parts) {
  if (part.received !== part.sizeBytes) throw new Error(`${part.name}: truncated`);
  if (part.hash.digest("hex") !== part.sha256) throw new Error(`${part.name}: SHA-256 mismatch`);
}
console.log(`Verified ${manifest.portVersion} runtime assets.`);
