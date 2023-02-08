import { createStore } from 'solid-js/store';
import { AbilityName } from './Ability';
import { SkillName } from './SkillEntry';

interface Store {
    abilities: {
        [K in AbilityName]: number;
    };
    skills: {
        [K in SkillName]?: number;
    };
    savingThrows: {
        [K in AbilityName]?: number;
    };
}

const initialStore: Store = {
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
    savingThrows: {},
};

export const [store, setStore] = createStore(initialStore);

//@ts-ignore
window.store = store;
//@ts-ignore
window.setStore = setStore;
