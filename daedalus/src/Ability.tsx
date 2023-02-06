import { Component, createMemo, For } from 'solid-js';

type AbilityName = 'str' | 'dex' | 'con' | 'int' | 'wis' | 'cha';

type AbilityStyles = {
    [P in AbilityName]: {
        title: string;
        container: string;
    };
};

const abilityStyles: AbilityStyles = {
    str: {
        title: 'Strength',
        container: 'bg-str-600',
    },
    dex: {
        title: 'Dexterity',
        container: 'bg-dex-600',
    },
    con: {
        title: 'Constitution',
        container: 'bg-con-600',
    },
    int: {
        title: 'Intelligence',
        container: 'bg-int-600',
    },
    wis: {
        title: 'Wisdom',
        container: 'bg-wis-600',
    },
    cha: {
        title: 'Charisma',
        container: 'bg-cha-600',
    },
};

interface Props {
    name: AbilityName;
    score: number;
}

export const Ability: Component<Props> = (props) => {
    const styles = createMemo(() => abilityStyles[props.name]);
    const modifier = createMemo(() => Math.floor((props.score - 10) / 2));

    return (
        <div class={styles().container + ' p-4 m-5 rounded-xl'}>
            <h1 class="text-3xl font-bold flex items-center text-white">
                <span>{styles().title}</span>
                <span class="bg-slate-700 rounded-full p-3 pr-0 ml-auto">
                    {props.score}
                    <span class="bg-slate-600 rounded-full ml-3 p-3">
                        {modifier()}
                    </span>
                </span>
            </h1>
        </div>
    );
};
