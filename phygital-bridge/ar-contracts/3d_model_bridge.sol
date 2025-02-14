// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

interface ISpaceNFT {
    function mintPhysical(address to, uint256 tokenId) external;
}

contract ModelBridge {
    address public constant NFT_CONTRACT = 0x...;
    uint256 public modelCounter;
    
    event PhysicalMinted(uint256 indexed tokenId);
    
    function bridgeToPhysical(uint256 digitalId) external {
        ISpaceNFT(NFT_CONTRACT).mintPhysical(msg.sender, digitalId);
        modelCounter++;
        emit PhysicalMinted(digitalId);
    }
}
