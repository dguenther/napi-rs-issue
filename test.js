const { TestStruct } = require('./index.js')
const { Worker, isMainThread, workerData } = require('worker_threads')

if (isMainThread) {
    const WORKER_COUNT = 6;

    for (let i = 1; i <= WORKER_COUNT; i++) {
        new Worker(__filename, {
            workerData: { workerNum: i }
        })
    }
} else {
    console.log('inside worker', workerData.workerNum)
    const s = TestStruct.fromNumber(workerData.workerNum)
    console.log('finished on worker', s.toString())
}
