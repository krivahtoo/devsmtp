import adapter from '@sveltejs/adapter-static';

const config = {
  kit: {
    adapter: adapter({
      fallback: "404.html"
    }),

    alias: {
    "@/*": "./src/lib/*",
    }
  }
};

export default config;
