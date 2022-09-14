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

export const plural = (value: number, singular: string, plural: string) =>
    value === 1 ? singular : plural;
