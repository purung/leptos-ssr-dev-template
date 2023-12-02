/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      gridTemplateColumns: {
        'smol': 'repeat(auto-fit, minmax(min(100%, var(--min)), 1fr))'
      }
    },
  },
  daisyui: {
    darkTheme: "business",
    themes: [
      {
        hoc: {
         neutral: '#090606',
         secondary: '#f7f3f3',
         "primary-content": '#f7f3f3',
         'base-100': '#7ea6a9',
         accent: '#dac9c8',
         primary: '#567e81',
        },
      },
      "business"
    ],
  },
  plugins: [require("daisyui")],
}
