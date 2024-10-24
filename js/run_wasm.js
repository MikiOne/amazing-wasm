const runtime = chrome.runtime || browser.runtime;

let wasmInstance;

async function initWasm() {
    const response = await fetch(runtime.getURL('amazing_wasm_bg.wasm'));
    const buffer = await response.arrayBuffer();
    const { instance } = await WebAssembly.instantiate(buffer);
    wasmInstance = instance;
}

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
    if (message.action === 'runWasm') {
        if (wasmInstance) {
            // 调用 WASM 中的某个函数，例如 run()
            wasmInstance.exports.run(); // 假设你有一个名为 run 的导出函数
        }
        sendResponse({ status: 'WASM function executed' });
    }
});

// 初始化 WASM 模块
initWasm();