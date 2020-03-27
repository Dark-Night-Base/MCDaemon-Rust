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

> Crates 换源（Windows也适用）
> 
> 编辑`~/.cargo/config`：
> 
> [source.crates-io]
> registry = "https://github.com/rust-lang/crates.io-index"
> replace-with = 'ustc'
> [source.ustc]
> registry = "git://mirrors.ustc.edu.cn/crates.io-index"
> 
> 如果所处的环境中不允许使用 git 协议，可以把上述地址改为：
> 
> registry = "https://mirrors.ustc.edu.cn/crates.io-index"
> 
>> 原文链接：https://www.linuxidc.com/Linux/2019-05/158485.htm

## 参考教程

* [初识 Rust](https://www.ibm.com/developerworks/cn/opensource/os-know-rust/)
* [通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/)

### match

相对于 `switch`

### 子进程

perlude 提供的 stdout、stdin会在使用一次后被移除。
