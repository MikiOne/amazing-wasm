- kt: https://github.com/dickwhite123/wallet-core/blob/36bce3da1408d2591c81593f3954d04a15dfccac/android/app/src/androidTest/java/com/trustwallet/core/app/blockchains/aptos/TestAptosSigner.kt#L48
```js


    // https://github.com/dickwhite123/wallet-core/blob/36bce3da1408d2591c81593f3954d04a15dfccac/android/app/src/androidTest/java/com/trustwallet/core/app/blockchains/aptos/TestAptosSigner.kt#L48
    const testSignAptosTx = () => {
        const {TW, WalletCore} = window;
        const {HexCoding, AnySigner, CoinType} = WalletCore;

        let key = HexCoding.decode("5d996aa76b3212142792d9130796cd2e11e3c445a93118c08414df4f66bc60ec");
        let payloadJson = {
            "function": "0x16fe2df00ea7dde4a63409201f7f4e536bde7bb7335526a35d05111e68aa322c::AnimeSwapPoolV1::swap_exact_coins_for_coins_3_pair_entry",
            "type_arguments": [
                "0x1::aptos_coin::AptosCoin",
                "0x881ac202b1f1e6ad4efcff7a1d0579411533f2502417a19211cfc49751ddb5f4::coin::MOJO",
                "0xf22bede237a07e121b56d91a491eb7bcdfd1f5907926a9e58338f964a01b17fa::asset::USDT",
                "0xf22bede237a07e121b56d91a491eb7bcdfd1f5907926a9e58338f964a01b17fa::asset::USDC"
            ],
            "arguments": [
                "1000000",
                "49329"
            ],
            "type": "entry_function_payload"
        };

        const input = TW.Aptos.Proto.SigningInput.create({
            chainId: 1,
            expirationTimestampSecs: 3664390082,
            gasUnitPrice: 100,
            maxGasAmount: 100011,
            sender: "0x07968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f30",
            sequenceNumber: 42,
            anyEncoded: payloadJson.toString(),
            privateKey: HexCoding.decode(
                "0x5d996aa76b3212142792d9130796cd2e11e3c445a93118c08414df4f66bc60ec"
            ),
        });

        const encoded = TW.Aptos.Proto.SigningInput.encode(input).finish();
        const outputData = AnySigner.sign(encoded, CoinType.aptos);
        const output = TW.Aptos.Proto.SigningOutput.decode(outputData);

        setRawTx(HexCoding.encode(output.encoded));
    };
```