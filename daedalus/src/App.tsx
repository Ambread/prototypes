import { Component } from 'solid-js';
import { Ability } from './Ability';

export const App: Component = () => {
    return (
        <div class="h-screen grid grid-cols-12 grid-rows-1">
            <aside class="col-span-2 bg-slate-800"></aside>
            <section class="col-span-7 bg-slate-600"></section>
            <section class="col-span-3 bg-slate-700">
                <Ability name="str"></Ability>
                <Ability name="dex"></Ability>
                <Ability name="con"></Ability>
                <Ability name="wis"></Ability>
                <Ability name="int"></Ability>
                <Ability name="cha"></Ability>
            </section>
        </div>
    );
};
