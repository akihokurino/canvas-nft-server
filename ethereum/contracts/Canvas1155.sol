pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/Context.sol";
import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract Canvas1155 is Context, ERC1155, Ownable {
    uint256 _localTokenId = 1;
    mapping(string => uint256) private _name2token;
    mapping(uint256 => string) private _token2name;

    string public name = "Canvas1155";
    string public symbol = "CV";

    constructor() ERC1155("") {}

    function mint(
        address to,
        string memory workId,
        uint256 amount
    ) public virtual onlyOwner {
        require(_name2token[workId] == 0, "already mint");

        uint256 tokenId = _localTokenId;

        _name2token[workId] = tokenId;
        _token2name[tokenId] = workId;

        _mint(to, tokenId, amount, "");

        _localTokenId += 1;
    }

    function mintBatch(
        address to,
        string[] memory workIds,
        uint256[] memory amounts
    ) public {
        require(workIds.length <= 10);

        uint256[] memory tokenIds = new uint256[](workIds.length);
        for (uint256 i = 0; i < workIds.length; i++) {
            string memory workId = workIds[i];
            require(_name2token[workId] == 0, "already mint");

            uint256 tokenId = _localTokenId;

            _name2token[workId] = tokenId;
            _token2name[tokenId] = workId;

            tokenIds[i] = tokenId;

            _localTokenId += 1;
        }

        _mintBatch(to, tokenIds, amounts, "");
    }

    function uri(uint256 tokenId)
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
                    "https://canvas-nft-userdata.s3.ap-northeast-1.amazonaws.com/1155_asset/",
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
