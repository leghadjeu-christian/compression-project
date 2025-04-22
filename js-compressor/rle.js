// // Javascript program to implement run length encoding
// 	// function printRLE(str)
// 	// {
// 	// 	let n = str.length;
// 	// 	for (let i = 0; i < n; i++)
// 	// 	{
// 	// 		// Count occurrences of current character
// 	// 		let count = 1;
// 	// 		while (i < n - 1 && str[i] == str[i+1])
// 	// 		{
// 	// 			count++;
// 	// 			i++;
// 	// 		}
			
// 	// 		// Print character and its count
// 	// 		document.write(str[i]);
// 	// 		document.write(count);
// 	// 	}
// 	// }
	
// 	// let str = "wwwwaaadexxxxxxywww";
// 	// printRLE(str);
	
// 	// This code is contributed by rag2127


// 	const fs = require('fs');
// const assert = require('assert');

// // RLE Compression and Decompression
// const rle = {
//     compress: (input) => {
//         if (!Buffer.isBuffer(input)) {
//             throw new Error('Input must be a Buffer');
//         }

//         if (input.length === 0) {
//             return Buffer.alloc(0); // Handle empty input
//         }

//         let compressed = [];
//         let count = 1;
//         let prev = input[0];

//         for (let i = 1; i < input.length; i++) {
//             if (input[i] === prev && count < 255) { // Limit count to 255
//                 count++;
//             } else {
//                 compressed.push(count, prev);
//                 count = 1;
//                 prev = input[i];
//             }
//         }
//         compressed.push(count, prev); // Add the last sequence
//         return Buffer.from(compressed);
//     },

//     decompress: (input) => {
//         if (!Buffer.isBuffer(input)) {
//             throw new Error('Input must be a Buffer');
//         }

//         if (input.length === 0) {
//           return Buffer.alloc(0);
//         }

//         let decompressed = [];
//         for (let i = 0; i < input.length; i += 2) {
//             const count = input[i];
//             const value = input[i + 1];
//             if (i + 1 >= input.length) {
//                 throw new Error('Invalid compressed data: Missing value');
//             }
//             for (let j = 0; j < count; j++) {
//                 decompressed.push(value);
//             }
//         }
//         return Buffer.from(decompressed);
//     },
// };

// // LZ77 Compression and Decompression
// const lz77 = {
//     compress: (input, windowSize = 32, lookaheadSize = 16) => {
//         if (!Buffer.isBuffer(input)) {
//             throw new Error('Input must be a Buffer');
//         }

//         let compressed = [];
//         let i = 0;

//         while (i < input.length) {
//             let matchOffset = 0;
//             let matchLength = 0;

//             for (let j = Math.max(0, i - windowSize); j < i; j++) {
//                 let length = 0;
//                 while (i + length < input.length && input[j + length] === input[i + length] && length < lookaheadSize) {
//                     length++;
//                 }
//                 if (length > matchLength) {
//                     matchOffset = i - j;
//                     matchLength = length;
//                 }
//             }

//             if (matchLength > 0) {
//                 compressed.push(matchOffset, matchLength, input[i + matchLength]);
//                 i += matchLength + 1;
//             } else {
//                 compressed.push(0, 0, input[i]);
//                 i++;
//             }
//         }
//         return Buffer.from(compressed);
//     },

//     decompress: (input) => {
//         if (!Buffer.isBuffer(input)) {
//             throw new Error('Input must be a Buffer');
//         }
//         let decompressed = [];
//         let i = 0;

//         while (i < input.length) {
//             const offset = input[i];
//             const length = input[i + 1];
//             const char = input[i + 2];

//              if (i + 2 >= input.length) {
//                 throw new Error('Invalid compressed data: Missing elements');
//             }

//             if (offset === 0 && length === 0) {
//                 decompressed.push(char);
//             } else {
//                 if (offset > decompressed.length)
//                     throw new Error("Offset is greater than decompressed length");
//                 for (let j = 0; j < length; j++) {
//                     decompressed.push(decompressed[decompressed.length - offset]);
//                 }
//                 decompressed.push(char);
//             }
//             i += 3;
//         }
//         return Buffer.from(decompressed);
//     },
// };

// // CLI Interface
// function runCli(args) {
//     if (args.length < 5) {
//         console.error('Usage: node script.js <compress|decompress> <input_file> <output_file> <algorithm> [--rle|--lz]');
//         process.exit(1);
//     }

//     const mode = args[2];
//     const inputFile = args[3];
//     const outputFile = args[4];
//     const algorithmType = args[5];

//     if (mode !== 'compress' && mode !== 'decompress') {
//         console.error('Error: Mode must be either "compress" or "decompress"');
//         process.exit(1);
//     }

//     if (algorithmType !== '--rle' && algorithmType !== '--lz') {
//         console.error('Error: Algorithm must be either "--rle" or "--lz"');
//         process.exit(1);
//     }

//     try {
//         const inputData = fs.readFileSync(inputFile);
//         let outputData;

//         if (mode === 'compress') {
//             if (algorithmType === '--rle') {
//                 outputData = rle.compress(inputData);
//             } else if (algorithmType === '--lz') {
//                 outputData = lz77.compress(inputData);
//             }
//         } else { // decompress
//             if (algorithmType === '--rle') {
//                 outputData = rle.decompress(inputData);
//             } else if (algorithmType === '--lz') {
//                 outputData = lz77.decompress(inputData);
//             }
//         }

//         fs.writeFileSync(outputFile, outputData);
//         console.log(`${mode}ed data written to ${outputFile}`);
//     } catch (err) {
//         console.error(`Error: ${err.message}`);
//         process.exit(1);
//     }
// }

// // Run tests if the script is run directly (e.g., `node script.js test`)
// if (process.argv[2] === 'test') {
//     console.log('Running tests...');
//     const testInput = Buffer.from('AAABBBCCCCCDDDDE');
//     const rleCompressed = rle.compress(testInput);
//     const rleDecompressed = rle.decompress(rleCompressed);

//     assert.strictEqual(rleDecompressed.toString(), testInput.toString(), 'RLE Test Failed');
//     console.log('RLE Test Passed');

//     const lz77Compressed = lz77.compress(testInput);
//     const lz77Decompressed = lz77.decompress(lz77Compressed);
//     assert.strictEqual(lz77Decompressed.toString(), testInput.toString(), 'LZ77 Test Failed');
//     console.log('LZ77 Test Passed');
//     console.log('All tests passed!');
// } else if (process.argv[2] !== 'test'){
//     // Run the CLI if not testing and not undefined
//     runCli(process.argv);
// }

// //Make sure to run the tests.
// module.exports = {
//     rle,
//     lz77
// };



// // rle.js
const fs = require('fs');
const assert = require('assert');

const rle = {
    compress: (input) => {
        if (!Buffer.isBuffer(input)) {
            throw new Error('Input must be a Buffer');
        }

        if (input.length === 0) {
            return Buffer.alloc(0); // Handle empty input
        }

        let compressed = [];
        let count = 1;
        let prev = input[0];

        for (let i = 1; i < input.length; i++) {
            if (input[i] === prev && count < 255) { // Limit count to 255
                count++;
            } else {
                compressed.push(count, prev);
                count = 1;
                prev = input[i];
            }
        }
        compressed.push(count, prev); // Add the last sequence
        return Buffer.from(compressed);
    },

    decompress: (input) => {
        if (!Buffer.isBuffer(input)) {
            throw new Error('Input must be a Buffer');
        }

        if (input.length === 0) {
          return Buffer.alloc(0);
        }

        let decompressed = [];
        for (let i = 0; i < input.length; i += 2) {
            const count = input[i];
            const value = input[i + 1];
             if (i + 1 >= input.length) {
                throw new Error('Invalid compressed data: Missing value');
            }
            for (let j = 0; j < count; j++) {
                decompressed.push(value);
            }
        }
        return Buffer.from(decompressed);
    },
};

if (process.argv[2] === 'test-rle') {
    console.log('Running RLE tests...');
    const testInput = Buffer.from('AAABBBCCCCCDDDDE');
    const rleCompressed = rle.compress(testInput);
    const rleDecompressed = rle.decompress(rleCompressed);
    assert.strictEqual(rleDecompressed.toString(), testInput.toString(), 'RLE Test Failed');
    console.log('RLE Test Passed');
}

module.exports = rle;
