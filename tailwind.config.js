module.exports = {
  darkMode: 'class',
  content: [
    "./crates/happyhome-ui/index.html",
    "./crates/happyhome-ui/src/**/*.rs",
  ],
  theme: {
    extend: {
      colors: {
        brand: {
          DEFAULT: '#36a1d7',
          dark: '#134cb3',
          deep: '#101a29',
          teal: '#2d8f7b',
        }
      },
      fontFamily: {
        sans: ['"Plus Jakarta Sans"', 'system-ui', '-apple-system', 'sans-serif'],
        serif: ['"DM Serif Display"', 'Georgia', 'serif'],
      }
    }
  }
}
