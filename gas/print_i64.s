.text
.globl print_i64
print_i64:
  pushq   %rbp                     # Save base pointer
  movq    %rsp, %rbp               # Set up base pointer
  subq    $64, %rsp                # Allocate space on stack
  movq    %rsi, %r15
  movq    %rdi, -8(%rbp)           # Store rdi on the stack
  movl    $0, -36(%rbp)            # Initialize local variable to 0
  movl    $0, -40(%rbp)            # Initialize local variable to 0
  cmpq    $0, -8(%rbp)             # Compare stored value with 0
  jge     .LBB0_2                  # Jump if greater or equal
  movl    $1, -40(%rbp)            # Set flag to indicate negative number
  xorl    %eax, %eax               # Zero out eax
  subq    -8(%rbp), %rax           # Compute -rax
  movq    %rax, -8(%rbp)           # Store the result back
  .LBB0_2:
  cmpq    $0, -8(%rbp)             # Compare stored value with 0
  jne     .LBB0_4                  # Jump if not equal
  movl    -36(%rbp), %eax          # Load local variable into eax
  movl    %eax, %ecx               # Copy eax to ecx
  addl    $1, %ecx                 # Increment ecx
  movl    %ecx, -36(%rbp)          # Store back the incremented value
  cdqe                             # Sign-extend eax into rax
  movb    $48, -32(%rbp, %rax)     # Store ASCII '0'
  jmp     .LBB0_8                  # Jump to end
  .LBB0_4:
  jmp     .LBB0_5                  # Jump to main loop
  .LBB0_5:
  cmpq    $0, -8(%rbp)             # Compare stored value with 0
  jle     .LBB0_7                  # Jump if less or equal
  movq    -8(%rbp), %rax           # Load stored value into rax
  movl    $10, %ecx                # Move 10 into ecx
  cqo                              # Sign-extend rax into rdx:rax
  idivq   %rcx                     # Divide rdx:rax by 10
  movl    %edx, %eax               # Move remainder into eax
  movl    %eax, -44(%rbp)          # Store remainder in local variable
  movl    -44(%rbp), %eax          # Load local variable into eax
  addl    $48, %eax                # Convert to ASCII
  movb    %al, %cl                 # Copy to cl
  movl    -36(%rbp), %eax          # Load another local variable
  movl    %eax, %edx               # Copy eax to edx
  addl    $1, %edx                 # Increment edx
  movl    %edx, -36(%rbp)          # Store back the incremented value
  cdqe                             # Sign-extend eax into rax
  movb    %cl, -32(%rbp, %rax)     # Store ASCII digit
  movq    -8(%rbp), %rax           # Load the stored value into rax
  movl    $10, %ecx                # Move 10 into ecx
  cqo                              # Sign-extend rax into rdx:rax
  idivq   %rcx                     # Divide rdx:rax by 10
  movq    %rax, -8(%rbp)           # Store quotient back
  jmp     .LBB0_5                  # Repeat loop
  .LBB0_7:
  jmp     .LBB0_8                  # Continue
  .LBB0_8:
  cmpl    $0, -40(%rbp)            # Check the negative flag
  je      .LBB0_10                 # Jump if zero
  movl    -36(%rbp), %eax          # Load local variable
  movl    %eax, %ecx               # Copy to ecx
  addl    $1, %ecx                 # Increment ecx
  movl    %ecx, -36(%rbp)          # Store back incremented value
  cdqe                             # Sign-extend eax into rax
  movb    $45, -32(%rbp, %rax)     # Store ASCII '-'
  .LBB0_10:
  movslq  -36(%rbp), %rax          # Sign-extend local variable to rax
  movb    $0, -32(%rbp, %rax)      # Null-terminate the string
  movl    $0, -48(%rbp)            # Initialize loop counter to 0
  movl    -36(%rbp), %eax          # Load local variable into eax
  subl    $1, %eax                 # Decrement eax
  movl    %eax, -52(%rbp)          # Store back decremented value
  .LBB0_11:
  movl    -48(%rbp), %eax          # Load loop counter into eax
  cmpl    -52(%rbp), %eax          # Compare with end index
  jge     .LBB0_13                 # Jump if greater or equal
  movslq  -48(%rbp), %rax          # Sign-extend loop counter
  movb    -32(%rbp, %rax), %al     # Load byte from string
  movb    %al, -53(%rbp)           # Store byte in temporary location
  movslq  -52(%rbp), %rax          # Sign-extend end index
  movb    -32(%rbp, %rax), %cl     # Load byte from end index
  movslq  -48(%rbp), %rax          # Sign-extend loop counter
  movb    %cl, -32(%rbp, %rax)     # Swap bytes
  movb    -53(%rbp), %cl           # Load byte from temporary location
  movslq  -52(%rbp), %rax          # Sign-extend end index
  movb    %cl, -32(%rbp, %rax)     # Swap bytes
  movl    -48(%rbp), %eax          # Load loop counter
  addl    $1, %eax                 # Increment loop counter
  movl    %eax, -48(%rbp)          # Store back incremented value
  movl    -52(%rbp), %eax          # Load end index
  subl    $1, %eax                 # Decrement end index
  movl    %eax, -52(%rbp)          # Store back decremented value
  jmp     .LBB0_11                 # Repeat loop
  .LBB0_13:
  movl    -36(%rbp), %eax          # Load local variable
  movl    %eax, %ecx               # Copy to ecx
  addl    $1, %ecx                 # Increment ecx
  movl    %ecx, -36(%rbp)          # Store back incremented value
  cdqe                             # Sign-extend eax into rax
  movb    $10, -32(%rbp, %rax)     # Store newline character
  leaq    -32(%rbp), %rsi          # Load address of string into rsi
  movslq  -36(%rbp), %rdx          # Load string length
  testb   %r15b, %r15b             # Test if r15b is zero
  jz      .not_newline             # If zero, skip newline handling
  jmp     .write                   # Otherwise, jump to write
  .not_newline:
  decq    %rdx                     # Decrement length
  .write:
  movq    $1, %rax                 # Syscall number for write
  movq    $1, %rdi                 # File descriptor 1 (stdout)
  syscall                          # Perform syscall
  addq    $64, %rsp                # Restore stack
  popq    %rbp                     # Restore base pointer
  ret                              # Return from function
