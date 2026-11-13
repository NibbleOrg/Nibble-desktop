import { readFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
const path = "src/ipc/bindings.ts";
const committed = readFileSync(path, "utf8");
execFileSync(
  "cargo",
  [
    "run",
    "--manifest-path",
    "src-tauri/Cargo.toml",
    "--locked",
    "--bin",
    "export-bindings",
  ],
  { stdio: "inherit" },
);
if (readFileSync(path, "utf8") !== committed) {
  throw new Error("IPC bindings drifted; regenerate and review the diff.");
}
