/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        './index.html',
        './src/**/*.{js,ts,jsx,tsx,css,md,mdx,html,json,scss}',
    ],
    darkMode: 'class',
    theme: {
        extend: {
            keyframes: {
                popup: {
                    '0%': { opacity: 0, transform: 'scale(0.5)' },
                    '100%': { opacity: 1, transform: 'scale(1)' },
                },
                zoom: {
                    '0%': { transform: 'scale(1)' },
                    '20%': { transform: 'scale(2)' },
                    '80%': { transform: 'scale(2)' },
                    '100%': { transform: 'scale(1)' },
                },
            },
            animation: { popup: 'popup 200ms', zoom: 'zoom 10000ms' },
        },
    },
    plugins: [],
};
