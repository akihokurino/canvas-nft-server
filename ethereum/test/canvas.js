const Canvas721 = artifacts.require("Canvas721");
const Canvas1155 = artifacts.require("Canvas1155");

contract("Canvas721", (accounts) => {
  it("canvas methods test", async () => {
    const contract = await Canvas721.deployed();

    await contract.mint(accounts[0], "A");
    const tokenURI1 = await contract.tokenURI(1);
    assert.equal(
      tokenURI1,
      "https://canvas-nft-userdata.s3.ap-northeast-1.amazonaws.com/721_asset/A.metadata.json",
      "error"
    );

    await contract.mint(accounts[0], "B");
    const tokenURI2 = await contract.tokenURI(2);
    assert.equal(
      tokenURI2,
      "https://canvas-nft-userdata.s3.ap-northeast-1.amazonaws.com/721_asset/B.metadata.json",
      "error"
    );

    try {
      await contract.mint(accounts[1], "B");
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

    const supply = await contract.currentSupply();
    assert.equal(supply, 2, "error");

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

    await contract.mint(accounts[0], "A", 10);
    const tokenURI1 = await contract.uri(1);
    assert.equal(
      tokenURI1,
      "https://canvas-nft-userdata.s3.ap-northeast-1.amazonaws.com/1155_asset/A.metadata.json",
      "error"
    );

    await contract.mint(accounts[0], "B", 10);
    const tokenURI2 = await contract.uri(2);
    assert.equal(
      tokenURI2,
      "https://canvas-nft-userdata.s3.ap-northeast-1.amazonaws.com/1155_asset/B.metadata.json",
      "error"
    );

    try {
      await contract.mint(accounts[1], "B", 10);
      throw new Error();
    } catch (error) {
      assert.equal(error.reason, "already mint", "error");
    }

    await contract.mintBatch(accounts[0], ["AA", "BB"], [10, 20]);
    const tokenURI3 = await contract.uri(3);
    assert.equal(
      tokenURI3,
      "https://canvas-nft-userdata.s3.ap-northeast-1.amazonaws.com/1155_asset/AA.metadata.json",
      "error"
    );
    const tokenURI4 = await contract.uri(4);
    assert.equal(
      tokenURI4,
      "https://canvas-nft-userdata.s3.ap-northeast-1.amazonaws.com/1155_asset/BB.metadata.json",
      "error"
    );

    const tokenId1 = await contract.tokenIdOf("A");
    assert.equal(tokenId1, 1, "error");

    const tokenId2 = await contract.tokenIdOf("B");
    assert.equal(tokenId2, 2, "error");

    const tokenId3 = await contract.tokenIdOf("AA");
    assert.equal(tokenId3, 3, "error");

    const tokenId4 = await contract.tokenIdOf("BB");
    assert.equal(tokenId4, 4, "error");

    const tokenId5 = await contract.tokenIdOf("C");
    assert.equal(tokenId5, 0, "error");

    const names = await contract.usedTokenNames();
    assert.equal(names[0], "A", "error");
    assert.equal(names[1], "B", "error");
    assert.equal(names[2], "AA", "error");
    assert.equal(names[3], "BB", "error");
    assert.equal(names.length, 4, "error");
  });
});
