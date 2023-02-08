import { createMemo } from 'solid-js';
import { AbilityName } from './Ability';
import { ParentComponent, VoidComponent } from 'solid-js/types/render';
import { JSX } from 'solid-js/types/jsx';

const entryStyles = {
    str: {
        border: 'border-str-600',
        bg: 'bg-str-600',
    },
    dex: {
        border: 'border-dex-600',
        bg: 'bg-dex-600',
    },
    con: {
        border: 'border-con-600',
        bg: 'bg-con-600',
    },
    wis: {
        border: 'border-wis-600',
        bg: 'bg-wis-600',
    },
    int: {
        border: 'border-int-600',
        bg: 'bg-int-600',
    },
    cha: {
        border: 'border-cha-600',
        bg: 'bg-cha-600',
    },
};

interface EntryProps {
    icon: JSX.Element;
    ability: AbilityName;
    label: string;
}

export const Entry: VoidComponent<EntryProps> = (props) => {
    const styles = createMemo(() => entryStyles[props.ability]);

    return (
        <div
            class={
                styles().border +
                ' rounded bg-slate-800 p-4 text-sm flex w-6/12 justify-between items-center shadow shadow-black/50 border-2 border-solid'
            }
        >
            <span
                class={
                    styles().bg + ' p-2 rounded shadow-inner shadow-slate-700'
                }
            >
                {props.icon}
            </span>

            <span>{props.label}</span>
        </div>
    );
};

export const EntryList: ParentComponent = (props) => {
    return <div class="w-6/12 flex flex-col gap-4">{props.children}</div>;
};

import { FiCrosshair } from 'solid-icons/fi';

export const SavingThrowEntry: VoidComponent<{ ability: AbilityName }> = (
    props,
) => {
    return (
        <Entry
            ability={props.ability}
            label="Saving Throw"
            icon={<FiCrosshair size={20} />}
        />
    );
};
