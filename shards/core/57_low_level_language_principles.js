/**
 * SigmaOS Low-Level Language Principles Shard
 * Memory management, pointer arithmetic, bit manipulation, etc.
 */

class LowLevelMemory {
    constructor(size = 1024) {
        this.shardId = 'LowLevelMemory';
        this.buffer = new ArrayBuffer(size);
        this.uint8 = new Uint8Array(this.buffer);
        this.uint16 = new Uint16Array(this.buffer);
        this.uint32 = new Uint32Array(this.buffer);
        this.float32 = new Float32Array(this.buffer);
        this.float64 = new Float64Array(this.buffer);
        console.log(`Σ://LLP> LowLevelMemory initialized with ${size} bytes.`);
    }

    writeUint8(offset, value) {
        this.uint8[offset] = value;
    }

    readUint8(offset) {
        return this.uint8[offset];
    }

    writeUint16(offset, value) {
        this.uint16[offset >> 1] = value;
    }

    readUint16(offset) {
        return this.uint16[offset >> 1];
    }

    writeUint32(offset, value) {
        this.uint32[offset >> 2] = value;
    }

    readUint32(offset) {
        return this.uint32[offset >> 2];
    }

    writeFloat32(offset, value) {
        this.float32[offset >> 2] = value;
    }

    readFloat32(offset) {
        return this.float32[offset >> 2];
    }

    writeFloat64(offset, value) {
        this.float64[offset >> 3] = value;
    }

    readFloat64(offset) {
        return this.float64[offset >> 3];
    }

    getBuffer() {
        return this.buffer;
    }
}

class BitManipulation {
    static setBit(value, bit) {
        return value | (1 << bit);
    }

    static clearBit(value, bit) {
        return value & ~(1 << bit);
    }

    static toggleBit(value, bit) {
        return value ^ (1 << bit);
    }

    static checkBit(value, bit) {
        return (value & (1 << bit)) !== 0;
    }

    static leftShift(value, bits) {
        return value << bits;
    }

    static rightShift(value, bits) {
        return value >> bits;
    }

    static unsignedRightShift(value, bits) {
        return value >>> bits;
    }

    static bitwiseAnd(a, b) {
        return a & b;
    }

    static bitwiseOr(a, b) {
        return a | b;
    }

    static bitwiseXor(a, b) {
        return a ^ b;
    }

    static bitwiseNot(value) {
        return ~value;
    }

    static countSetBits(value) {
        let count = 0;
        while (value) {
            count += value & 1;
            value >>= 1;
        }
        return count;
    }
}

class SigmaLowLevelFramework {
    constructor() {
        this.shardId = 'S57_LowLevelLanguagePrinciples';
        this.memory = new LowLevelMemory();
        console.log(`Σ://INIT> ${this.shardId} Initializing low-level framework...`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            console.log(`Σ://LLP> ${this.shardId} Online. Low-level principles framework active.`);
        });
    }

    createMemory(size) {
        return new LowLevelMemory(size);
    }

    demonstrateBitManipulation() {
        console.log('Σ://LLP> Bit Manipulation Demo:');
        let val = 0b1010;
        console.log('Original:', val.toString(2));
        console.log('Set bit 2:', BitManipulation.setBit(val, 2).toString(2));
        console.log('Clear bit 3:', BitManipulation.clearBit(val, 3).toString(2));
        console.log('Toggle bit 1:', BitManipulation.toggleBit(val, 1).toString(2));
        console.log('Check bit 1:', BitManipulation.checkBit(val, 1));
    }
}

window.SigmaLowLevel = new SigmaLowLevelFramework();
