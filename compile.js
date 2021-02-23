const path = require('path');
const fs = require('fs');
const solc = require('solc');

// Compile contract
const contractPath = path.resolve(__dirname, 'ERC20PresetMinterBurner.sol');
const source = fs.readFileSync(contractPath, 'utf8');
const input = {
   language: 'Solidity',
   sources: {
      'ERC20PresetMinterBurner.sol': {
         content: source,
      },
   },
   settings: {
      outputSelection: {
         '*': {
            '*': ['*'],
         },
      },
   },
};
const tempFile = JSON.parse(solc.compile(JSON.stringify(input)));
const contractFile = tempFile.contracts['ERC20PresetMinterBurner.sol']['ERC20PresetMinterBurner'];
const byteCode = contractFile.evm.bytecode.object;

// Write bytecode to text file or return error 
fs.writeFile('bytecode.txt', byteCode, (err) => {
    if (err) throw err; 
})
module.exports.bytecode = byteCode;
module.exports.abi = contractFile.abi;