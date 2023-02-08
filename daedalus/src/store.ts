import { createStore } from 'solid-js/store';

const initialState = {
    abilities: {
        str: 10,
        dex: 12,
        con: 14,
        wis: 16,
        int: 18,
        cha: 20,
    },
    skills: {
        Stealth: 1,
        Insight: 2,
    },
};

export const [store, setStore] = createStore(initialState);

//@ts-ignore
window.store = store;
//@ts-ignore
window.setStore = setStore;
