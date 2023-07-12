/*********************************************************************************
 * 
 *                              这是一个GPT AI的脚本 V1.0
 * 
 **********************************************************************************/
if (window.location.href.indexOf("gptai.cloud") != -1) {
    try {
        // 创建一个MutationObserver实例
        let observer = new MutationObserver(function (mutations) {
            try {
                processGPTAI();
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

function processGPTAI() {
    // 移除头部
    let header = document.querySelector(".flex.justify-center.mt-2");
    if (header != null && header != undefined) {
        header.remove();
    }

    // 移除广告
    let ads = document.getElementsByClassName("adsbygoogle");
    if (ads != null && ads != undefined) {
        for (let index = ads.length - 1; index >= 0; index--) {
            let element = ads[index];
            element.remove();
        }
    }

}