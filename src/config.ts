export const getConfig = (name: string) => {
  const outDirectory = process.env[name];
  if (!outDirectory) {
    throw new Error(`No config found for ${name}`);
  }
  return outDirectory;
};

export const getWatchDirectory = () => {
  return getConfig('WATCH_DIR');
};

export const getDeadLetterDirectory = () => {
  return getConfig('DEAD_LETTER_DIR');
};
