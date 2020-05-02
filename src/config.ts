import { config } from 'dotenv';

config();

export const getWatchDirectory = () => {
  return process.env.WATCH_DIR;
};

export const getDeadLetterDirectory = () => {
  return process.env.DEAD_LETTER_DIR;
};

export const getOutDirectory = (name: string) => {
  const outDirectory = process.env[name];
  if (!outDirectory) {
    throw new Error(`No config found for ${name}`);
  }
  return outDirectory;
};
