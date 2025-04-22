const fs = require('fs');
const rle = require('./rle.js');
const lz77 = require('./lz.js');

function runCli(args) {
    if (args.length < 5) {
        console.error('Usage: node index.js <compress|decompress> <input_file> <output_file> <algorithm> [--rle|--lz]');
        process.exit(1);
    }

    const mode = args[2];
    const inputFile = args[3];
    const outputFile = args[4];
    const algorithmType = args[5];

    if (mode !== 'compress' && mode !== 'decompress') {
        console.error('Error: Mode must be either "compress" or "decompress"');
        process.exit(1);
    }

    if (algorithmType !== '--rle' && algorithmType !== '--lz') {
        console.error('Error: Algorithm must be either "--rle" or "--lz"');
        process.exit(1);
    }

    try {
        const inputData = fs.readFileSync(inputFile);
        let outputData;

        if (mode === 'compress') {
            if (algorithmType === '--rle') {
                outputData = rle.compress(inputData);
            } else if (algorithmType === '--lz') {
                outputData = lz77.compress(inputData);
            }
        } else { // decompress
            if (algorithmType === '--rle') {
                outputData = rle.decompress(inputData);
            } else if (algorithmType === '--lz') {
                outputData = lz77.decompress(inputData);
            }
        }

        fs.writeFileSync(outputFile, outputData);
        console.log(`${mode}ed data written to ${outputFile}`);
    } catch (err) {
        console.error(`Error: ${err.message}`);
        process.exit(1);
    }
}

// Run the CLI
runCli(process.argv);
