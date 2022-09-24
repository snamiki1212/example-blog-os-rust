
```sh
$ rustup override set nightly
$ rustc --version
# => xx-nightly
$ rustup component add rust-src
```

```sh
$ cargo install bootimage
$ rustup component add llvm-tools-preview
$ cargo bootimage
$ ls -l target/x86_64-blog_os/debug/bootimage-blog_os.bin
```

```sh
$ brew install qemu
$ qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin
```
