// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "openzeppelin/utils/structs/EnumerableSet.sol";

import "./LBErrors.sol";
import "./interfaces/ILBToken.sol";

/// @title Liquidity Book Token
/// @author Trader Joe
/// @notice The LBToken is an implementation of a multi-token.
/// It allows to create multi-ERC20 represented by their ids
contract LBToken is ILBToken {
    using EnumerableSet for EnumerableSet.UintSet;

    /// @dev Mapping from token id to account balances
    mapping(uint256 => mapping(address => uint256)) private _balances;

    /// @dev Mapping from account to spender approvals
    mapping(address => mapping(address => bool)) private _spenderApprovals;

    /// @dev Mapping from token id to total supplies
    mapping(uint256 => uint256) private _totalSupplies;

    /// @dev  Mapping from account to set of ids, where user currently have a non-zero balance
    mapping(address => EnumerableSet.UintSet) private _userIds;

    string private constant _name = "Liquidity Book Token";
    string private constant _symbol = "LBT";

    modifier checkApproval(address _from, address _spender) {
        if (!_isApprovedForAll(_from, _spender)) revert LBToken__SpenderNotApproved(_from, _spender);
        _;
    }

    modifier checkAddresses(address _from, address _to) {
        if (_from == address(0) || _to == address(0)) revert LBToken__TransferFromOrToAddress0();
        _;
    }

    modifier checkLength(uint256 _lengthA, uint256 _lengthB) {
        if (_lengthA != _lengthB) revert LBToken__LengthMismatch(_lengthA, _lengthB);
        _;
    }

    /// @notice Returns the name of the token
    /// @return The name of the token
    function name() public pure virtual override returns (string memory) {
        return _name;
    }

    /// @notice Returns the symbol of the token, usually a shorter version of the name
    /// @return The symbol of the token
    function symbol() public pure virtual override returns (string memory) {
        return _symbol;
    }

    /// @notice Returns the total supply of token of type `id`
    /// @dev This is the amount of token of type `id` minted minus the amount burned
    /// @param _id The token id
    /// @return The total supply of that token id
    function totalSupply(uint256 _id) public view virtual override returns (uint256) {
        return _totalSupplies[_id];
    }

    /// @notice Returns the amount of tokens of type `id` owned by `_account`
    /// @param _account The address of the owner
    /// @param _id The token id
    /// @return The amount of tokens of type `id` owned by `_account`
    function balanceOf(address _account, uint256 _id) public view virtual override returns (uint256) {
        return _balances[_id][_account];
    }

    /// @notice Return the balance of multiple (account/id) pairs
    /// @param _accounts The addresses of the owners
    /// @param _ids The token ids
    /// @return batchBalances The balance for each (account, id) pair
    function balanceOfBatch(address[] memory _accounts, uint256[] memory _ids)
        public
        view
        virtual
        override
        checkLength(_accounts.length, _ids.length)
        returns (uint256[] memory batchBalances)
    {
        batchBalances = new uint256[](_accounts.length);

        unchecked {
            for (uint256 i; i < _accounts.length; ++i) {
                batchBalances[i] = balanceOf(_accounts[i], _ids[i]);
            }
        }
    }

    /// @notice Returns the type id at index `_index` where `account` has a non-zero balance
    /// @param _account The address of the account
    /// @param _index The position index
    /// @return The `account` non-zero position at index `_index`
    function userPositionAtIndex(address _account, uint256 _index) public view virtual override returns (uint256) {
        return _userIds[_account].at(_index);
    }

    /// @notice Returns the number of non-zero balances of `account`
    /// @param _account The address of the account
    /// @return The number of non-zero balances of `account`
    function userPositionNumber(address _account) public view virtual override returns (uint256) {
        return _userIds[_account].length();
    }

    /// @notice Returns true if `spender` is approved to transfer `_account`'s tokens
    /// @param _owner The address of the owner
    /// @param _spender The address of the spender
    /// @return True if `spender` is approved to transfer `_account`'s tokens
    function isApprovedForAll(address _owner, address _spender) public view virtual override returns (bool) {
        return _isApprovedForAll(_owner, _spender);
    }

    /// @notice Grants or revokes permission to `spender` to transfer the caller's tokens, according to `approved`
    /// @param _spender The address of the spender
    /// @param _approved The boolean value to grant or revoke permission
    function setApprovalForAll(address _spender, bool _approved) public virtual override {
        _setApprovalForAll(msg.sender, _spender, _approved);
    }

    /// @notice Transfers `_amount` token of type `_id` from `_from` to `_to`
    /// @param _from The address of the owner of the token
    /// @param _to The address of the recipient
    /// @param _id The token id
    /// @param _amount The amount to send
    function safeTransferFrom(
        address _from,
        address _to,
        uint256 _id,
        uint256 _amount
    ) public virtual override checkAddresses(_from, _to) checkApproval(_from, msg.sender) {
        address _spender = msg.sender;

        _transfer(_from, _to, _id, _amount);

        emit TransferSingle(_spender, _from, _to, _id, _amount);
    }

    /// @notice Batch transfers `_amount` tokens of type `_id` from `_from` to `_to`
    /// @param _from The address of the owner of the tokens
    /// @param _to The address of the recipient
    /// @param _ids The list of token ids
    /// @param _amounts The list of amounts to send
    function safeBatchTransferFrom(
        address _from,
        address _to,
        uint256[] memory _ids,
        uint256[] memory _amounts
    )
        public
        virtual
        override
        checkLength(_ids.length, _amounts.length)
        checkAddresses(_from, _to)
        checkApproval(_from, msg.sender)
    {
        unchecked {
            for (uint256 i; i < _ids.length; ++i) {
                _transfer(_from, _to, _ids[i], _amounts[i]);
            }
        }

        emit TransferBatch(msg.sender, _from, _to, _ids, _amounts);
    }

    /// @notice Internal function to transfer `_amount` tokens of type `_id` from `_from` to `_to`
    /// @param _from The address of the owner of the token
    /// @param _to The address of the recipient
    /// @param _id The token id
    /// @param _amount The amount to send
    function _transfer(
        address _from,
        address _to,
        uint256 _id,
        uint256 _amount
    ) internal virtual {
        uint256 _fromBalance = _balances[_id][_from];
        if (_fromBalance < _amount) revert LBToken__TransferExceedsBalance(_from, _id, _amount);

        _beforeTokenTransfer(_from, _to, _id, _amount);

        uint256 _toBalance = _balances[_id][_to];

        unchecked {
            _balances[_id][_from] = _fromBalance - _amount;
            _balances[_id][_to] = _toBalance + _amount;
        }

        _remove(_from, _id, _fromBalance, _amount);
        _add(_to, _id, _toBalance, _amount);
    }

    /// @dev Creates `_amount` tokens of type `_id`, and assigns them to `_account`
    /// @param _account The address of the recipient
    /// @param _id The token id
    /// @param _amount The amount to mint
    function _mint(
        address _account,
        uint256 _id,
        uint256 _amount
    ) internal virtual {
        if (_account == address(0)) revert LBToken__MintToAddress0();

        _beforeTokenTransfer(address(0), _account, _id, _amount);

        _totalSupplies[_id] += _amount;

        uint256 _accountBalance = _balances[_id][_account];
        unchecked {
            _balances[_id][_account] = _accountBalance + _amount;
        }

        _add(_account, _id, _accountBalance, _amount);

        emit TransferSingle(msg.sender, address(0), _account, _id, _amount);
    }

    /// @dev Destroys `_amount` tokens of type `_id` from `_account`
    /// @param _account The address of the owner
    /// @param _id The token id
    /// @param _amount The amount to destroy
    function _burn(
        address _account,
        uint256 _id,
        uint256 _amount
    ) internal virtual {
        if (_account == address(0)) revert LBToken__BurnFromAddress0();

        uint256 _accountBalance = _balances[_id][_account];
        if (_accountBalance < _amount) revert LBToken__BurnExceedsBalance(_account, _id, _amount);

        _beforeTokenTransfer(address(0), _account, _id, _amount);

        unchecked {
            _balances[_id][_account] = _accountBalance - _amount;
            _totalSupplies[_id] -= _amount;
        }

        _remove(_account, _id, _accountBalance, _amount);

        emit TransferSingle(msg.sender, _account, address(0), _id, _amount);
    }

    /// @notice Grants or revokes permission to `spender` to transfer the caller's tokens, according to `approved`
    /// @param _owner The address of the owner
    /// @param _spender The address of the spender
    /// @param _approved The boolean value to grant or revoke permission
    function _setApprovalForAll(
        address _owner,
        address _spender,
        bool _approved
    ) internal virtual {
        if (_owner == _spender) revert LBToken__SelfApproval(_owner);

        _spenderApprovals[_owner][_spender] = _approved;
        emit ApprovalForAll(_owner, _spender, _approved);
    }

    /// @notice Returns true if `spender` is approved to transfer `owner`'s tokens
    /// or if `sender` is the `owner`
    /// @param _owner The address of the owner
    /// @param _spender The address of the spender
    /// @return True if `spender` is approved to transfer `owner`'s tokens
    function _isApprovedForAll(address _owner, address _spender) internal view virtual returns (bool) {
        return _owner == _spender || _spenderApprovals[_owner][_spender];
    }

    /// @notice Internal function to add an id to an user's set
    /// @param _account The user's address
    /// @param _id The id of the token
    /// @param _accountBalance The user's balance
    /// @param _amount The amount of tokens
    function _add(
        address _account,
        uint256 _id,
        uint256 _accountBalance,
        uint256 _amount
    ) internal {
        if (_accountBalance == 0 && _amount != 0) {
            _userIds[_account].add(_id);
        }
    }

    /// @notice Internal function to remove an id from an user's set
    /// @param _account The user's address
    /// @param _id The id of the token
    /// @param _accountBalance The user's balance
    /// @param _amount The amount of tokens
    function _remove(
        address _account,
        uint256 _id,
        uint256 _accountBalance,
        uint256 _amount
    ) internal {
        if (_accountBalance == _amount && _amount != 0) {
            _userIds[_account].remove(_id);
        }
    }

    /// @notice Hook that is called before any token transfer. This includes minting
    /// and burning.
    ///
    /// Calling conditions (for each `id` and `amount` pair):
    ///
    /// - When `from` and `to` are both non-zero, `amount` of ``from``'s tokens
    /// of token type `id` will be  transferred to `to`.
    /// - When `from` is zero, `amount` tokens of token type `id` will be minted
    /// for `to`.
    /// - when `to` is zero, `amount` of ``from``'s tokens of token type `id`
    /// will be burned.
    /// - `from` and `to` are never both zero.
    /// @param from The address of the owner of the token
    /// @param to The address of the recipient of the  token
    /// @param id The id of the token
    /// @param amount The amount of token of type `id`
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 id,
        uint256 amount
    ) internal virtual {}
}
