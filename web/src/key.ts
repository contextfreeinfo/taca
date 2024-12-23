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
  Backspace: 8,
  Del: 9,
  Delete: 9,
  Enter: 7,
  Escape: 6,
  NumpadEnter: 10,
  Space: 5,
};
