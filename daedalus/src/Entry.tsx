import { createMemo } from 'solid-js';
import { AbilityName, abilityTitles } from './Ability';
import { ParentComponent, VoidComponent } from 'solid-js/types/render';
import { JSX } from 'solid-js/types/jsx';

const entryStyles = {
    str: {
        border: 'border-str-600',
        bg: 'bg-str-600',
        color: 'text-str-500',
    },
    dex: {
        border: 'border-dex-600',
        bg: 'bg-dex-600',
        color: 'text-dex-500',
    },
    con: {
        border: 'border-con-600',
        bg: 'bg-con-600',
        color: 'text-con-500',
    },
    wis: {
        border: 'border-wis-600',
        bg: 'bg-wis-600',
        color: 'text-wis-500',
    },
    int: {
        border: 'border-int-600',
        bg: 'bg-int-600',
        color: 'text-int-500',
    },
    cha: {
        border: 'border-cha-600',
        bg: 'bg-cha-600',
        color: 'text-cha-500',
    },
};

interface EntryProps {
    icon: JSX.Element;
    ability: AbilityName;
    label: string;
    base: string;
    modifier: string;
    prof?: string;
}

export const Entry: VoidComponent<EntryProps> = (props) => {
    const styles = createMemo(() => entryStyles[props.ability]);

    return (
        <div
            class={
                styles().border +
                ' rounded bg-slate-800 p-4 flex justify-between items-center shadow shadow-black/50 border-2 border-solid'
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

            <span class="font-bold text-gray-300">
                {props.base}
                <span class={styles().color}>{props.modifier}</span>
            </span>
        </div>
    );
};

export const EntryList: ParentComponent = (props) => {
    return (
        <div class="w-full p-24 flex flex-col gap-4 overflow-y-scroll h-full">
            {props.children}
        </div>
    );
};

import { FiCrosshair } from 'solid-icons/fi';
import { formatNumber, modifierFromAbility } from './App';
import { store } from './store';

export const SavingThrowEntry: VoidComponent<{ ability: AbilityName }> = (
    props,
) => {
    return (
        <Entry
            ability={props.ability}
            label={abilityTitles[props.ability] + ' Saving Throw'}
            icon={<FiCrosshair size={20} />}
            base={'1d20'}
            modifier={formatNumber(
                modifierFromAbility(store.abilities[props.ability]),
            )}
        />
    );
};
