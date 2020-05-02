import * as bencode from 'bencode';
import * as chokidar from 'chokidar';
import { parse as parseURL } from 'url';
import { readFileSync, writeFileSync, unlinkSync } from 'fs';
import { basename, join } from 'path';
import { getWatchDirectory, getDeadLetterDirectory, getConfig } from './config';

const filesToWatch = `${getWatchDirectory()}/**.torrent`;
console.log(`Watching: ${filesToWatch}`);

const generateConfigName = (url: string) =>
  parseURL(url).hostname.replace(/\./g, '_').toUpperCase();

const processTorrent = (path: string, fileName: string, data: Buffer) => {
  const decoded = bencode.decode(data, 'utf8');
  if (decoded.announce) {
    const { announce } = decoded;
    const configName = generateConfigName(announce);
    const outDir = getConfig(configName);
    const outPath = join(outDir, fileName);
    writeFileSync(outPath, data);
    console.info(
      `Processed: "${fileName}" using config: "${configName}" sent to "${outDir}"`,
    );
  }
};

chokidar
  .watch(filesToWatch, { awaitWriteFinish: true, usePolling: true })
  .on('add', (path) => {
    try {
      const fileName = basename(path);
      const data = readFileSync(path);
      try {
        processTorrent(path, fileName, data);
      } catch (error) {
        console.error(`Unable to process file: ${path} - ${error}`);
        const outPath = join(getDeadLetterDirectory(), fileName);
        writeFileSync(outPath, data);
      }
      unlinkSync(path);
    } catch (error) {
      console.error(`Failed to process path: ${path} - ${error}`);
    }
  });
