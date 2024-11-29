export let keyText = (event: KeyboardEvent): string => {
  const { key } = event;
  if (key.length > 1 && [...key].every(c => c.charCodeAt(0) < 0x100)) {
    return "";
  }
  return key;
}

export let keys: { [key: string]: number } = {
  ArrowDown: 2,
  ArrowLeft: 3,
  ArrowRight: 4,
  ArrowUp: 1,
  Escape: 6,
  Space: 5,
};
