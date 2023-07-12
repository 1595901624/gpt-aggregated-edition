/*********************************************************************************
 * 
 *                              这是一个New Bing的脚本 V1.0
 * 
 **********************************************************************************/
const _chatBtnClass = "joinWaitList link primary";
if (window.location.href.indexOf("bing.com/new") != -1) {   
    try {
        // 创建一个MutationObserver实例
        let observer = new MutationObserver(function (mutations) {
            let chatBtnArray = document.getElementsByClassName(_chatBtnClass);
            // 修复立即聊天弹出新窗口的问题
            processNewBing(chatBtnArray);
        });

        // 开始观察document，并在节点添加或删除时检测变化
        observer.observe(document, {
            childList: true,
            subtree: true
        });
    } catch (e) {

    }
}

function processNewBing(chatBtnArray) {
    console.log('process new bing window');
    if (chatBtnArray.length > 0) {
        for (let index = 0; index < chatBtnArray.length; index++) {
            let element = chatBtnArray[index];
            element.target = "";
        }
    }

}