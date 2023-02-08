import { Component } from 'solid-js';
import { Ability } from './Ability';
import { EntryList, SavingThrowEntry } from './Entry';
import { SkillEntry } from './SkillEntry';

export const App: Component = () => {
    return (
        <div class="h-screen grid grid-cols-12 grid-rows-1 text-white">
            <aside class="col-span-2 bg-slate-800"></aside>
            <section class="col-span-6 bg-slate-700 grid place-items-center">
                <EntryList>
                    <SavingThrowEntry ability="str" />
                    <SkillEntry skill="Athletics" />
                    <SavingThrowEntry ability="dex" />
                    <SkillEntry skill="Acrobatics" />
                    <SkillEntry skill="Sleight of Hand" />
                    <SkillEntry skill="Stealth" />
                    <SavingThrowEntry ability="con" />
                    <SavingThrowEntry ability="wis" />
                    <SkillEntry skill="Animal Handling" />
                    <SkillEntry skill="Insight" />
                    <SkillEntry skill="Medicine" />
                    <SkillEntry skill="Perception" />
                    <SkillEntry skill="Survival" />
                    <SavingThrowEntry ability="int" />
                    <SkillEntry skill="Arcana" />
                    <SkillEntry skill="History" />
                    <SkillEntry skill="Investigation" />
                    <SkillEntry skill="Nature" />
                    <SkillEntry skill="Religion" />
                    <SavingThrowEntry ability="cha" />
                    <SkillEntry skill="Deception" />
                    <SkillEntry skill="Intimidation" />
                    <SkillEntry skill="Performance" />
                    <SkillEntry skill="Persuasion" />
                </EntryList>
            </section>
            <section class="col-span-4 bg-slate-700">
                <EntryList>
                    <SavingThrowEntry ability="str" />
                    <SkillEntry skill="Athletics" />
                    <SavingThrowEntry ability="dex" />
                    <SkillEntry skill="Acrobatics" />
                    <SkillEntry skill="Sleight of Hand" />
                    <SkillEntry skill="Stealth" />
                    <SavingThrowEntry ability="con" />
                    <SavingThrowEntry ability="wis" />
                    <SkillEntry skill="Animal Handling" />
                    <SkillEntry skill="Insight" />
                    <SkillEntry skill="Medicine" />
                    <SkillEntry skill="Perception" />
                    <SkillEntry skill="Survival" />
                    <SavingThrowEntry ability="int" />
                    <SkillEntry skill="Arcana" />
                    <SkillEntry skill="History" />
                    <SkillEntry skill="Investigation" />
                    <SkillEntry skill="Nature" />
                    <SkillEntry skill="Religion" />
                    <SavingThrowEntry ability="cha" />
                    <SkillEntry skill="Deception" />
                    <SkillEntry skill="Intimidation" />
                    <SkillEntry skill="Performance" />
                    <SkillEntry skill="Persuasion" />
                </EntryList>
            </section>
        </div>
    );
};
