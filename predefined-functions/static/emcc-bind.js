// Run in node.js context via emscripten.
// Node.js version is v4.1.1 now.
"use strict";

mergeInto(LibraryManager.library, {
    exec_js_eval: function (ptr) {
        "use strict";

        const str = Module.Pointer_stringify(ptr);
        new Function(str)();
    },
});
