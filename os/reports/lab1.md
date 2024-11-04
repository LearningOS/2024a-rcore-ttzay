### 实现功能总结




### 问答题

#### 1
SBI版本和程序出错行为如下
``` sh
[rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.3

程序访问了addr 0x000 
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
程序在U mode使用特权指令 sret ，在trap_handler 被处理
[kernel] IllegalInstruction in application, kernel killed it.
程序在U mode访问特权寄存器 sstatus ，在trap_handler 被处理
[kernel] IllegalInstruction in application, kernel killed it.
``` 

### 2
 - 刚进入__restore时，a0寄存器是上一个函数的返回值，也就是trap_handler的返回 &mut TrapContext
 - 
