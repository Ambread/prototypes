import { Component, createMemo, createSignal, For } from 'solid-js';

type AbilityName = 'str' | 'dex' | 'con' | 'int' | 'wis' | 'cha';

type AbilityStyles = {
    [P in AbilityName]: {
        title: string;
        header: string;
    };
};

const abilityStyles: AbilityStyles = {
    str: {
        title: 'Strength',
        header: 'bg-str-600',
    },
    dex: {
        title: 'Dexterity',
        header: 'bg-dex-600',
    },
    con: {
        title: 'Constitution',
        header: 'bg-con-600',
    },
    int: {
        title: 'Intelligence',
        header: 'bg-int-600',
    },
    wis: {
        title: 'Wisdom',
        header: 'bg-wis-600',
    },
    cha: {
        title: 'Charisma',
        header: 'bg-cha-600',
    },
};

interface Props {
    name: AbilityName;
    score: number;
}

export const Ability: Component<Props> = (props) => {
    const styles = createMemo(() => abilityStyles[props.name]);

    const [score, setScore] = createSignal(props.score);
    const modifier = createMemo(() => Math.floor((score() - 10) / 2));

    return (
        <header
            class={
                styles().header +
                ' p-4 m-5 rounded-xl text-3xl font-bold text-white flex'
            }
        >
            <h1 class="flex items-center ">{styles().title}</h1>
            <span class="bg-slate-800 rounded-full p-3 pr-0 ml-auto flex items-center h-12 w-32">
                <input
                    class="w-12 text-center bg-slate-800"
                    type="number"
                    min={0}
                    max={30}
                    value={score()}
                    onInput={(e) => setScore(parseInt(e.currentTarget.value))}
                />
                <span class="bg-slate-700 rounded-full ml-auto p-3 h-16 w-16 grid place-content-center">
                    {modifier()}
                </span>
            </span>
        </header>
    );
};
