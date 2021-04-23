/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  mount: {
    public: { url: '/', static: true },
    src: { url: '/dist' },
  },
  plugins: [
    '@snowpack/plugin-react-refresh',
    '@snowpack/plugin-typescript',
  ],
  routes: [
    /* Enable an SPA Fallback in development: */
    // {"match": "routes", "src": ".*", "dest": "/index.html"},
  ],
  optimize: {
    /* Example: Bundle your final build: */
    // "bundle": true,
    //bundle: true,
    minify: true,
    target: "es2017"
  },
  packageOptions: {
    /* ... */
  },
  devOptions: {
    port: 3000,
    open: 'none',
    output: 'stream',
  },
  buildOptions: {
    /* ... */
  },
};
