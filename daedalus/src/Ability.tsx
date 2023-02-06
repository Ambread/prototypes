import { Component, createMemo, For } from 'solid-js';

type AbilityName = 'str' | 'dex' | 'con' | 'int' | 'wis' | 'cha';

type AbilityStyles = {
    [P in AbilityName]: {
        title: string;
        container: string;
        modifiers: string;
    };
};

const abilityStyles: AbilityStyles = {
    str: {
        title: 'Strength',
        container: 'border-str-700 text-str-500',
        modifiers: 'border-str-500',
    },
    dex: {
        title: 'Dexterity',
        container: 'border-dex-700 text-dex-500',
        modifiers: 'border-dex-500',
    },
    con: {
        title: 'Constitution',
        container: 'border-con-700 text-con-500',
        modifiers: 'border-con-500',
    },
    int: {
        title: 'Intelligence',
        container: 'border-int-700 text-int-500',
        modifiers: 'border-int-500',
    },
    wis: {
        title: 'Wisdom',
        container: 'border-wis-700 text-wis-500',
        modifiers: 'border-wis-500',
    },
    cha: {
        title: 'Charisma',
        container: 'border-cha-700 text-cha-500',
        modifiers: 'border-cha-500',
    },
};

export const Ability: Component<{ name: AbilityName }> = (props) => {
    const styles = createMemo(() => abilityStyles[props.name]);
    return (
        <div class={styles().container + ' border-2 p-2 m-5 rounded'}>
            <h1 class="text-3xl font-bold flex items-center">
                <span>{styles().title}</span>
                <span class="ml-auto mr-2">14 =</span>
                <span class={styles().modifiers + ' border-2 rounded-full p-2'}>
                    +2
                </span>
            </h1>
        </div>
    );
};
