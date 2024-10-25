module.exports = {
<<<<<<< HEAD
  ...require('./ava.config.cjs'),
};

module.exports.environmentVariables = {
  NEAR_WORKSPACES_NETWORK: 'testnet',
};

module.exports.files.push(
  '!__tests__/02*',
  '!__tests__/05*',
=======
  ...require('near-workspaces-ava/ava.testnet.config.cjs'),
  ...require('./ava.config.cjs'),
};

// Add files you only want to run in Sandbox mode here
module.exports.files.push(
  // '!__tests__/example-file-name*',
  // '!__tests__/another-example-file-name*',
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
);
