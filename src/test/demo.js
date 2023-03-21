if (window.location.href.includes("yiyan.baidu.com")) {
    // 文心一言
    const style = document.createElement('style');
    style.innerHTML = `.ebhelper-hide { visibility: hidden !important; }`;
    document.head.appendChild(style);

    // ai图片水印标记
    const aiImageWaterFlag = "x-bce-process=style/wm_ai";

    // 创建一个MutationObserver实例
    const observer = new MutationObserver(function (mutations) {
        // 获取水印元素
        let watermark = document.querySelector("div[id^='eb_']");
        if (watermark != null && !watermark.classList.contains('ebhelper-hide')) {
            hideWatermark(watermark);
        }

        // 获取弹窗的元素
        let timeoutDialog = document.querySelector("div[class='ant-modal-root']");
        if (timeoutDialog != null && !timeoutDialog.classList.contains('ebhelper-hide')) {
            hideTimeoutDialog(timeoutDialog);
        }

        // 隐藏图片水印并处理头像
        let allImage = document.querySelectorAll("img");
        if (allImage != null) {
            hideAIImageWatermark(allImage);
        }
    });

    // 开始观察document，并在节点添加或删除时检测变化
    observer.observe(document, {
        childList: true,
        subtree: true
    });


    /**
     * 隐藏超时弹窗
     */
    function hideTimeoutDialog(element) {
        console.log("隐藏超时弹窗!");
        element.classList.add('ebhelper-hide');
    }


    /**
     * 隐藏水印
     */
    function hideWatermark(element) {
        console.log("隐藏水印!");
        element.classList.add('ebhelper-hide');
    }

    /**
     * 隐藏图片水印并处理头像
     */
    function hideAIImageWatermark(images) {
        images.forEach(element => {
            let url = element.getAttribute("src");
            // 去除水印
            if (url != null && url.indexOf(aiImageWaterFlag) != -1) {
                if (url.indexOf(aiImageWaterFlag) != -1) {
                    console.log("隐藏图片水印!");
                    element.setAttribute("src", url.replace(aiImageWaterFlag, ""))
                }
            }
            // 处理头像
            if (url != null
                && element.getAttribute("alt") == '头像'
                && url.indexOf('icon-rb') == '-1') {
                console.log("设置头像为默认值!");
                element.setAttribute("src", 'https://nlp-eb.cdn.bcebos.com/logo/favicon.ico')
            }
        });
    }
}
