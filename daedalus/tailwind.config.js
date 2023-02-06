const colors = require('tailwindcss/colors');

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        './index.html',
        './src/**/*.{js,ts,jsx,tsx,css,md,mdx,html,json,scss}',
    ],
    darkMode: 'class',
    theme: {
        extend: {
            colors: {
                str: colors.red,
                dex: colors.orange,
                con: colors.yellow,
                int: colors.blue,
                wis: colors.green,
                cha: colors.purple,
            },
        },
    },
    plugins: [],
};
