let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

export function start() {
    wasm.start();
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_3.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

const BattleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_battle_free(ptr >>> 0, 1));

export class Battle {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BattleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_battle_free(ptr, 0);
    }
    constructor() {
        const ret = wasm.battle_new();
        this.__wbg_ptr = ret >>> 0;
        BattleFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {Position}
     */
    get_bot1() {
        const ret = wasm.battle_get_bot1(this.__wbg_ptr);
        return Position.__wrap(ret);
    }
    /**
     * @returns {Position}
     */
    get_bot2() {
        const ret = wasm.battle_get_bot2(this.__wbg_ptr);
        return Position.__wrap(ret);
    }
    /**
     * @returns {Position}
     */
    get_ball() {
        const ret = wasm.battle_get_ball(this.__wbg_ptr);
        return Position.__wrap(ret);
    }
    /**
     * @param {Uint8Array} wasm_bytes
     */
    add_bot(wasm_bytes) {
        var ptr0 = passArray8ToWasm0(wasm_bytes, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.battle_add_bot(this.__wbg_ptr, ptr0, len0, wasm_bytes);
    }
    init() {
        wasm.battle_init(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    update() {
        const ret = wasm.battle_update(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    execute() {
        const ret = wasm.battle_execute(this.__wbg_ptr);
        return ret;
    }
}

const PositionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_position_free(ptr >>> 0, 1));

export class Position {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Position.prototype);
        obj.__wbg_ptr = ptr;
        PositionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PositionFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_position_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_position_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_position_x(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_position_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_position_y(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get z() {
        const ret = wasm.__wbg_get_position_z(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set z(arg0) {
        wasm.__wbg_set_position_z(this.__wbg_ptr, arg0);
    }
}

const TrapFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_trap_free(ptr >>> 0, 1));
/**
 * A struct representing a Trap
 */
export class Trap {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TrapFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_trap_free(ptr, 0);
    }
    /**
     * A marker method to indicate that an object is an instance of the `Trap`
     * class.
     */
    static __wbg_wasmer_trap() {
        wasm.trap___wbg_wasmer_trap();
    }
}

export function __wbindgen_copy_to_typed_array(arg0, arg1, arg2) {
    new Uint8Array(arg2.buffer, arg2.byteOffset, arg2.byteLength).set(getArrayU8FromWasm0(arg0, arg1));
};

export function __wbindgen_string_new(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
};

export function __wbindgen_is_undefined(arg0) {
    const ret = arg0 === undefined;
    return ret;
};

export function __wbindgen_string_get(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbindgen_error_new(arg0, arg1) {
    const ret = new Error(getStringFromWasm0(arg0, arg1));
    return ret;
};

export function __wbindgen_number_new(arg0) {
    const ret = arg0;
    return ret;
};

export function __wbindgen_bigint_from_i64(arg0) {
    const ret = arg0;
    return ret;
};

export function __wbindgen_bigint_from_u128(arg0, arg1) {
    const ret = BigInt.asUintN(64, arg0) << BigInt(64) | BigInt.asUintN(64, arg1);
    return ret;
};

export function __wbindgen_number_get(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'number' ? obj : undefined;
    getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
};

export function __wbg_new_abda76e883ba8a5f() {
    const ret = new Error();
    return ret;
};

export function __wbg_stack_658279fe44541cf6(arg0, arg1) {
    const ret = arg1.stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_error_f851667af71bcfc6(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

export function __wbindgen_is_object(arg0) {
    const val = arg0;
    const ret = typeof(val) === 'object' && val !== null;
    return ret;
};

export function __wbindgen_is_function(arg0) {
    const ret = typeof(arg0) === 'function';
    return ret;
};

export function __wbg_instanceof_Global_e0304fee54c853fa(arg0) {
    let result;
    try {
        result = arg0 instanceof WebAssembly.Global;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_get_5419cf6b954aa11d(arg0, arg1) {
    const ret = arg0[arg1 >>> 0];
    return ret;
};

export function __wbg_length_f217bbbf7e8e4df4(arg0) {
    const ret = arg0.length;
    return ret;
};

export function __wbg_new_034f913e7636e987() {
    const ret = new Array();
    return ret;
};

export function __wbg_get_ef828680c64da212() { return handleError(function (arg0, arg1) {
    const ret = Reflect.get(arg0, arg1);
    return ret;
}, arguments) };

export function __wbg_call_a9ef466721e824f2() { return handleError(function (arg0, arg1) {
    const ret = arg0.call(arg1);
    return ret;
}, arguments) };

export function __wbg_new_e69b5f66fda8f13c() {
    const ret = new Object();
    return ret;
};

export function __wbg_push_36cf4d81d7da33d1(arg0, arg1) {
    const ret = arg0.push(arg1);
    return ret;
};

export function __wbg_instanceof_Error_a0af335a62107964(arg0) {
    let result;
    try {
        result = arg0 instanceof Error;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_message_00eebca8fa4dd7db(arg0) {
    const ret = arg0.message;
    return ret;
};

export function __wbg_instanceof_Function_f9b8826cda607bb2(arg0) {
    let result;
    try {
        result = arg0 instanceof Function;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_apply_c37b42ff44b0469d() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.apply(arg1, arg2);
    return ret;
}, arguments) };

export function __wbg_bind_8f736f9ed3d36fec(arg0, arg1, arg2, arg3) {
    const ret = arg0.bind(arg1, arg2, arg3);
    return ret;
};

export function __wbg_instanceof_Object_4bbac482eda9b711(arg0) {
    let result;
    try {
        result = arg0 instanceof Object;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_constructor_ad370c0159e195d8(arg0) {
    const ret = arg0.constructor;
    return ret;
};

export function __wbg_toString_aea130fe68d19e1a(arg0) {
    const ret = arg0.toString();
    return ret;
};

export function __wbg_buffer_ccaed51a635d8a2d(arg0) {
    const ret = arg0.buffer;
    return ret;
};

export function __wbg_newwithbyteoffsetandlength_7e3eb787208af730(arg0, arg1, arg2) {
    const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
    return ret;
};

export function __wbg_new_8fa1414101222712() { return handleError(function (arg0, arg1) {
    const ret = new WebAssembly.Instance(arg0, arg1);
    return ret;
}, arguments) };

export function __wbg_exports_b6cadece20c9cac3(arg0) {
    const ret = arg0.exports;
    return ret;
};

export function __wbg_new_8b6dd0c294e8b0d3() { return handleError(function (arg0) {
    const ret = new WebAssembly.Module(arg0);
    return ret;
}, arguments) };

export function __wbg_exports_92d8a4eff77fcbd2(arg0) {
    const ret = WebAssembly.Module.exports(arg0);
    return ret;
};

export function __wbg_imports_36e97e2e9477b793(arg0) {
    const ret = WebAssembly.Module.imports(arg0);
    return ret;
};

export function __wbg_instanceof_Table_2ab59c9e5730163c(arg0) {
    let result;
    try {
        result = arg0 instanceof WebAssembly.Table;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_get_7eb27976902923de() { return handleError(function (arg0, arg1) {
    const ret = arg0.get(arg1 >>> 0);
    return ret;
}, arguments) };

export function __wbg_instanceof_Memory_2544fea26af5efcc(arg0) {
    let result;
    try {
        result = arg0 instanceof WebAssembly.Memory;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_getPrototypeOf_1e48fb9e64cc1681() { return handleError(function (arg0) {
    const ret = Reflect.getPrototypeOf(arg0);
    return ret;
}, arguments) };

export function __wbg_set_e864d25d9b399c9f() { return handleError(function (arg0, arg1, arg2) {
    const ret = Reflect.set(arg0, arg1, arg2);
    return ret;
}, arguments) };

export function __wbindgen_debug_string(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_memory() {
    const ret = wasm.memory;
    return ret;
};

export function __wbindgen_function_table() {
    const ret = wasm.__wbindgen_export_2;
    return ret;
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_3;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

