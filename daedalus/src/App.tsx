import { Component, For } from 'solid-js';
import { Ability, abilityNames } from './Ability';
import { FiCrosshair } from 'solid-icons/fi';
import { Entry, EntryList, SavingThrowEntry } from './Entry';

export const App: Component = () => {
    return (
        <div class="h-screen grid grid-cols-12 grid-rows-1 text-white">
            <aside class="col-span-2 bg-slate-800"></aside>
            <section class="col-span-7 bg-slate-700 grid place-items-center">
                <EntryList>
                    <For each={abilityNames}>
                        {(name) => <SavingThrowEntry ability={name} />}
                    </For>
                </EntryList>
            </section>
            <section class="col-span-3 bg-slate-700 overflow-y-auto">
                <Ability name="str" score={16}></Ability>
                <Ability name="dex" score={14}></Ability>
                <Ability name="con" score={12}></Ability>
                <Ability name="wis" score={10}></Ability>
                <Ability name="int" score={8}></Ability>
                <Ability name="cha" score={6}></Ability>
            </section>
        </div>
    );
};
