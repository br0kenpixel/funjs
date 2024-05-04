((globalThis) => {
    const core = Deno.core;

    function argsToMessage(...args) {
        stack = args.map((arg) => JSON.stringify(arg)).join(" ");
        var len = stack.length;

        if (stack[0] === '"' && stack[len - 1] === '"') {
            stack = stack.substring(1, len - 1);
        }

        if (stack[0] === '\'' && stack[len - 1] === '\'') {
            stack = stack.substring(1, len - 1);
        }

        return stack;
    }

    globalThis.console = {
        log: (...args) => {
            core.print(`${argsToMessage(...args)}\n`, false);
        },
        error: (...args) => {
            core.print(`${argsToMessage(...args)}\n`, true);
        },
    };

    globalThis.funjs = {
        fs: {
            readFile: async (path) => {
                return core.opAsync("op_read_file", path);
            },
            writeFile: async (path, contents) => {
                return core.opAsync("op_write_file", path, contents);
            },
            removeFile: (path) => {
                return core.ops.op_remove_file(path);
            },
            listDir: (path) => {
                return core.ops.op_listdir(path);
            }
        },
        math: {
            min: (nums) => {
                return core.ops.op_math_min(nums);
            },
            max: (nums) => {
                return core.ops.op_math_max(nums);
            },
            sum: (nums) => {
                return core.ops.op_math_sum(nums);
            }
        },
        random: {
            range: (min, max, inclusive) => {
                return core.ops.op_rand_range(min, max, inclusive);
            },
            string: (len) => {
                return core.ops.op_rand_string(len);
            }
        },
        runtime: {
            version: () => {
                return core.ops.op_runtime_version();
            }
        }
    };
})(globalThis);