const { clamp_host, data_view, to_string, Slab } = require('./intrinsics.js');
class Rustfft {
  constructor() {
    this._resource0_slab = new Slab();
    this._resource1_slab = new Slab();
    this._resource2_slab = new Slab();
  }
  addToImports(imports) {
    if (!("canonical_abi" in imports)) imports["canonical_abi"] = {};
    
    imports.canonical_abi['resource_drop_inner-fft'] = i => {
      this._resource0_slab.remove(i).drop();
    };
    imports.canonical_abi['resource_clone_inner-fft'] = i => {
      const obj = this._resource0_slab.get(i);
      return this._resource0_slab.insert(obj.clone())
    };
    imports.canonical_abi['resource_get_inner-fft'] = i => {
      return this._resource0_slab.get(i)._wasm_val;
    };
    imports.canonical_abi['resource_new_inner-fft'] = i => {
      const registry = this._registry0;
      return this._resource0_slab.insert(new InnerFft(i, this));
    };
    
    imports.canonical_abi['resource_drop_fft-planner'] = i => {
      this._resource1_slab.remove(i).drop();
    };
    imports.canonical_abi['resource_clone_fft-planner'] = i => {
      const obj = this._resource1_slab.get(i);
      return this._resource1_slab.insert(obj.clone())
    };
    imports.canonical_abi['resource_get_fft-planner'] = i => {
      return this._resource1_slab.get(i)._wasm_val;
    };
    imports.canonical_abi['resource_new_fft-planner'] = i => {
      const registry = this._registry1;
      return this._resource1_slab.insert(new FftPlanner(i, this));
    };
    
    imports.canonical_abi['resource_drop_algorithm'] = i => {
      this._resource2_slab.remove(i).drop();
    };
    imports.canonical_abi['resource_clone_algorithm'] = i => {
      const obj = this._resource2_slab.get(i);
      return this._resource2_slab.insert(obj.clone())
    };
    imports.canonical_abi['resource_get_algorithm'] = i => {
      return this._resource2_slab.get(i)._wasm_val;
    };
    imports.canonical_abi['resource_new_algorithm'] = i => {
      const registry = this._registry2;
      return this._resource2_slab.insert(new Algorithm(i, this));
    };
  }
  
  async instantiate(module, imports) {
    imports = imports || {};
    this.addToImports(imports);
    
    if (module instanceof WebAssembly.Instance) {
      this.instance = module;
    } else if (module instanceof WebAssembly.Module) {
      this.instance = await WebAssembly.instantiate(module, imports);
    } else if (module instanceof ArrayBuffer || module instanceof Uint8Array) {
      const { instance } = await WebAssembly.instantiate(module, imports);
      this.instance = instance;
    } else {
      const { instance } = await WebAssembly.instantiateStreaming(module, imports);
      this.instance = instance;
    }
    this._exports = this.instance.exports;
    this._registry0 = new FinalizationRegistry(this._exports['canonical_abi_drop_inner-fft']);
    this._registry1 = new FinalizationRegistry(this._exports['canonical_abi_drop_fft-planner']);
    this._registry2 = new FinalizationRegistry(this._exports['canonical_abi_drop_algorithm']);
  }
}

class InnerFft {
  constructor(wasm_val, obj) {
    this._wasm_val = wasm_val;
    this._obj = obj;
    this._refcnt = 1;
    obj._registry0.register(this, wasm_val, this);
  }
  
  clone() {
    this._refcnt += 1;
    return this;
  }
  
  drop() {
    this._refcnt -= 1;
    if (this._refcnt !== 0)
    return;
    this._obj._registry0.unregister(this);
    const dtor = this._obj._exports['canonical_abi_drop_inner-fft'];
    const wasm_val = this._wasm_val;
    delete this._obj;
    delete this._refcnt;
    delete this._wasm_val;
    dtor(wasm_val);
  }
  process(arg1) {
    const memory = this._obj._exports.memory;
    const realloc = this._obj._exports["canonical_abi_realloc"];
    const obj0 = this;
    const vec2 = arg1;
    const len2 = vec2.length;
    const result2 = realloc(0, 0, 8, len2 * 16);
    for (let i = 0; i < vec2.length; i++) {
      const e = vec2[i];
      const base = result2 + i * 16;
      const {re: v1_0, im: v1_1 } = e;
      data_view(memory).setFloat64(base + 0, +v1_0, true);
      data_view(memory).setFloat64(base + 8, +v1_1, true);
    }
    this._obj._exports['inner-fft::process'](this._obj._resource0_slab.insert(obj0.clone()), result2, len2);
    return undefined;
  }
}

class FftPlanner {
  constructor(wasm_val, obj) {
    this._wasm_val = wasm_val;
    this._obj = obj;
    this._refcnt = 1;
    obj._registry1.register(this, wasm_val, this);
  }
  
  clone() {
    this._refcnt += 1;
    return this;
  }
  
  drop() {
    this._refcnt -= 1;
    if (this._refcnt !== 0)
    return;
    this._obj._registry1.unregister(this);
    const dtor = this._obj._exports['canonical_abi_drop_fft-planner'];
    const wasm_val = this._wasm_val;
    delete this._obj;
    delete this._refcnt;
    delete this._wasm_val;
    dtor(wasm_val);
  }
  static new(rustfft) {
    const ret = rustfft._exports['fft-planner::new']();
    return rustfft._resource1_slab.remove(ret);
  }
  planFft(arg1, arg2) {
    const obj0 = this;
    const val1 = to_string(arg2);
    let enum1;
    switch (val1) {
      case "forward": {
        enum1 = 0;
        break;
      }
      case "inverse": {
        enum1 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val1}" is not one of the cases of fft-direction`);
      }
    }
    const ret = this._obj._exports['fft-planner::plan-fft'](this._obj._resource1_slab.insert(obj0.clone()), clamp_host(arg1, 0, 4294967295), enum1);
    return this._obj._resource0_slab.remove(ret);
  }
  planFftForward(arg1) {
    const obj0 = this;
    const ret = this._obj._exports['fft-planner::plan-fft-forward'](this._obj._resource1_slab.insert(obj0.clone()), clamp_host(arg1, 0, 4294967295));
    return this._obj._resource0_slab.remove(ret);
  }
  planFftInverse(arg1) {
    const obj0 = this;
    const ret = this._obj._exports['fft-planner::plan-fft-inverse'](this._obj._resource1_slab.insert(obj0.clone()), clamp_host(arg1, 0, 4294967295));
    return this._obj._resource0_slab.remove(ret);
  }
}

class Algorithm {
  constructor(wasm_val, obj) {
    this._wasm_val = wasm_val;
    this._obj = obj;
    this._refcnt = 1;
    obj._registry2.register(this, wasm_val, this);
  }
  
  clone() {
    this._refcnt += 1;
    return this;
  }
  
  drop() {
    this._refcnt -= 1;
    if (this._refcnt !== 0)
    return;
    this._obj._registry2.unregister(this);
    const dtor = this._obj._exports['canonical_abi_drop_algorithm'];
    const wasm_val = this._wasm_val;
    delete this._obj;
    delete this._refcnt;
    delete this._wasm_val;
    dtor(wasm_val);
  }
  static newBluesteinsAlgorithm(rustfft, arg0, arg1) {
    const obj0 = arg1;
    if (!(obj0 instanceof InnerFft)) throw new TypeError('expected instance of InnerFft');
    const ret = rustfft._exports['algorithm::new-bluesteins-algorithm'](clamp_host(arg0, 0, 4294967295), rustfft._resource0_slab.insert(obj0.clone()));
    return rustfft._resource2_slab.remove(ret);
  }
  static newDft(rustfft, arg0, arg1) {
    const val0 = to_string(arg1);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-dft'](clamp_host(arg0, 0, 4294967295), enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newGoodThomasAlgorithm(rustfft, arg0, arg1) {
    const obj0 = arg0;
    if (!(obj0 instanceof InnerFft)) throw new TypeError('expected instance of InnerFft');
    const obj1 = arg1;
    if (!(obj1 instanceof InnerFft)) throw new TypeError('expected instance of InnerFft');
    const ret = rustfft._exports['algorithm::new-good-thomas-algorithm'](rustfft._resource0_slab.insert(obj0.clone()), rustfft._resource0_slab.insert(obj1.clone()));
    return rustfft._resource2_slab.remove(ret);
  }
  static newGoodThomasAlgorithmSmall(rustfft, arg0, arg1) {
    const obj0 = arg0;
    if (!(obj0 instanceof InnerFft)) throw new TypeError('expected instance of InnerFft');
    const obj1 = arg1;
    if (!(obj1 instanceof InnerFft)) throw new TypeError('expected instance of InnerFft');
    const ret = rustfft._exports['algorithm::new-good-thomas-algorithm-small'](rustfft._resource0_slab.insert(obj0.clone()), rustfft._resource0_slab.insert(obj1.clone()));
    return rustfft._resource2_slab.remove(ret);
  }
  static newMixedRadix(rustfft, arg0, arg1) {
    const obj0 = arg0;
    if (!(obj0 instanceof InnerFft)) throw new TypeError('expected instance of InnerFft');
    const obj1 = arg1;
    if (!(obj1 instanceof InnerFft)) throw new TypeError('expected instance of InnerFft');
    const ret = rustfft._exports['algorithm::new-mixed-radix'](rustfft._resource0_slab.insert(obj0.clone()), rustfft._resource0_slab.insert(obj1.clone()));
    return rustfft._resource2_slab.remove(ret);
  }
  static newMixedRadixSmall(rustfft, arg0, arg1) {
    const obj0 = arg0;
    if (!(obj0 instanceof InnerFft)) throw new TypeError('expected instance of InnerFft');
    const obj1 = arg1;
    if (!(obj1 instanceof InnerFft)) throw new TypeError('expected instance of InnerFft');
    const ret = rustfft._exports['algorithm::new-mixed-radix-small'](rustfft._resource0_slab.insert(obj0.clone()), rustfft._resource0_slab.insert(obj1.clone()));
    return rustfft._resource2_slab.remove(ret);
  }
  static newRadersAlgorithm(rustfft, arg0) {
    const obj0 = arg0;
    if (!(obj0 instanceof InnerFft)) throw new TypeError('expected instance of InnerFft');
    const ret = rustfft._exports['algorithm::new-raders-algorithm'](rustfft._resource0_slab.insert(obj0.clone()));
    return rustfft._resource2_slab.remove(ret);
  }
  static newRadix3(rustfft, arg0, arg1) {
    const val0 = to_string(arg1);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-radix3'](clamp_host(arg0, 0, 4294967295), enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newRadix4(rustfft, arg0, arg1) {
    const val0 = to_string(arg1);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-radix4'](clamp_host(arg0, 0, 4294967295), enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly1(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly1'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly2(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly2'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly3(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly3'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly4(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly4'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly5(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly5'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly6(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly6'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly7(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly7'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly8(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly8'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly9(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly9'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly11(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly11'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly13(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly13'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly16(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly16'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly17(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly17'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly19(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly19'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly23(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly23'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly27(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly27'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly29(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly29'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly31(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly31'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  static newButterfly32(rustfft, arg0) {
    const val0 = to_string(arg0);
    let enum0;
    switch (val0) {
      case "forward": {
        enum0 = 0;
        break;
      }
      case "inverse": {
        enum0 = 1;
        break;
      }
      default: {
        throw new TypeError(`"${val0}" is not one of the cases of fft-direction`);
      }
    }
    const ret = rustfft._exports['algorithm::new-butterfly32'](enum0);
    return rustfft._resource2_slab.remove(ret);
  }
  len() {
    const obj0 = this;
    const ret = this._obj._exports['algorithm::len'](this._obj._resource2_slab.insert(obj0.clone()));
    return ret >>> 0;
  }
  fftDirection() {
    const obj0 = this;
    const ret = this._obj._exports['algorithm::fft-direction'](this._obj._resource2_slab.insert(obj0.clone()));
    let enum1;
    switch (ret) {
      case 0: {
        enum1 = "forward";
        break;
      }
      case 1: {
        enum1 = "inverse";
        break;
      }
      default: {
        throw new RangeError("invalid discriminant specified for FftDirection");
      }
    }
    return enum1;
  }
  compute(arg1) {
    const memory = this._obj._exports.memory;
    const realloc = this._obj._exports["canonical_abi_realloc"];
    const free = this._obj._exports["canonical_abi_free"];
    const obj0 = this;
    const vec2 = arg1;
    const len2 = vec2.length;
    const result2 = realloc(0, 0, 8, len2 * 16);
    for (let i = 0; i < vec2.length; i++) {
      const e = vec2[i];
      const base = result2 + i * 16;
      const {re: v1_0, im: v1_1 } = e;
      data_view(memory).setFloat64(base + 0, +v1_0, true);
      data_view(memory).setFloat64(base + 8, +v1_1, true);
    }
    const ret = this._obj._exports['algorithm::compute'](this._obj._resource2_slab.insert(obj0.clone()), result2, len2);
    const len3 = data_view(memory).getInt32(ret + 4, true);
    const base3 = data_view(memory).getInt32(ret + 0, true);
    const result3 = [];
    for (let i = 0; i < len3; i++) {
      const base = base3 + i * 16;
      result3.push({
        re: data_view(memory).getFloat64(base + 0, true),
        im: data_view(memory).getFloat64(base + 8, true),
      });
    }
    free(base3, len3 * 16, 8);
    return result3;
  }
}

module.exports = { Rustfft, InnerFft, FftPlanner, Algorithm };
