#!/usr/bin/env node

/**
 * Extract changelog for a specific version from CHANGELOG.md
 * Usage: node scripts/extract-changelog.js <version>
 * Example: node scripts/extract-changelog.js 1.0.0
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const version = process.argv[2];
if (!version) {
  console.error('Error: Version argument is required');
  console.error('Usage: node scripts/extract-changelog.js <version>');
  process.exit(1);
}

const changelogPath = path.join(__dirname, '..', 'CHANGELOG.md');

if (!fs.existsSync(changelogPath)) {
  console.error('Error: CHANGELOG.md not found');
  process.exit(1);
}

const changelogContent = fs.readFileSync(changelogPath, 'utf-8');
const versionHeading = new RegExp(`^## \\[?${version}\\]?.*$`, 'm');
const lines = changelogContent.split('\n');
let inVersion = false;
let changelogLines = [];
let indentLevel = 0;

for (const line of lines) {
  if (versionHeading.test(line)) {
    inVersion = true;
    changelogLines.push(line);
    continue;
  }

  if (inVersion) {
    if (line.startsWith('## ')) {
      // Found next version section, stop
      break;
    }
    changelogLines.push(line);
  }
}

if (changelogLines.length === 0) {
  console.error(`Error: Version ${version} not found in CHANGELOG.md`);
  process.exit(1);
}

console.log(changelogLines.join('\n'));
