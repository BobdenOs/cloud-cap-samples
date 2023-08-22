const mod = require("./admin-service");

module.exports = new Promise((resolve) => {
  mod.onRuntimeInitialized = () => resolve(mod);
});
