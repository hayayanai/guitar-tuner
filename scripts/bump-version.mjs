#!/usr/bin/env node

import { readFile, writeFile } from "fs/promises";
import { resolve } from "path";
import { execSync } from "child_process";

const VERSION_REGEX = /^\d+\.\d+\.\d+$/;

async function updateJsonFile(filePath, versionPath) {
  const content = await readFile(filePath, "utf-8");
  const json = JSON.parse(content);

  const keys = versionPath.split(".");
  let obj = json;
  for (let i = 0; i < keys.length - 1; i++) {
    obj = obj[keys[i]];
  }
  const oldVersion = obj[keys[keys.length - 1]];
  obj[keys[keys.length - 1]] = newVersion;

  await writeFile(filePath, JSON.stringify(json, null, 2) + "\n", "utf-8");
  console.log(`✓ Updated ${filePath}: ${oldVersion} → ${newVersion}`);
}

async function updateTomlFile(filePath) {
  const content = await readFile(filePath, "utf-8");
  const lines = content.split("\n");

  let oldVersion = null;
  const newLines = lines.map((line) => {
    const match = line.match(/^version\s*=\s*"(.+)"$/);
    if (match) {
      oldVersion = match[1];
      return `version = "${newVersion}"`;
    }
    return line;
  });

  await writeFile(filePath, newLines.join("\n"), "utf-8");
  console.log(`✓ Updated ${filePath}: ${oldVersion} → ${newVersion}`);
}

// Parse arguments
const args = process.argv.slice(2);
if (args.length !== 1 || !VERSION_REGEX.test(args[0])) {
  console.error("Usage: npm run bump <version>");
  console.error("Example: npm run bump 0.3.0");
  process.exit(1);
}

const newVersion = args[0];

try {
  // Update package.json
  await updateJsonFile(resolve(process.cwd(), "package.json"), "version");

  // Update package-lock.json
  console.log("\n⏳ Updating package-lock.json...");
  execSync("npm install --package-lock-only", {
    cwd: process.cwd(),
    stdio: "inherit",
  });
  console.log("✓ package-lock.json updated");

  // Update tauri.conf.json
  await updateJsonFile(resolve(process.cwd(), "src-tauri/tauri.conf.json"), "version");

  // Update Cargo.toml
  await updateTomlFile(resolve(process.cwd(), "src-tauri/Cargo.toml"));

  // Update Cargo.lock
  console.log("\n⏳ Updating Cargo.lock...");
  execSync("cargo update -p app", {
    cwd: resolve(process.cwd(), "src-tauri"),
    stdio: "inherit",
  });
  console.log("✓ Cargo.lock updated");

  console.log("\n✨ Version bump complete!");
  console.log("\nNext steps:");
  console.log("  git add -A");
  console.log(`  git commit -m "chore: bump version to ${newVersion}"`);
  console.log(`  git tag v${newVersion}`);
  console.log("  git push && git push --tags");
} catch (error) {
  console.error("❌ Error:", error.message);
  process.exit(1);
}
