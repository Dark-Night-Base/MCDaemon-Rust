# Rust Note

Rust 学习笔记

## 安装

[官方教程-入门](https://www.rust-lang.org/zh-CN/learn/get-started)

*Windows* 换源：
> 在环境变量中加入以下两个：
> 
> 变量名：RUSTUP_DIST_SERVER，变量值：https://mirrors.ustc.edu.cn/rust-static
> 
> 变量名：RUSTUP_UPDATE_ROOT，变量值：https://mirrors.ustc.edu.cn/rust-static/rustup
>
> 加好之后再运行rustup-init.exe即可
>> 原文链接：https://blog.csdn.net/bu2_int/article/details/79758960

## 参考教程

* [初识 Rust](https://www.ibm.com/developerworks/cn/opensource/os-know-rust/)
* [通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/)

### match

相对于 `switch`

### 子进程

perlude 提供的 stdout、stdin会在使用一次后被移除。
