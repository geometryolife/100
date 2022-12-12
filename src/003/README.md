# Web Server

这是一个用 Rust 实现的单线程 Web 服务器，简单实现了请求-响应的功能。

- HTTP and TCP
  - 请求-响应
- 实现服务端
- 客户端：浏览器

## Lib

- `std::net::TcpListener` => TCP 套接字服务器，监听连接
- `std::net::TcpStream` => 本地和远程套接字之间的 TCP 流

- HTTP 简介
  - (1) http 请求报文包含三部分：请求行、请求头、请求体
    - Method Request-URL HTTP-Version CRLF    // 请求行：请求方式、协议、协议版本等
    - headers CRLF    // 请求头：包含若干个属性，格式为 `属性名:属性值`，服务端据此获取客户端的信息
    - message-body    // 请求体：客户端真正要传递给服务端的内容
  - (2) http 响应报文也有三部分内容：响应行、响应头、响应体
    - HTTP-Version Status-Code Reason-Phrase CRLF    // 响应行：报文协议及版本，状态码及状态描述
    - headers CRLF    // 响应头：由多个属性组成
    - message-body    // 响应体：真正响应的内容

## 步骤

1. 建立 TcpListener
2. 读取请求内容
3. 编写响应
  - 返回一个响应行
  - 返回一个真正的网页
  - 有条件地返回网页

## 视频资源

[【rust编程小项目：webserver-哔哩哔哩】](https://b23.tv/5MsyTQz)
