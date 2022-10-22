
## Use assembly to call an external contract
Instead of using Solidity to call an external contract, you can assembly for significant gas savings. 

Note: While the gas savings can be significant, it is important to thoroughly understand low level operations and the safegaurds needed when using the `call()` instruction in assembly. 

```js


contract GasTest is DSTest {
    Contract0 c0;
    Contract1 c1;

    function setUp() public {
        c0 = new Contract0();
        c1 = new Contract1();
    }

    function testGas() public {
        SimpleStore _simpleStore = new SimpleStore();

        ///@dev when testing, test one by one and comment out the others
        ///@dev otherwise you will get in inaccurate gas reading due to warm/cold addresses

        c0.functionCall(address(_simpleStore), address(this));
        // c1.assemblyCall(address(_simpleStore), address(this));
    }
}

contract Contract0 {
    function functionCall(address simpleStoreAddress, address newAddress)
        public
    {
        SimpleStore(simpleStoreAddress).store(newAddress);
    }
}

contract Contract1 {
    function assemblyCall(address simpleStoreAddress, address newAddress)
        public
    {
        bytes memory data = abi.encode(
            bytes4(keccak256("store(address)")),
            newAddress
        );

        uint256 dataLength = data.length;

        assembly {
            let success := call(
                //forward the gas left
                gas(),
                //addthe "to" address
                simpleStoreAddress,
                //add the msg value
                0,
                //memory offset where calldata starts,
                //32 bytes is added to the memory location of data because the first 32 bytes contains the length of the byte array
                add(data, 0x20),
                //size of calldata
                dataLength,
                //memory offset where to store the return data
                0x00,
                //size of the return data
                0x00
            )

            if iszero(success) {
                revert(0x00, 0x00)
            }
        }
    }
}

contract SimpleStore {
    address addr;

    function store(address _addr) public {
        addr = _addr;
    }
}

```

### Gas Report
```js
╭────────────────────┬─────────────────┬───────┬────────┬───────┬─────────╮
│ Contract0 contract ┆                 ┆       ┆        ┆       ┆         │
╞════════════════════╪═════════════════╪═══════╪════════╪═══════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆       ┆        ┆       ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 69717              ┆ 380             ┆       ┆        ┆       ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg   ┆ median ┆ max   ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ functionCall       ┆ 23082           ┆ 23082 ┆ 23082  ┆ 23082 ┆ 1       │
╰────────────────────┴─────────────────┴───────┴────────┴───────┴─────────╯
╭────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ Contract1 contract ┆                 ┆      ┆        ┆      ┆         │
╞════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 74723              ┆ 405             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ assemblyCall       ┆ 3105            ┆ 3105 ┆ 3105   ┆ 3105 ┆ 1       │
╰────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯

```