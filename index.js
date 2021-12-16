import { fileURLToPath } from 'node:url';
import path from 'node:path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(fileURLToPath(import.meta.url));

import { loadBinding } from "@node-rs/helper";

const bbe = loadBinding(__dirname, "bengbenge", "bengbenge");

console.log('bbe', bbe);

export {bbe};


