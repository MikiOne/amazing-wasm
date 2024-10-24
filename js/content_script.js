document.addEventListener('click', () => {
    chrome.runtime.sendMessage({ action: 'runWasm' }, (response) => {
        console.log(response.status); // 打印来自背景脚本的响应
    });
});