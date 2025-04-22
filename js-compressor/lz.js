// lz77.js
const fs = require('fs');
const assert = require('assert');

const lz77 = {
    compress: (input, windowSize = 32, lookaheadSize = 16) => {
        if (!Buffer.isBuffer(input)) {
            throw new Error('Input must be a Buffer');
        }

        let compressed = [];
        let i = 0;

        while (i < input.length) {
            let matchOffset = 0;
            let matchLength = 0;

            for (let j = Math.max(0, i - windowSize); j < i; j++) {
                let length = 0;
                while (i + length < input.length && input[j + length] === input[i + length] && length < lookaheadSize) {
                    length++;
                }
                if (length > matchLength) {
                    matchOffset = i - j;
                    matchLength = length;
                }
            }

            if (matchLength > 0) {
                compressed.push(matchOffset, matchLength, input[i + matchLength]);
                i += matchLength + 1;
            } else {
                compressed.push(0, 0, input[i]);
                i++;
            }
        }
        return Buffer.from(compressed);
    },

    decompress: (input) => {
        if (!Buffer.isBuffer(input)) {
            throw new Error('Input must be a Buffer');
        }
        let decompressed = [];
        let i = 0;

        while (i < input.length) {
            const offset = input[i];
            const length = input[i + 1];
            const char = input[i + 2];

            if (i + 2 >= input.length) {
                throw new Error('Invalid compressed data: Missing elements');
            }

            if (offset === 0 && length === 0) {
                decompressed.push(char);
            } else {
                 if (offset > decompressed.length)
                    throw new Error("Offset is greater than decompressed length");
                for (let j = 0; j < length; j++) {
                    decompressed.push(decompressed[decompressed.length - offset]);
                }
                decompressed.push(char);
            }
            i += 3;
        }
        return Buffer.from(decompressed);
    },
};

if (process.argv[2] === 'test-lz77') {
    console.log('Running LZ77 tests...');
    const testInput = Buffer.from('AAABBBCCCCCDDDDE');
    const lz77Compressed = lz77.compress(testInput);
    const lz77Decompressed = lz77.decompress(lz77Compressed);
    assert.strictEqual(lz77Decompressed.toString(), testInput.toString(), 'LZ77 Test Failed');
    console.log('LZ77 Test Passed');
}
module.exports = lz77;
