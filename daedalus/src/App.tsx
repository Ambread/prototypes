import { Component } from 'solid-js';
import { Ability } from './Ability';

export const App: Component = () => {
    return (
        <div class="h-screen grid grid-cols-12 grid-rows-1">
            <aside class="col-span-2 bg-slate-800"></aside>
            <section class="col-span-7 bg-slate-600"></section>
            <section class="col-span-3 bg-slate-700">
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
