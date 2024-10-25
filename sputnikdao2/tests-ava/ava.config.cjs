<<<<<<< HEAD
require('util').inspect.defaultOptions.depth = 5; // Increase AVA's printing depth

module.exports = {
  timeout: '300000',
  files: ['**/*.ava.ts', '**/*.ava.js', '!examples/**/*.ava.js'],
  failWithoutAssertions: false,
  extensions: [
    'ts',
    'js',
  ],
  require: [
    'ts-node/register',
  ],
=======
module.exports = {
  ...require('near-workspaces-ava/ava.config.cjs'),
  concurrency: 1,
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
};
