import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createVuetify } from 'vuetify'
import 'vuetify/styles'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { aliases, mdi } from 'vuetify/iconsets/mdi'
import '@mdi/font/css/materialdesignicons.css'

import App from './App.vue'
import router from './router'

const vuetify = createVuetify({
  components,
  directives,
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: { mdi },
  },
  theme: {
    defaultTheme: 'dark',
    themes: {
      dark: {
        dark: true,
        colors: {
          background: '#121212',
          surface: '#1E1E2E',
          'surface-bright': '#2A2A3C',
          'surface-light': '#333347',
          'surface-variant': '#262638',
          'on-surface-variant': '#B0B0C0',
          primary: '#89B4FA',
          'primary-darken-1': '#7CA4EA',
          secondary: '#CBA6F7',
          'secondary-darken-1': '#B896E7',
          accent: '#94E2D5',
          error: '#F38BA8',
          info: '#89B4FA',
          success: '#A6E3A1',
          warning: '#F9E2AF',
        },
      },
      light: {
        dark: false,
        colors: {
          background: '#EFF1F5',
          surface: '#FFFFFF',
          'surface-bright': '#FFFFFF',
          'surface-light': '#CCD0DA',
          'surface-variant': '#E6E9EF',
          'on-surface-variant': '#5C5F77',
          primary: '#1E66F5',
          'primary-darken-1': '#1A5CDD',
          secondary: '#8839EF',
          accent: '#179299',
          error: '#D20F39',
          info: '#1E66F5',
          success: '#40A02B',
          warning: '#DF8E1D',
        },
      },
    },
  },
  defaults: {
    VCard: {
      rounded: 'lg',
      elevation: 2,
    },
    VBtn: {
      rounded: 'lg',
    },
  },
})

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(vuetify)

app.mount('#app')
