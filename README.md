<div align="center">
    <img src="demo/gpt_all_in_one.jpg" alt="chatgpt"/>
    <h1>Gpt聚合版</h1>
    <div><img src="https://img.shields.io/badge/stable%20version-v0.4.0-blue.svg?style=flat"></img>
<!-- <img src="https://img.shields.io/badge/preview%20version-v0.4.0%20PREVIEW%201-orange.svg?style=flat"></img> -->
<img src="https://img.shields.io/badge/license-GPL%203.0-brightgreen.svg?style=flat"></img>
<img src="https://img.shields.io/badge/language-简体中文-brightgreen.svg?style=flat"></img></div>
    <h4>聚合ChatGPT官方版、ChatGPT免费版、文心一言、POE、chat chat等多个平台。</h4>
</div>
  
目前软件仍处于早期开发阶段，功能尚不完善且极其不稳定，后续功能敬请期待。如果您有好的建议，可以通过提 issue 来告诉我。

**首次平台切换会很慢，请耐心等待**

#### 截图样例
<div>
<img src="demo/freegpt.png" width=30% alt="freegpt"/>
<img src="demo/chatchat.png" width=30% alt="chatchat"/>
<img src="demo/yiyan.png" width=30% alt="文心一言"/>
<img src="demo/poe.png" width=30% alt="poe"/>
<img src="demo/chatgpt.png" width=30% alt="chatgpt"/>
<img src="demo/taskmode.png" height=30% width="30%" alt=""/>
</div>


#### 聚合的平台

##### ChatGPT官方版
需要注册账号、需要登录官方账号(不建议非美国地区访问)

##### ChatGPT免费版
无需注册账号、无需登录账号、稳定极速(部分国家和地区无法正常访问)

##### ChatChat
无需注册账号、无需登录账号、稳定极速、每日限额，可正常访问。

##### POE
需要注册账户、需要登录账号、集合Sage（免费）、ChatGPT4（付费）、ChatGPT（付费）、Clude（免费）、Clude+（付费）、DragonFly（免费）(部分国家和地区无法正常访问)

##### 文心一言
需要有体验资格且必须登录百度账号

##### Bard
需有体验资格且必须登录 Google 账号(部分国家和地区无法正常访问)

#### 功能介绍

软件是使用 Rust + tauri 构建的。

- [x] 多平台切换
- [x] 窗口模式和任务栏模式切换
- [x] 跨平台（Windows、Mac、Linux）
- [x] 聚合更多平台
- [x] 文心一言定制化功能
- [ ] 自定义脚本支持
- [ ] 更多功能敬请期待...

#### 下载
请前往 Release 页面下载最新版本。

[Gitee](https://gitee.com/haoyu3/gpt-aggregated-edition/releases)  

[Github](https://github.com/1595901624/gpt-aggregated-edition/releases)

注：Windows、Mac、Linux平台先均已支持。Linux平台自 `0.4.0` 版本开始支持。

#### 反馈及建议

* 通过《Rust学习日记》公众号内小窗反馈
* Github 提 issue
* Gitee 提 issue

#### 二次开发
```shell
pnpm install
pnpm tauri dev
pnpm tauri build
```


