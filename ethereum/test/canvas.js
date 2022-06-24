const Canvas721 = artifacts.require("Canvas721");
const Canvas1155 = artifacts.require("Canvas1155");

contract("Canvas721", (accounts) => {
  it("canvas methods test", async () => {
    const contract = await Canvas721.deployed();

    await contract.mint(accounts[0], "A", "", "721_asset/A.metadata.json");
    const tokenURI1 = await contract.tokenURI(1);
    assert.equal(
      tokenURI1,
      "https://canvas-nft-userdata.s3.ap-northeast-1.amazonaws.com/721_asset/A.metadata.json",
      "error"
    );

    await contract.mint(
      accounts[0],
      "B",
      "Qme4nCCQgRSeiprzEAmKuVxEjmmAfHhwhWJw4xe1pG7mhD",
      ""
    );
    const tokenURI2 = await contract.tokenURI(2);
    assert.equal(
      tokenURI2,
      "https://ipfs.moralis.io:2053/ipfs/Qme4nCCQgRSeiprzEAmKuVxEjmmAfHhwhWJw4xe1pG7mhD",
      "error"
    );

    try {
      await contract.mint(accounts[1], "B", "", "");
      throw new Error();
    } catch (error) {
      assert.equal(error.reason, "already mint", "error");
    }

    const address1 = await contract.ownerAddressOf("A");
    assert.equal(address1, accounts[0], "error");

    const address2 = await contract.ownerAddressOf("B");
    assert.equal(address2, accounts[0], "error");

    const address3 = await contract.ownerAddressOf("C");
    assert.equal(
      address3,
      "0x0000000000000000000000000000000000000000",
      "error"
    );

    const isOwn1 = await contract.isOwn(accounts[0], "A");
    assert.equal(isOwn1, true, "error");

    const isOwn2 = await contract.isOwn(accounts[1], "A");
    assert.equal(isOwn2, false, "error");

    const tokenId1 = await contract.tokenIdOf("A");
    assert.equal(tokenId1, 1, "error");

    const tokenId2 = await contract.tokenIdOf("B");
    assert.equal(tokenId2, 2, "error");

    const tokenId3 = await contract.tokenIdOf("C");
    assert.equal(tokenId3, 0, "error");

    const names = await contract.usedTokenNames();
    assert.equal(names[0], "A", "error");
    assert.equal(names[1], "B", "error");
    assert.equal(names.length, 2, "error");
  });
});

contract("Canvas1155", (accounts) => {
  it("canvas methods test", async () => {
    const contract = await Canvas1155.deployed();

    await contract.mint(accounts[0], "A", 10, "", "1155_asset/A.metadata.json");
    const tokenURI1 = await contract.uri(1);
    assert.equal(
      tokenURI1,
      "https://canvas-nft-userdata.s3.ap-northeast-1.amazonaws.com/1155_asset/A.metadata.json",
      "error"
    );

    await contract.mint(
      accounts[0],
      "B",
      10,
      "Qme4nCCQgRSeiprzEAmKuVxEjmmAfHhwhWJw4xe1pG7mhD",
      ""
    );
    const tokenURI2 = await contract.uri(2);
    assert.equal(
      tokenURI2,
      "https://ipfs.moralis.io:2053/ipfs/Qme4nCCQgRSeiprzEAmKuVxEjmmAfHhwhWJw4xe1pG7mhD",
      "error"
    );

    try {
      await contract.mint(accounts[1], "B", 10, "", "");
      throw new Error();
    } catch (error) {
      assert.equal(error.reason, "already mint", "error");
    }

    const tokenId1 = await contract.tokenIdOf("A");
    assert.equal(tokenId1, 1, "error");

    const tokenId2 = await contract.tokenIdOf("B");
    assert.equal(tokenId2, 2, "error");

    const tokenId3 = await contract.tokenIdOf("C");
    assert.equal(tokenId3, 0, "error");

    const names = await contract.usedTokenNames();
    assert.equal(names[0], "A", "error");
    assert.equal(names[1], "B", "error");
    assert.equal(names.length, 2, "error");
  });
});
