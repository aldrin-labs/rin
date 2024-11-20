const withMT = require("@material-tailwind/html/utils/withMT");

/** @type {import('tailwindcss').Config} */
module.exports = withMT({
  content: ["./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        navy: {
          800: '#0a0a29',
          900: '#050514',
        },
      },
    },
  },
  plugins: [],
});