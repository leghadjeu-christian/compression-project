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
