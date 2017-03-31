// Run in node.js context via emscripten.
// Node.js version is v4.1.1 now.
"use strict";

mergeInto(LibraryManager.library, {
    exec_js_eval: function (ptr) {
        "use strict";

        const str = Module.Pointer_stringify(ptr);
        new Function(str)();
    },
    exec_async: function (containerPtr) {
        "use strict";

        const asyncResultContainerSet = Module.cwrap("async_result_container_set", void 0, ["number", "string"]);

        console.log("async js context start");
        Module.asm.setAsync();

        new Promise(resolve => {
            setTimeout(resolve, 100);
        }).then(() => {
            asyncResultContainerSet(containerPtr, `callback from Promise at ${new Date()}`);
            Module._emscripten_async_resume();
            console.log("async js context end");
        });
    },
});
