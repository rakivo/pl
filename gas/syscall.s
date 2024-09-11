.text
.globl syscall1
syscall1:
  movq %rdi, %rax
  movq %rsi, %rdi
  syscall
  ret

syscall3:
  .globl syscall3
  movq %rdi, %rax
  movq %rsi, %rdi
  movq %rdx, %rsi
  movq %rcx, %rdx
  syscall
  ret
