
function clamp_host(i, min, max) {
  if (!Number.isInteger(i)) throw new TypeError(`must be an integer`);
  if (i < min || i > max) throw new RangeError(`must be between ${min} and ${max}`);
  return i;
}

let DATA_VIEW = new DataView(new ArrayBuffer());

function data_view(mem) {
  if (DATA_VIEW.buffer !== mem.buffer) DATA_VIEW = new DataView(mem.buffer);
  return DATA_VIEW;
}

function to_string(val) {
  if (typeof val === 'symbol') {
    throw new TypeError('symbols cannot be converted to strings');
  } else {
    // Calling `String` almost directly calls `ToString`, except that it also allows symbols,
    // which is why we have the symbol-rejecting branch above.
    //
    // Definition of `String`: https://tc39.es/ecma262/#sec-string-constructor-string-value
    return String(val);
  }
}

class Slab {
  constructor() {
    this.list = [];
    this.head = 0;
  }
  
  insert(val) {
    if (this.head >= this.list.length) {
      this.list.push({
        next: this.list.length + 1,
        val: undefined,
      });
    }
    const ret = this.head;
    const slot = this.list[ret];
    this.head = slot.next;
    slot.next = -1;
    slot.val = val;
    return ret;
  }
  
  get(idx) {
    if (idx >= this.list.length)
    throw new RangeError('handle index not valid');
    const slot = this.list[idx];
    if (slot.next === -1)
    return slot.val;
    throw new RangeError('handle index not valid');
  }
  
  remove(idx) {
    const ret = this.get(idx); // validate the slot
    const slot = this.list[idx];
    slot.val = undefined;
    slot.next = this.head;
    this.head = idx;
    return ret;
  }
}

module.exports = { clamp_host, data_view, to_string, Slab };