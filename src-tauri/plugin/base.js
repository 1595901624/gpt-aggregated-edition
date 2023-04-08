/*********************************************************************************
 * 
 *                              这是一个基础脚本 V1.0
 * 
 **********************************************************************************/
// 禁止右键
// window.oncontextmenu = function (e) {
// e.preventDefault();
// console.log(e);
// 获取上下文菜单的 menu 元素  
// const menu = document.getElementById('context-menu');
// console.log(menu)
// // 创建新的菜单项  
// const item = document.createElement('div');
// item.classList.add('item');

// // 为菜单项设置点击事件  
// item.addEventListener('click', function () {
//     // 处理点击事件  
// });

// // 将菜单项添加到上下文菜单中  
// menu.appendChild(item);
// }

// 公告条幅
// window.onload = function (e) {
//     // alert("1");
//     // // 创建一个div元素，并设置id和文本内容
//     let bar = document.createElement('div');
//     bar.id = 'one-bar';
//     let html = '<div style="position: fixed;top: 0; left: 0;width: 100%;background-color: #007acc;color: #fff;font-size: 24px;font-weight: bold;text-align: center;line-height: 30px; height:30px; background-color:orange; width:100%; font-size:12px;z-index: 9999;">平台切换时需要时间，请耐心等待...</div>';
//     bar.innerHTML = html;

//     // 将元素添加到body中
//     document.body.insertAdjacentElement('afterbegin', bar);

//     setTimeout(function() {
//         document.getElementById('one-bar').remove();
//     }, 5000)
// }

// 重写replace方法
// const _replace = location.replace;
// location.replace = function (url) {
//     alert(url)
//     // _replace(url);
// }

// IPC 以下代码来自：https://github.dev/lencx/ChatGPT
const uid = () => window.crypto.getRandomValues(new Uint32Array(1))[0];
function transformCallback(callback = () => { }, once = false) {
    const identifier = uid();
    const prop = `_${identifier}`;
    Object.defineProperty(window, prop, {
        value: (result) => {
            if (once) {
                Reflect.deleteProperty(window, prop);
            }
            return callback(result)
        },
        writable: false,
        configurable: true,
    })
    return identifier;
}
async function invoke(cmd, args) {
    return new Promise((resolve, reject) => {
        if (!window.__TAURI_POST_MESSAGE__) reject('__TAURI_POST_MESSAGE__ does not exist!');
        const callback = transformCallback((e) => {
            resolve(e);
            Reflect.deleteProperty(window, `_${error}`);
        }, true)
        const error = transformCallback((e) => {
            reject(e);
            Reflect.deleteProperty(window, `_${callback}`);
        }, true)
        window.__TAURI_POST_MESSAGE__({
            cmd,
            callback,
            error,
            ...args
        });
    });
}

async function message(message) {
    invoke('messageDialog', {
        __tauriModule: 'Dialog',
        message: {
            cmd: 'messageDialog',
            message: message.toString(),
            title: null,
            type: null,
            buttonLabel: null
        }
    });
}

window.uid = uid;
window.invoke = invoke;
window.message = message;
window.transformCallback = transformCallback;





