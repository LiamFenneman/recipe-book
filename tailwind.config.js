/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {
            fontFamily: {
                'sans': ['Alata', 'sans-serif'],
                'black': ['Black Han Sans', 'sans-serif'],
            },
        },
    },
    plugins: [],
}
