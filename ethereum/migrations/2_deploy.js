const Canvas721 = artifacts.require("Canvas721");
const Canvas1155 = artifacts.require("Canvas1155");

module.exports = function (deployer) {
  deployer.deploy(Canvas721);
  deployer.deploy(Canvas1155);
};
