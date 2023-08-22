const modules = [
  require("./admin-service/rust"),
  require("./admin-service/net"),
];

module.exports = async (srv) =>
  Promise.all(
    modules.map(async (m) => {
      m = await m; // allow module to load async
      srv[m.phase()](m.event(), m.entity(), async (..._) => m.exec(..._));
    })
  );
