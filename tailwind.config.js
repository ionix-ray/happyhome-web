module.exports = {
  darkMode: 'class',
  content: [
    "./index.html",
    "./src/**/*.rs",
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
      },
      animation: {
        'fade-in': 'fadeIn 0.5s ease-in-out',
        'slide-up': 'slideUp 0.6s ease-out forwards',
        'bounce-slow': 'bounce 3s infinite',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(20px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        }
      },
      transitionProperty: {
        'height': 'height',
        'spacing': 'margin, padding',
        'all-smooth': 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
      }
    }
  }
}
