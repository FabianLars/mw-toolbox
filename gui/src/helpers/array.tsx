const removeFirst = (array: string[], element: string): string[] => {
    const index = array.indexOf(element);
    if (index === -1) return array;
    return [...array.slice(0, index), ...array.slice(index + 1)];
};

export { removeFirst };
