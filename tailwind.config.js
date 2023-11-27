/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.{html,rs}",
        "./node_modules/flowbite/**/*.js"
    ],
    theme: {
        extend: {},
    },
    plugins: [
        require('flowbite/plugin')
    ],
}