# 编程作业

本次实现实现了以下内容
1. 为TaskControlBlock增加syscall_count字段，统计系统调用次数
2. 实现了系统调用号到索引的方便函数，使得syscall_count只需要占用n和系统调用号的空间，极大的降低了TaskControlBlock的空间占用
3. 对TaskManager实现了两个pub函数，分别是当前进程的系统调用计数(syscall_count_inc)和获取系统调用计数(get_syscall_count)
4. 按照要求实现了trace系统调用
5. 在进入系统调用入口处时，执行syscall_count_inc增加系统调用计数

# 简答作业

1.  程序报错如下：
    - ch2b_bad_address: PageFault 触发页错误
    - ch2b_bad_instructions: IllegalInstruction 触发非法指令
    - ch2b_bad_register: IllegalInstruction 触发非法指令

    库版本：QEMU 7.0.0



2. `trap.S` 中 `__alltraps` 和 `__restore` 的作用
    1. 刚进入 __restore 时，sp 代表了什么值。请指出 __restore 的两种使用情景。
        sp是内核栈指针 用于返回用户态和首次启动应用程序
    2.  ld t0, 32*8(sp)    # 加载 sstatus
        ld t1, 33*8(sp)    # 加载 sepc
        ld t2, 2*8(sp)     # 加载用户栈指针（x2/sp）
        csrw sstatus, t0   # 恢复 sstatus
        csrw sepc, t1      # 恢复 sepc
        csrw sscratch, t2  # 恢复用户栈指针到 sscratch
    3. x2目前指向内核栈需要操作完成后才切换到用户栈  x4用户态无法使用
    4. sp指向用户栈，sscratch指向内核栈。
    5. 状态切换发生在 sret，此时寄存器切换到到用户态状态
    6. 执行指令后sp指向内核栈，sscratch指向用户栈。
    7. 从用户态进入内核态发生在 call trap_handler。

# 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

    *无*

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

    *无*

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。
我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。
我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。
我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。
我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。