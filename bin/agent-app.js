#!/usr/bin/env node
import { execSync } from "child_process";
import { existsSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const binary = join(__dirname, "..", "native", "target", "release", "agent-app.exe");

if (!existsSync(binary)) {
  console.error("agent-app binary not found. Run `npm run build:native` first.");
  process.exit(1);
}

const args = process.argv.slice(2);
try {
  execSync(`"${binary}" ${args.join(" ")}`, { stdio: "inherit" });
} catch {
  process.exit(1);
}
