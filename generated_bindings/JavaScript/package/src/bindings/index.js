const fs = require("fs/promises");
const { Rustfft: _Rustfft } = require("./rustfft/rustfft.js");

class Bindings {
    constructor() {
        this._cache = {}
    }

    /** Lazily fetch and compile the WebAssembly module */
    async _getModule(filename) {
        if (filename in this._cache) {
            return this._cache[filename];
        }

        const wasm = await fs.readFile(`${__dirname}/${filename}`);
        this._cache[filename] = await WebAssembly.compile(wasm);
        return this._cache[filename];
    }
    async rustfft(options) {
        const wrapper = new _Rustfft();
        const module = await this._getModule("rustfft/rustfft.wasm");
        const imports = options?.imports || {};

        await wrapper.instantiate(module, imports);

        return wrapper;
    }
}

module.exports = { Bindings };