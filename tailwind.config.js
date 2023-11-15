/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
    },
  },
  daisyui: {
    themes: [
      {
        hoc: {
         neutral: '#090606',
         'base-100': '#f7f3f3',
         primary: '#7ea6a9',
         secondary: '#dac9c8',
         accent: '#567e81',
        },
      },
      "dark"
    ],
  },
  plugins: [require("daisyui")],
}
