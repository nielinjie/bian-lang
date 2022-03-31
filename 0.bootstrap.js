(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/bian_front_wasm.js":
/*!*********************************!*\
  !*** ../pkg/bian_front_wasm.js ***!
  \*********************************/
/*! exports provided: greet, compute_and_represent, __wbg_alert_1b463e82e7151f25, __wbindgen_json_parse */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _bian_front_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./bian_front_wasm_bg.wasm */ \"../pkg/bian_front_wasm_bg.wasm\");\n/* harmony import */ var _bian_front_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./bian_front_wasm_bg.js */ \"../pkg/bian_front_wasm_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"greet\", function() { return _bian_front_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"greet\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"compute_and_represent\", function() { return _bian_front_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"compute_and_represent\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_alert_1b463e82e7151f25\", function() { return _bian_front_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_alert_1b463e82e7151f25\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_json_parse\", function() { return _bian_front_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_json_parse\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/bian_front_wasm.js?");

/***/ }),

/***/ "../pkg/bian_front_wasm_bg.js":
/*!************************************!*\
  !*** ../pkg/bian_front_wasm_bg.js ***!
  \************************************/
/*! exports provided: greet, compute_and_represent, __wbg_alert_1b463e82e7151f25, __wbindgen_json_parse */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"greet\", function() { return greet; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"compute_and_represent\", function() { return compute_and_represent; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_alert_1b463e82e7151f25\", function() { return __wbg_alert_1b463e82e7151f25; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_json_parse\", function() { return __wbindgen_json_parse; });\n/* harmony import */ var _bian_front_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./bian_front_wasm_bg.wasm */ \"../pkg/bian_front_wasm_bg.wasm\");\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\nlet cachedTextDecoder = new lTextDecoder('utf-8', {\n  ignoreBOM: true,\n  fatal: true\n});\ncachedTextDecoder.decode();\nlet cachegetUint8Memory0 = null;\n\nfunction getUint8Memory0() {\n  if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _bian_front_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n    cachegetUint8Memory0 = new Uint8Array(_bian_front_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n  }\n\n  return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nconst heap = new Array(32).fill(undefined);\nheap.push(undefined, null, true, false);\nlet heap_next = heap.length;\n\nfunction addHeapObject(obj) {\n  if (heap_next === heap.length) heap.push(heap.length + 1);\n  const idx = heap_next;\n  heap_next = heap[idx];\n  heap[idx] = obj;\n  return idx;\n}\n/**\n*/\n\n\nfunction greet() {\n  _bian_front_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"greet\"]();\n}\nlet WASM_VECTOR_LEN = 0;\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\nlet cachedTextEncoder = new lTextEncoder('utf-8');\nconst encodeString = typeof cachedTextEncoder.encodeInto === 'function' ? function (arg, view) {\n  return cachedTextEncoder.encodeInto(arg, view);\n} : function (arg, view) {\n  const buf = cachedTextEncoder.encode(arg);\n  view.set(buf);\n  return {\n    read: arg.length,\n    written: buf.length\n  };\n};\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n  if (realloc === undefined) {\n    const buf = cachedTextEncoder.encode(arg);\n    const ptr = malloc(buf.length);\n    getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n    WASM_VECTOR_LEN = buf.length;\n    return ptr;\n  }\n\n  let len = arg.length;\n  let ptr = malloc(len);\n  const mem = getUint8Memory0();\n  let offset = 0;\n\n  for (; offset < len; offset++) {\n    const code = arg.charCodeAt(offset);\n    if (code > 0x7F) break;\n    mem[ptr + offset] = code;\n  }\n\n  if (offset !== len) {\n    if (offset !== 0) {\n      arg = arg.slice(offset);\n    }\n\n    ptr = realloc(ptr, len, len = offset + arg.length * 3);\n    const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n    const ret = encodeString(arg, view);\n    offset += ret.written;\n  }\n\n  WASM_VECTOR_LEN = offset;\n  return ptr;\n}\n\nfunction getObject(idx) {\n  return heap[idx];\n}\n\nfunction dropObject(idx) {\n  if (idx < 36) return;\n  heap[idx] = heap_next;\n  heap_next = idx;\n}\n\nfunction takeObject(idx) {\n  const ret = getObject(idx);\n  dropObject(idx);\n  return ret;\n}\n/**\n* @param {string} e\n* @returns {any}\n*/\n\n\nfunction compute_and_represent(e) {\n  var ptr0 = passStringToWasm0(e, _bian_front_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _bian_front_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n  var len0 = WASM_VECTOR_LEN;\n  var ret = _bian_front_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"compute_and_represent\"](ptr0, len0);\n  return takeObject(ret);\n}\nfunction __wbg_alert_1b463e82e7151f25(arg0, arg1) {\n  alert(getStringFromWasm0(arg0, arg1));\n}\n;\nfunction __wbindgen_json_parse(arg0, arg1) {\n  var ret = JSON.parse(getStringFromWasm0(arg0, arg1));\n  return addHeapObject(ret);\n}\n;\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/bian_front_wasm_bg.js?");

/***/ }),

/***/ "../pkg/bian_front_wasm_bg.wasm":
/*!**************************************!*\
  !*** ../pkg/bian_front_wasm_bg.wasm ***!
  \**************************************/
/*! exports provided: memory, greet, compute_and_represent, __wbindgen_malloc, __wbindgen_realloc */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./bian_front_wasm_bg.js */ \"../pkg/bian_front_wasm_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/bian_front_wasm_bg.wasm?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ }),

/***/ "./w.js":
/*!**************!*\
  !*** ./w.js ***!
  \**************/
/*! exports provided: compute_and_represent */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"compute_and_represent\", function() { return compute_and_represent; });\n/* harmony import */ var bian_front_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! bian-front-wasm */ \"../pkg/bian_front_wasm.js\");\n\nvar compute_and_represent = function compute_and_represent(s) {\n  return bian_front_wasm__WEBPACK_IMPORTED_MODULE_0__[\"compute_and_represent\"](s);\n};\n\n//# sourceURL=webpack:///./w.js?");

/***/ })

}]);