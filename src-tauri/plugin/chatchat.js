/*********************************************************************************
 * 
 *                              这是一个Chat Chat的脚本 V1.0
 * 
 **********************************************************************************/
if (window.location.href.indexOf("chat.okis.dev") != -1) {
    try {
        window.onload = function (e) {
            process();
        }
    } catch (e) {

    }
}

const _chatChatMinHeithClassName = "md:min-h-[600px]";

function process() {
    let textContentElement = document.getElementsByClassName("md:min-h-[600px]")[0];
    textContentElement.classList.remove(_chatChatMinHeithClassName);
}