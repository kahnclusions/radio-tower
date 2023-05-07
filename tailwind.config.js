/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/**/*.rs',
    './index.html',
    './src/**/*.html',
    './src/**/*.css'
  ],
  theme: {
    fontFamily: {
      sans: ['Noto Sans', 'Arial'],
      serif: ['Noto Serif', 'Georgia'],
      display: ['Silkscreen']
    },
    extend: {
      colors: {
        black: "#2C363C",
        white: "#F0EDEC",
        darkBg: "#1C1917",
        darkFg: "#B4BDC3",
        grey: {
          800: "#CBD9E3",
          700: "#A9BED1",
          600: "#728794",
          500: "#596A76",
          400: "#556570",
          300: "#4F5E68",
          200: "#44525B",
          100: "#3E4B53",
        },
        beige: {
          900: "#E9E4E2",
          800: "#DDD6D3",
          700: "#D4CDCA",
          600: "#CDC2BC",
          500: "#BBABA3",
          400: "#A4968F",
          300: "#8E817B",
          200: "#6A5549",
          100: "#564E4A",
        },
        red: {
          100: '#A8334C',
          200: '#94253E',
          300: '#EBD8DA'
        },
        green: {
          100: '#3F5A22',
          200: '#4F6C31',
          300: '#819B69',
          400: '#8BAE68',
          600: "#CBE5B8"
        },
        yellow: {
          100: '#944927',
          200: '#803D1C',
          300: "#EFDFDC",
        },
        blue: {
          100: '#286486',
          200: '#1D5573',
          300: "#D9E4EF",
        },
        magenta: {
          100: '#88507D',
          200: '#7B3B70',
          300: "#EFDEEB",
        },
        cyan: {
          100: '#3B8992',
          200: '#2B747C',
          300: "#D6EBED"
        },
      }
    },
  },
  plugins: [],
}
