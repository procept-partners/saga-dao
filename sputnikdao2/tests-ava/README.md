<<<<<<< HEAD
These tests use [near-workspaces](https://github.com/near/near-workspaces-js): delightful, deterministic local testing for NEAR smart contracts.
=======
These tests use [near-workspaces-ava](https://github.com/near/workspaces-js/tree/main/packages/ava): delightful, deterministic local testing for NEAR smart contracts.
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

You will need to install [NodeJS](https://nodejs.dev/). Then you can use the `scripts` defined in [package.json](./package.json):

    npm run test

<<<<<<< HEAD
If you want to run `ava` directly, you can use [npx](https://nodejs.dev/learn/the-npx-nodejs-package-runner):

=======
If you want to run `near-workspaces-ava` or `ava` directly, you can use [npx](https://nodejs.dev/learn/the-npx-nodejs-package-runner):

    npx near-workspaces-ava --help
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    npx ava --help

To run only one test file:

    npm run test "**/main*"         # matches test files starting with "main"
    npm run test "**/whatever/**/*" # matches test files in the "whatever" directory

To run only one test:

    npm run test -- -m "root sets*" # matches tests with titles starting with "root sets"
    yarn test -m "root sets*"       # same thing using yarn instead of npm, see https://yarnpkg.com/

If debugging:

    NEAR_WORKSPACES_DEBUG=true npm run test
