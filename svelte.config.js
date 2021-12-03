import sveltePreprocess from 'svelte-preprocess';

module.exports = {
  extensions: [".svelte", ".svx", '.md'],
  preprocess: [
    mdsvex({ extensions: ['.svelte.md', '.md', '.svx'] }),
    sveltePreprocess({
      scss: {prependData: `@import 'src/styles/variables.scss';`},
      postcss: {
        plugins: [require('autoprefixer')()]
      }
    }),
  ],
};