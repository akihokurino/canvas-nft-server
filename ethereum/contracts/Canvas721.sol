pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/Context.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract Canvas721 is Context, ERC721Enumerable, Ownable {
    mapping(string => uint256) private _name2token;
    mapping(uint256 => string) private _token2name;
    uint256 _localTokenId = 1;

    constructor() ERC721("Canvas721", "CV") {}

    function mint(address to, string memory workId) public virtual onlyOwner {
        require(_name2token[workId] == 0, "already mint");

        uint256 tokenId = _localTokenId;

        _name2token[workId] = tokenId;
        _token2name[tokenId] = workId;
        _mint(to, tokenId);

        _localTokenId += 1;
    }

    function ownerAddressOf(string memory workId)
        public
        view
        virtual
        returns (address)
    {
        uint256 tokenId = _name2token[workId];

        if (!_exists(tokenId)) {
            return address(0);
        }

        return ownerOf(tokenId);
    }

    function isOwn(address addr, string memory workId)
        public
        view
        virtual
        returns (bool)
    {
        uint256 tokenId = _name2token[workId];

        if (!_exists(tokenId)) {
            return false;
        }

        return addr == ownerOf(tokenId);
    }

    function currentSupply() public view virtual returns (uint256) {
        return _localTokenId - 1;
    }

    function tokenURI(uint256 tokenId)
        public
        view
        virtual
        override
        returns (string memory)
    {
        string memory workId = _token2name[tokenId];

        return
            string(
                abi.encodePacked(
                    "https://canvas-nft-userdata.s3.ap-northeast-1.amazonaws.com/721_asset/",
                    workId,
                    ".metadata.json"
                )
            );
    }

    function tokenIdOf(string memory workId)
        public
        view
        virtual
        returns (uint256)
    {
        uint256 tokenId = _name2token[workId];
        if (!_exists(tokenId)) {
            return 0;
        }

        return tokenId;
    }

    function usedTokenNames() public view virtual returns (string[] memory) {
        if (_localTokenId == 1) {
            return new string[](0);
        }
        uint256 len = _localTokenId - 1;
        string[] memory names = new string[](len);
        for (uint256 i = 0; i < len; i++) {
            names[i] = _token2name[i + 1];
        }
        return names;
    }
}
