# Detects instances of new Buffer(argument) where argument is any non-literal value (`security/detect-new-buffer`)

⚠️ This rule *warns* in the ✅ `recommended` config.

<!-- end auto-generated rule header -->

`new Buffer()` now emits a deprecation warning in Node.js.

More information: [new Buffer(number) is unsafe](https://github.com/nodejs/node/issues/4660)
