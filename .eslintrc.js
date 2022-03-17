const config = require('../../.eslintrc');

module.exports = {
  ...config,
  globals: {
    NodeJS: true,
  },
};
