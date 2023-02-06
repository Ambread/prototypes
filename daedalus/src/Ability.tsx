import { Component, createMemo, createSignal, For, Show } from 'solid-js';

type AbilityName = 'str' | 'dex' | 'con' | 'int' | 'wis' | 'cha';

type AbilityStyles = {
    [P in AbilityName]: {
        title: string;
        header: string;
        border: string;
        skills: string[];
    };
};

const abilityStyles: AbilityStyles = {
    str: {
        title: 'Strength',
        header: 'bg-str-600',
        border: 'border-str-600',
        skills: ['Athletics'],
    },
    dex: {
        title: 'Dexterity',
        header: 'bg-dex-500',
        border: 'border-dex-500',
        skills: ['Acrobatics', 'Slight of Hand', 'Stealth'],
    },
    con: {
        title: 'Constitution',
        header: 'bg-con-500',
        border: 'border-con-500',
        skills: [],
    },
    int: {
        title: 'Intelligence',
        header: 'bg-int-600',
        border: 'border-int-600',
        skills: ['Arcana', 'History', 'Investigation', 'Nature', 'Religion'],
    },
    wis: {
        title: 'Wisdom',
        header: 'bg-wis-600',
        border: 'border-wis-600',
        skills: [
            'Animal Handling',
            'Insight',
            'Medicine',
            'Perception',
            'Survival',
        ],
    },
    cha: {
        title: 'Charisma',
        header: 'bg-cha-600',
        border: 'border-cha-600',
        skills: ['Deception', 'Intimidation', 'Performance', 'Persuasion'],
    },
};

interface Props {
    name: AbilityName;
    score: number;
}

export const Ability: Component<Props> = (props) => {
    const [open, setOpen] = createSignal(false);

    const styles = createMemo(() => abilityStyles[props.name]);

    const [score, setScore] = createSignal(props.score);
    const modifier = createMemo(() => Math.floor((score() - 10) / 2));

    return (
        <div
            class={styles().border + ' m-5 p-0 border-2 rounded-2xl text-white'}
        >
            <header
                class={
                    styles().header +
                    ' p-3 pl-5 rounded-xl text-2xl font-bold flex'
                }
                onClick={() => setOpen((open) => !open)}
            >
                <h1 class="flex items-center">{styles().title}</h1>
                <span class="bg-slate-800 rounded-full p-3 pr-0 ml-auto flex items-center h-8 w-28">
                    <input
                        class="w-12 text-center bg-slate-800 outline-none"
                        type="number"
                        min={0}
                        max={30}
                        value={score()}
                        onInput={(e) => {
                            setScore(parseInt(e.currentTarget.value));
                        }}
                        onClick={(e) => e.stopPropagation()}
                    />
                    <span class="bg-slate-700 rounded-full ml-auto p-3 h-12 w-12 grid place-content-center">
                        {modifier()}
                    </span>
                </span>
            </header>
            <Show when={open()}>
                <section class="p-2 pl-5 rounded-xl text-xl flex">
                    <h1 class="flex items-center ">Saving Throw</h1>
                    <span
                        class={
                            styles().border +
                            ' w-12 ml-auto grid place-content-center'
                        }
                    >
                        {modifier()}
                    </span>
                </section>
                <Show when={styles().skills.length > 0}>
                    <div
                        class={styles().border + ' border-b-2 ml-4 mr-4'}
                    ></div>
                </Show>
                <For each={styles().skills}>
                    {(skill) => (
                        <section class="p-2 pl-5 rounded-xl text-xl flex">
                            <h1 class="flex items-center">{skill}</h1>
                            <span
                                class={
                                    styles().border +
                                    '  w-12 ml-auto grid place-content-center'
                                }
                            >
                                {modifier()}
                            </span>
                        </section>
                    )}
                </For>
            </Show>
        </div>
    );
};
