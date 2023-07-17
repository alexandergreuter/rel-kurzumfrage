interface Entries {
    votedLocations: string[]
}

const initialValues: {[key in keyof Entries]: Entries[key]} = {
    votedLocations: []
}

export function mutateLocal<T extends keyof Entries>(
  key: T,
  mutate: (value: Entries[T]) => Entries[T]
) {
    localStorage.setItem(key, JSON.stringify(mutate(getLocal(key) ?? initialValues[key])));
}

export function getLocal<T extends keyof Entries>(key: T): Entries[T] {
    const result = localStorage.getItem(key);

    if (result == null) {
        return initialValues[key];
    }

    return JSON.parse(result) as Entries[T];
}
