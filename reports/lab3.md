# 编程作业

本次实现实现了以下内容
1. 将syscall_count、mmap、munmap迁移到TaskControlBlockInner
2. 实现sys_set_priority系统调用为进程设置优先级
3. 实现sys_spawn系统调用，需要注意fork后当前环境还是父进程
4. 实现了stride调度算法 需要注意BigStride不要设置的太大，否则容易溢出，导致进程饥饿

# 简答作业

1. 实际情况是p1永远饥饿
2. stride每次增加的值<=BigStride / 2,会导致两者之间的值也会<=BigStride / 2
3. 
    ```rust

    use core::cmp::Ordering;

    struct Stride(u64);

    impl PartialOrd for Stride {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let diff = self.0.wrapping_sub(other.0) as i64;
        if diff < 0 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
        }
    }

    impl PartialEq for Stride {
        fn eq(&self, other: &Self) -> bool {
            false
        }
    }
    ```


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