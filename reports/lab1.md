### 实现功能总结

我在程序的TaskControlBlock中添加了task_info的结构体，结构体初始化为空
在程序第一次运行时调用let time = crate::timer::get_time_ms();并将time记录在task_info结构中，直到程序调用sys_get_time时进行计算间隔时间
程序每次trap时，我在trap_handler中进行了一部分修改，将对task-info结构体中的调用计数进行更新


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
 - 1.刚进入__restore时，a0寄存器是上一个函数的返回值，也就是trap_handler的返回 &mut TrapContext
 - 2.
    ``` asm
    // 将sp + 32*8 的内存位置的数据加载到t0寄存器中，此时sp->kernel stack,所以是将中断前保存的sstatus寄存器（中断前的特权级）的内容放置到t0
    ld t0, 32*8(sp)
    // t1保存的spec寄存器内容，记录用户模式trap前的最后一条指令的地址
    ld t1, 33*8(sp)
    // t2保存的sscratch寄存器，sscratch->user stack
    ld t2, 2*8(sp)
    //将t0写入sstatus
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    // 这段即恢复user栈指针，记录trap前的最后一条指令地址，以及记录trap前的特权级，以便系统处理完trap_handler之后返回
    ``` 
- 3.x2 寄存器一般是sp寄存器，但是我们通过sscratch保存了sp，用户程序无需知道kernel的sp指针，x4是线程指针寄存器，目前没有使用到
- 4.执行过后，sp->kernel stack, sscratch->user stack
- 5.发生在sret这一条指令，这条指令执行后，
    - PC <- sepc
    - 特权级 <- sstatus.SPP //已设置为用户态，所以回到用户态
    - sstatus.SPP <- 0
    - sstatus.SPIE <- 1
- 6.执行之后，sp->kernel stack, sscratch->user stack
- 7.是"ecall"指令执行后

### 荣誉准则

1.在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无交流

2.此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

参考了rCore-Tutorial-Book 第三版

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

