## Reproduction steps

1. Run `yarn install` to install packages.
2. Run `yarn build` to build the module.
3. Run `yarn test` to execute `test.js`.

## Observed output

You should expect to see something like this:

```
$ cross-env RUST_BACKTRACE=1 node test.js
inside worker 4
inside worker 1
inside worker 2
inside worker 3
inside worker 5
inside worker 6
finished on worker 4

node:internal/event_target:777
  process.nextTick(() => { throw err; });
                           ^
Error: Class contains no `constructor`, can not new it!
    at Object.<anonymous> (D:\repos\napi-rs-issue\test.js:14:26)
    at Module._compile (node:internal/modules/cjs/loader:1101:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1153:10)
    at Module.load (node:internal/modules/cjs/loader:981:32)
    at Function.Module._load (node:internal/modules/cjs/loader:822:12)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:81:12)
    at MessagePort.<anonymous> (node:internal/main/worker_thread:187:24)
    at MessagePort.[nodejs.internal.kHybridDispatch] (node:internal/event_target:562:20)
    at MessagePort.exports.emitMessage (node:internal/per_context/messageport:23:28)
Emitted 'error' event on Worker instance at:
    at Worker.[kOnErrorMessage] (node:internal/worker:289:10)
    at Worker.[kOnMessage] (node:internal/worker:300:37)
    at MessagePort.<anonymous> (node:internal/worker:201:57)
    at MessagePort.[nodejs.internal.kHybridDispatch] (node:internal/event_target:562:20)
    at MessagePort.exports.emitMessage (node:internal/per_context/messageport:23:28)
error Command failed with exit code 1.
```