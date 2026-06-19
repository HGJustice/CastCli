// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract StringStore {
    string private storedString;

    function setString(string memory _newString) public {
        storedString = _newString;
    }

    function getString() public view returns (string memory) {
        return storedString;
    }
}