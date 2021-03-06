# react-native-chain-libs

## Getting started

`$ npm install react-native-chain-libs --save`

### Mostly automatic installation

`$ react-native link react-native-chain-libs`

### Manual installation


#### iOS

1. In XCode, in the project navigator, right click `Libraries` ➜ `Add Files to [your project's name]`
2. Go to `node_modules` ➜ `react-native-chain-libs` and add `ChainLibs.xcodeproj`
3. In XCode, in the project navigator, select your project. Add `libChainLibs.a` to your project's `Build Phases` ➜ `Link Binary With Libraries`
4. Run your project (`Cmd+R`)<

#### Android

1. Open up `android/app/src/main/java/[...]/MainApplication.java`
  - Add `import com.reactlibrary.ChainLibsPackage;` to the imports at the top of the file
  - Add `new ChainLibsPackage()` to the list returned by the `getPackages()` method
2. Append the following lines to `android/settings.gradle`:
  	```
  	include ':react-native-chain-libs'
  	project(':react-native-chain-libs').projectDir = new File(rootProject.projectDir, 	'../node_modules/react-native-chain-libs/android')
  	```
3. Insert the following lines inside the dependencies block in `android/app/build.gradle`:
  	```
      compile project(':react-native-chain-libs')
  	```


## Usage
```javascript
import {
    OutputPolicy,
    TransactionBuilder,
    Address,
    Input,
    Value,
    Fee,
    Fragment,
    PrivateKey,
    Witness,
    SpendingCounter,
    Hash,
    Account,
    InputOutputBuilder,
    Witnesses,
    PayloadAuthData,
    Payload,
} from 'react-native-chain-libs';
​
​function buffer2hex(buffer) {
    return Array.prototype.map.call(new Uint8Array(buffer), x => ('00' + x.toString(16)).slice(-2)).join('');
​}
​
const accountInputAddress = await Address.from_string(
    'ca1qh9u0nxmnfg7af8ycuygx57p5xgzmnmgtaeer9xun7hly6mlgt3pj2xk344'
);
const account = await Account.from_address(accountInputAddress);

const input = await Input.from_account(account, await Value.from_str('1000'));

account.free();

const ioBuilder = await InputOutputBuilder.empty();

await ioBuilder.add_input(input);

input.free();

const address = await Address.from_string(
    'ca1q5nr5pvt9e5p009strshxndrsx5etcentslp2rwj6csm8sfk24a2w3swacn'
);

const value500 = await Value.from_str('500');

await ioBuilder.add_output(
    address,
    value500,
);

address.free();
value500.free();

const value20 = await Value.from_str('20');
const value5 = await Value.from_str('5');
const value0 = await Value.from_str('0');

const feeAlgorithm = await Fee.linear_fee(
    // constant fee
    value20,
    // coefficient
    value5,
    // certificate cost
    value0,
);

value20.free();
value5.free();
value0.free();

const payload = await Payload.no_payload();
const policy = await OutputPolicy.one(accountInputAddress);

accountInputAddress.free();

const IOs = await ioBuilder.seal_with_output_policy(
    payload,
    feeAlgorithm,
    policy,
);

payload.free();
feeAlgorithm.free();
policy.free();

const inputs = await IOs.inputs();
const outputs = await IOs.outputs();

IOs.free();

const txBuilderSetIOs = await (await TransactionBuilder.new()).no_payload();

const txBuilderSetWitness = await txBuilderSetIOs.set_ios(
    inputs,
    outputs,
);

inputs.free();
outputs.free();

// Sign the transaction
// We need the genesis hash, the transaction id and the input account private key
const genesisHash =
    '6a702a181151b772ca0acbdc4d2870ed80c09b626b29fffc2e47abf2330ad0cd';

const privateKey = await PrivateKey.from_bech32(
    'ed25519e_sk1gz0ff4w444nwejap5shxrllypz5euswq6wn04fffzes02atw99xkd4jn838v3vrfg9eqt7f4sxjlsy0tdcmj0d2dqvwc8ztwgyfnwyszvjg32'
);

const hash = await Hash.from_hex(genesisHash);

const accountSpendingCounter = await SpendingCounter.zero();

const witness = await Witness.for_account(
    hash,
    await txBuilderSetWitness.get_auth_data_for_witness(),
    privateKey,
    accountSpendingCounter,
);

hash.free();
privateKey.free();
accountSpendingCounter.free();

const witnesses = await Witnesses.new();
await witnesses.add(witness);

const txBuilderSetAuthData = await txBuilderSetWitness.set_witnesses(witnesses);

witnesses.free();

const signedTx = await txBuilderSetAuthData.set_payload_auth(await PayloadAuthData.for_no_payload());

const readyToSendTx = await Fragment.from_transaction(signedTx);

signedTx.free();

const readyToSendTxBytes = await readyToSendTx.as_bytes();

readyToSendTx.free();

​console.log(buffer2hex(readyToSendTxBytes.buffer));
```
