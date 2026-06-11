import { Interpreter } from './snippets/dioxus-interpreter-js-459fb15b86d869f7/src/interpreter.js';

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_AppendChildren_1de01c7342c851c5: function(arg0, arg1) {
            arg0.AppendChildren(arg1 >>> 0);
        },
        __wbg_CreateElementNs_3d7090ead2b34696: function(arg0, arg1, arg2, arg3, arg4, arg5) {
            var v0 = getCachedStringFromWasm0(arg1, arg2);
            var v1 = getCachedStringFromWasm0(arg4, arg5);
            arg0.CreateElementNs(v0, BigInt.asUintN(64, arg3), v1);
        },
        __wbg_CreateElement_b1466c737b240cbc: function(arg0, arg1, arg2, arg3) {
            var v0 = getCachedStringFromWasm0(arg1, arg2);
            arg0.CreateElement(v0, BigInt.asUintN(64, arg3));
        },
        __wbg_CreatePlaceholder_2238d697d0b876c0: function(arg0, arg1) {
            arg0.CreatePlaceholder(BigInt.asUintN(64, arg1));
        },
        __wbg_CreateTextNode_3de7809ee3dd8c20: function(arg0, arg1, arg2) {
            arg0.CreateTextNode(arg1, BigInt.asUintN(64, arg2));
        },
        __wbg_InsertAfter_c4050dc8e349fb7c: function(arg0, arg1, arg2) {
            arg0.InsertAfter(BigInt.asUintN(64, arg1), arg2 >>> 0);
        },
        __wbg_InsertBefore_c31965bdbec877b4: function(arg0, arg1, arg2) {
            arg0.InsertBefore(BigInt.asUintN(64, arg1), arg2 >>> 0);
        },
        __wbg_NewEventListener_764a7f7f33d6208b: function(arg0, arg1, arg2, arg3, arg4) {
            var v0 = getCachedStringFromWasm0(arg1, arg2);
            arg0.NewEventListener(v0, BigInt.asUintN(64, arg3), arg4);
        },
        __wbg_PopRoot_015aedfb36752c07: function(arg0) {
            arg0.PopRoot();
        },
        __wbg_PushRoot_ebbd35dbfe325099: function(arg0, arg1) {
            arg0.PushRoot(BigInt.asUintN(64, arg1));
        },
        __wbg_RemoveAttribute_0859330eb8fcb627: function(arg0, arg1, arg2, arg3, arg4, arg5) {
            var v0 = getCachedStringFromWasm0(arg2, arg3);
            var v1 = getCachedStringFromWasm0(arg4, arg5);
            arg0.RemoveAttribute(BigInt.asUintN(64, arg1), v0, v1);
        },
        __wbg_RemoveEventListener_395b3f56d597dcc7: function(arg0, arg1, arg2, arg3) {
            var v0 = getCachedStringFromWasm0(arg2, arg3);
            arg0.RemoveEventListener(BigInt.asUintN(64, arg1), v0);
        },
        __wbg_Remove_ff403fc244e6b133: function(arg0, arg1) {
            arg0.Remove(BigInt.asUintN(64, arg1));
        },
        __wbg_ReplaceWith_acdff0ee27fc2df1: function(arg0, arg1, arg2) {
            arg0.ReplaceWith(BigInt.asUintN(64, arg1), arg2 >>> 0);
        },
        __wbg_SetAttribute_59129bb18b1554bb: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
            var v0 = getCachedStringFromWasm0(arg2, arg3);
            var v1 = getCachedStringFromWasm0(arg5, arg6);
            arg0.SetAttribute(BigInt.asUintN(64, arg1), v0, arg4, v1);
        },
        __wbg_SetNode_51e3a0ad699897da: function(arg0, arg1, arg2) {
            arg0.SetNode(arg1 >>> 0, arg2);
        },
        __wbg_SetText_334b39f218f11559: function(arg0, arg1, arg2) {
            arg0.SetText(BigInt.asUintN(64, arg1), arg2);
        },
        __wbg___wbindgen_debug_string_edece8177ad01481: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_is_function_5cd60d5cf78b4eef: function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_object_b4593df85baada48: function(arg0) {
            const val = arg0;
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        },
        __wbg___wbindgen_is_string_dde0fd9020db4434: function(arg0) {
            const ret = typeof(arg0) === 'string';
            return ret;
        },
        __wbg___wbindgen_is_undefined_35bb9f4c7fd651d5: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_memory_9544558992fc5400: function() {
            const ret = wasm.memory;
            return ret;
        },
        __wbg___wbindgen_throw_9c31b086c2b26051: function(arg0, arg1) {
            var v0 = getCachedStringFromWasm0(arg0, arg1);
            throw new Error(v0);
        },
        __wbg__wbg_cb_unref_3fa391f3fcdb55f8: function(arg0) {
            arg0._wbg_cb_unref();
        },
        __wbg_altKey_05f1250da41a9431: function(arg0) {
            const ret = arg0.altKey;
            return ret;
        },
        __wbg_altKey_2516b7969270625e: function(arg0) {
            const ret = arg0.altKey;
            return ret;
        },
        __wbg_altKey_4c746e2bfdeccc44: function(arg0) {
            const ret = arg0.altKey;
            return ret;
        },
        __wbg_animationName_d8039a24ac38fdad: function(arg0, arg1) {
            const ret = arg1.animationName;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_buffer_20eb5c4c035a2b02: function(arg0) {
            const ret = arg0.buffer;
            return ret;
        },
        __wbg_button_131ad2458f5f990b: function(arg0) {
            const ret = arg0.button;
            return ret;
        },
        __wbg_buttons_42291c85c3cc8132: function(arg0) {
            const ret = arg0.buttons;
            return ret;
        },
        __wbg_call_e1bb38bab8876690: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments); },
        __wbg_call_f9b20945c2f1bfe3: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_charCode_fb85072f582d993e: function(arg0) {
            const ret = arg0.charCode;
            return ret;
        },
        __wbg_checked_1d30124949a5c47d: function(arg0) {
            const ret = arg0.checked;
            return ret;
        },
        __wbg_childNodes_0f838d099d226773: function(arg0) {
            const ret = arg0.childNodes;
            return ret;
        },
        __wbg_clearTimeout_01406e55473040f6: function(arg0) {
            const ret = clearTimeout(arg0);
            return ret;
        },
        __wbg_clientX_e52a3f884d43ad5e: function(arg0) {
            const ret = arg0.clientX;
            return ret;
        },
        __wbg_clientY_341c21784d6eccac: function(arg0) {
            const ret = arg0.clientY;
            return ret;
        },
        __wbg_createElement_c7448bdb0467f8a9: function() { return handleError(function (arg0, arg1, arg2) {
            var v0 = getCachedStringFromWasm0(arg1, arg2);
            const ret = arg0.createElement(v0);
            return ret;
        }, arguments); },
        __wbg_crypto_f43a4c86a0c7157a: function(arg0) {
            const ret = arg0.crypto;
            return ret;
        },
        __wbg_ctrlKey_41fa7a46c82d4503: function(arg0) {
            const ret = arg0.ctrlKey;
            return ret;
        },
        __wbg_ctrlKey_68ed66f78d384194: function(arg0) {
            const ret = arg0.ctrlKey;
            return ret;
        },
        __wbg_ctrlKey_715518d18976699b: function(arg0) {
            const ret = arg0.ctrlKey;
            return ret;
        },
        __wbg_data_c57adadc17eca554: function(arg0, arg1) {
            const ret = arg1.data;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_deltaMode_25c7816c6235d2e5: function(arg0) {
            const ret = arg0.deltaMode;
            return ret;
        },
        __wbg_deltaX_1739faef85e3a0a3: function(arg0) {
            const ret = arg0.deltaX;
            return ret;
        },
        __wbg_deltaY_f37d2baa43c4f1ae: function(arg0) {
            const ret = arg0.deltaY;
            return ret;
        },
        __wbg_deltaZ_54b884ad94487d43: function(arg0) {
            const ret = arg0.deltaZ;
            return ret;
        },
        __wbg_document_155c63045537f27d: function(arg0) {
            const ret = arg0.document;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_elapsedTime_381ccb6198f8f708: function(arg0) {
            const ret = arg0.elapsedTime;
            return ret;
        },
        __wbg_elapsedTime_59851463f9076e4e: function(arg0) {
            const ret = arg0.elapsedTime;
            return ret;
        },
        __wbg_elements_1fe41bcc2b832a14: function(arg0) {
            const ret = arg0.elements;
            return ret;
        },
        __wbg_error_a6fa202b58aa1cd3: function(arg0, arg1) {
            var v0 = getCachedStringFromWasm0(arg0, arg1);
            if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1, 1); }
            console.error(v0);
        },
        __wbg_getAttribute_4940bb22c8b6ee1f: function(arg0, arg1, arg2, arg3) {
            var v0 = getCachedStringFromWasm0(arg2, arg3);
            const ret = arg1.getAttribute(v0);
            var ptr2 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len2 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len2, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr2, true);
        },
        __wbg_getElementById_3abe0ed2fa34bb0f: function(arg0, arg1, arg2) {
            var v0 = getCachedStringFromWasm0(arg1, arg2);
            const ret = arg0.getElementById(v0);
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_getRandomValues_6b4be600718ac5c0: function() { return handleError(function (arg0, arg1) {
            arg0.getRandomValues(arg1);
        }, arguments); },
        __wbg_get_5a63ecffc3d6ddc9: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_globalThis_6d5c691cd8302b28: function() { return handleError(function () {
            const ret = globalThis.globalThis;
            return ret;
        }, arguments); },
        __wbg_global_2621fef02ebf3494: function() { return handleError(function () {
            const ret = global.global;
            return ret;
        }, arguments); },
        __wbg_hasOwnProperty_074ad15de74af30c: function(arg0, arg1) {
            const ret = arg0.hasOwnProperty(arg1);
            return ret;
        },
        __wbg_height_f8b3b2a3fa8bae23: function(arg0) {
            const ret = arg0.height;
            return ret;
        },
        __wbg_instanceof_CompositionEvent_ed20cc67a53229b1: function(arg0) {
            let result;
            try {
                result = arg0 instanceof CompositionEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Element_ed2b1c4b3019d60f: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Element;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlElement_ac213f3290575467: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlFormElement_618ecf8cc9696d82: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLFormElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlInputElement_6fe0049d58f4cf0d: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLInputElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlSelectElement_d23f4a79e52d5b3d: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLSelectElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlTextAreaElement_1003891cb7873955: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLTextAreaElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_IdleDeadline_c4df263005d45201: function(arg0) {
            let result;
            try {
                result = arg0 instanceof IdleDeadline;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Node_65f1babdf8f0092c: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Node;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Object_8fa16bee04ace91c: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Object;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Text_2c3cff0dbe48b4b3: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Text;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Window_f9d7a02094d39fa0: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_isPrimary_5149994ff1fc35db: function(arg0) {
            const ret = arg0.isPrimary;
            return ret;
        },
        __wbg_item_a48f67f06ce9567d: function(arg0, arg1) {
            const ret = arg0.item(arg1 >>> 0);
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_keyCode_4b95680820a099ea: function(arg0) {
            const ret = arg0.keyCode;
            return ret;
        },
        __wbg_key_6bef8a8ae87b8c2b: function(arg0, arg1) {
            const ret = arg1.key;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_length_563f2044099d4a23: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_length_8eb681c81233d914: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_location_db8e7bc4d7fb01a0: function(arg0) {
            const ret = arg0.location;
            return ret;
        },
        __wbg_metaKey_a1fa9da163ab3ddc: function(arg0) {
            const ret = arg0.metaKey;
            return ret;
        },
        __wbg_metaKey_b19ba71cc11b0330: function(arg0) {
            const ret = arg0.metaKey;
            return ret;
        },
        __wbg_metaKey_d1f778a31ed9dbf9: function(arg0) {
            const ret = arg0.metaKey;
            return ret;
        },
        __wbg_msCrypto_9ff276969b3ab68c: function(arg0) {
            const ret = arg0.msCrypto;
            return ret;
        },
        __wbg_new_227d7c05414eb861: function() {
            const ret = new Error();
            return ret;
        },
        __wbg_new_5a8720951af575e8: function(arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        },
        __wbg_new_f7fb8728567e0ecf: function(arg0) {
            const ret = new Interpreter(arg0);
            return ret;
        },
        __wbg_new_no_args_af7ce56c0a943483: function(arg0, arg1) {
            var v0 = getCachedStringFromWasm0(arg0, arg1);
            const ret = new Function(v0);
            return ret;
        },
        __wbg_new_with_length_1d0fa3912516a1a7: function(arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return ret;
        },
        __wbg_node_112d6e6b60d5ad71: function(arg0) {
            const ret = arg0.node;
            return ret;
        },
        __wbg_pageX_c6ceaea3b38e5746: function(arg0) {
            const ret = arg0.pageX;
            return ret;
        },
        __wbg_pageY_8146de7012ccd001: function(arg0) {
            const ret = arg0.pageY;
            return ret;
        },
        __wbg_parentElement_fb0c291ae6f8bd7e: function(arg0) {
            const ret = arg0.parentElement;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_pointerId_8b37acc491187c93: function(arg0) {
            const ret = arg0.pointerId;
            return ret;
        },
        __wbg_pointerType_35bc74c0197ab173: function(arg0, arg1) {
            const ret = arg1.pointerType;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_pressure_c176deb5f6501f79: function(arg0) {
            const ret = arg0.pressure;
            return ret;
        },
        __wbg_preventDefault_c45668f0a6306d5b: function(arg0) {
            arg0.preventDefault();
        },
        __wbg_process_71594fb16469954f: function(arg0) {
            const ret = arg0.process;
            return ret;
        },
        __wbg_propertyName_3ff77ff4c9aa3c6a: function(arg0, arg1) {
            const ret = arg1.propertyName;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_pseudoElement_0112a52dd5c80fe0: function(arg0, arg1) {
            const ret = arg1.pseudoElement;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_pseudoElement_c0fa82d947e308b1: function(arg0, arg1) {
            const ret = arg1.pseudoElement;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_randomFillSync_57af018321fe5242: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.randomFillSync(getArrayU8FromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_repeat_8ce42c4503cd467b: function(arg0) {
            const ret = arg0.repeat;
            return ret;
        },
        __wbg_requestAnimationFrame_cd7967649bf208f8: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.requestAnimationFrame(arg1);
            return ret;
        }, arguments); },
        __wbg_requestIdleCallback_ef030b89f73b34d8: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.requestIdleCallback(arg1);
            return ret;
        }, arguments); },
        __wbg_require_fce86418f1f47c91: function() { return handleError(function () {
            const ret = module.require;
            return ret;
        }, arguments); },
        __wbg_resolve_dd61af963a7292bf: function(arg0) {
            const ret = Promise.resolve(arg0);
            return ret;
        },
        __wbg_screenX_5a151c39d929aec5: function(arg0) {
            const ret = arg0.screenX;
            return ret;
        },
        __wbg_screenY_1d7dd58c25150488: function(arg0) {
            const ret = arg0.screenY;
            return ret;
        },
        __wbg_self_4d13fca7e70dc961: function() { return handleError(function () {
            const ret = self.self;
            return ret;
        }, arguments); },
        __wbg_setAttribute_59010a2ba07fefe6: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            var v0 = getCachedStringFromWasm0(arg1, arg2);
            var v1 = getCachedStringFromWasm0(arg3, arg4);
            arg0.setAttribute(v0, v1);
        }, arguments); },
        __wbg_setTimeout_613a21b62dc655a1: function() { return handleError(function (arg0, arg1) {
            const ret = setTimeout(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_set_3965876d3f9128ce: function(arg0, arg1, arg2) {
            arg0.set(arg1, arg2 >>> 0);
        },
        __wbg_set_textContent_96374feab3bdb74d: function(arg0, arg1, arg2) {
            var v0 = getCachedStringFromWasm0(arg1, arg2);
            arg0.textContent = v0;
        },
        __wbg_shiftKey_1fc278f6c0bdc762: function(arg0) {
            const ret = arg0.shiftKey;
            return ret;
        },
        __wbg_shiftKey_5de2af880593f01d: function(arg0) {
            const ret = arg0.shiftKey;
            return ret;
        },
        __wbg_shiftKey_cf8e45a1073ecfba: function(arg0) {
            const ret = arg0.shiftKey;
            return ret;
        },
        __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_subarray_6ecaf2270e9d1ab5: function(arg0, arg1, arg2) {
            const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
            return ret;
        },
        __wbg_tangentialPressure_2bef30d970fa0c45: function(arg0) {
            const ret = arg0.tangentialPressure;
            return ret;
        },
        __wbg_target_ed13ba11247ec98d: function(arg0) {
            const ret = arg0.target;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_textContent_609d269fdd8c549b: function(arg0, arg1) {
            const ret = arg1.textContent;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_then_a881425aa91448ac: function(arg0, arg1) {
            const ret = arg0.then(arg1);
            return ret;
        },
        __wbg_tiltX_853d954419bfaec9: function(arg0) {
            const ret = arg0.tiltX;
            return ret;
        },
        __wbg_tiltY_83b2cdf9b0dacf96: function(arg0) {
            const ret = arg0.tiltY;
            return ret;
        },
        __wbg_timeRemaining_56761439753819a1: function(arg0) {
            const ret = arg0.timeRemaining();
            return ret;
        },
        __wbg_twist_e95c6cc00a9ca118: function(arg0) {
            const ret = arg0.twist;
            return ret;
        },
        __wbg_type_47133b4912fea9de: function(arg0, arg1) {
            const ret = arg1.type;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_type_480f7c56892a6bf4: function(arg0, arg1) {
            const ret = arg1.type;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_01f43def0638071f: function(arg0, arg1) {
            const ret = arg1.value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_69816e5b431b8bab: function(arg0, arg1) {
            const ret = arg1.value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_bc48dcc56e747156: function(arg0, arg1) {
            const ret = arg1.value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_versions_1d7e019137ae42fd: function(arg0) {
            const ret = arg0.versions;
            return ret;
        },
        __wbg_which_19b697b272bb2e08: function(arg0) {
            const ret = arg0.which;
            return ret;
        },
        __wbg_width_757292d095be3165: function(arg0) {
            const ret = arg0.width;
            return ret;
        },
        __wbg_window_aedc18e26f22b6c8: function() { return handleError(function () {
            const ret = window.window;
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 215, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__hd22e813cfed4608b);
            return ret;
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 221, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h0c2ca8bd1cb63063);
            return ret;
        },
        __wbindgen_cast_0000000000000003: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Ref(NamedExternref("Event"))], shim_idx: 217, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures________invoke__hc043897b67ac2cbe);
            return ret;
        },
        __wbindgen_cast_0000000000000004: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 239, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h552ff5291d735070);
            return ret;
        },
        __wbindgen_cast_0000000000000005: function(arg0, arg1) {
            var v0 = getCachedStringFromWasm0(arg0, arg1);
            // Cast intrinsic for `Ref(CachedString) -> Externref`.
            const ret = v0;
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./cal-ender_bg.js": import0,
    };
}

function wasm_bindgen__convert__closures_____invoke__h552ff5291d735070(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures_____invoke__h552ff5291d735070(arg0, arg1);
}

function wasm_bindgen__convert__closures_____invoke__hd22e813cfed4608b(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__hd22e813cfed4608b(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h0c2ca8bd1cb63063(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h0c2ca8bd1cb63063(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures________invoke__hc043897b67ac2cbe(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures________invoke__hc043897b67ac2cbe(arg0, arg1, arg2);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_destroy_closure(state.a, state.b));

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
    if (builtInMatches && builtInMatches.length > 1) {
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

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getFromExternrefTable0(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getFromExternrefTable0(idx) { return wasm.__wbindgen_externrefs.get(idx); }

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

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
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('cal-ender_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
