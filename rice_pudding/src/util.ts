import { User } from 'discord.js';
import { prisma } from '.';

const englishOrdinalRules = new Intl.PluralRules('en', { type: 'ordinal' });
const suffixes = {
    zero: 'th',
    one: 'st',
    two: 'nd',
    few: 'rd',
    many: 'th',
    other: 'th',
};

export const ordinal = (value: number) =>
    value + suffixes[englishOrdinalRules.select(value)];

export const getUserData = (user: User) =>
    prisma.user.upsert({
        where: { id: user.id },
        update: { username: user.username },
        create: { id: user.id, username: user.username },
    });
