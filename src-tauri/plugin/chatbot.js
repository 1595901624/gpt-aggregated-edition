/*********************************************************************************
 * 
 *                              这是一个Chat Bot的脚本 V1.0
 * 
 **********************************************************************************/
if (window.location.href.indexOf("chatbot.theb.ai") != -1) {
    try {
        // 创建一个MutationObserver实例
        const observer = new MutationObserver(function (mutations) {
            try {
                processChatBot();
            } catch (e) {

            }
        });

        // 开始观察document，并在节点添加或删除时检测变化
        observer.observe(document, {
            childList: true,
            subtree: true
        });

    } catch (e) {

    }
}

function processChatBot() {
    console.log("process!")
    let ads = document.getElementsByClassName("adsbygoogle");
    if (ads == null || ads == undefined) {
        return;
    }
    for (let index = ads.length - 1; index >= 0; index--) {
        const element = ads[index];
        element.remove();
    }

    let style = window.getComputedStyle(document.body);
    let paddingBottom = style.getPropertyValue('padding-bottom');
    if (paddingBottom != '0px') {
        document.body.style.setProperty('padding-bottom', '0px')
    }
}