pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/Context.sol";
import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract Canvas1155 is Context, ERC1155, Ownable {
    mapping(string => uint256) private _name2token;
    mapping(uint256 => string) private _token2name;
    mapping(uint256 => string) private _token2ipfsHash;
    mapping(uint256 => string) private _token2s3Key;

    uint256 _localTokenId = 1;

    string public name = "C_NFT_1155";
    string public symbol = "CN";

    constructor() ERC1155("") {}

    function mint(
        address to,
        string memory workId,
        uint256 amount,
        string memory ipfsHash,
        string memory s3Key
    ) public virtual onlyOwner {
        require(_name2token[workId] == 0, "already mint");

        uint256 tokenId = _localTokenId;

        _name2token[workId] = tokenId;
        _token2name[tokenId] = workId;
        _token2ipfsHash[tokenId] = ipfsHash;
        _token2s3Key[tokenId] = s3Key;

        _mint(to, tokenId, amount, "");

        _localTokenId += 1;
    }

    function uri(uint256 tokenId)
        public
        view
        virtual
        override
        returns (string memory)
    {
        string memory ipfsHash = _token2ipfsHash[tokenId];
        string memory s3Key = _token2s3Key[tokenId];

        if (!isEmptyString(ipfsHash)) {
            return
                string(
                    abi.encodePacked(
                        "https://ipfs.moralis.io:2053/ipfs/",
                        ipfsHash
                    )
                );
        }

        if (!isEmptyString(s3Key)) {
            return
                string(
                    abi.encodePacked(
                        "https://canvas-nft-userdata.s3.ap-northeast-1.amazonaws.com/",
                        s3Key
                    )
                );
        }

        revert("unknown token");
    }

    function isEmptyString(string memory value) private pure returns (bool) {
        bytes memory b = bytes(value);
        return b.length == 0;
    }

    function tokenIdOf(string memory workId)
        public
        view
        virtual
        returns (uint256)
    {
        uint256 tokenId = _name2token[workId];
        if (tokenId == 0) {
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
