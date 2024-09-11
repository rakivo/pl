.text
.globl print_f64
print_f64:
  pushq   %rbp                     # Save base pointer
  movq    %rsp, %rbp               # Set up base pointer
  subq    $128, %rsp               # Allocate space on stack
  mov     %rsi, %r15
  movsd   %xmm0, -8(%rbp)          # Store the float in xmm0 into the stack
  movl    $0, -52(%rbp)            # Initialize a local variable to 0
  xorpd   %xmm0, %xmm0             # Zero xmm0
  ucomisd -8(%rbp), %xmm0          # Compare xmm0 with the value at [rbp-8]
  jbe     .LBB0_2                  # Jump if below or equal
  movl    -52(%rbp), %eax          # Move local variable into eax
  movl    %eax, %ecx               # Copy eax to ecx
  incl    %ecx                     # Increment ecx
  movl    %ecx, -52(%rbp)          # Update local variable
  cltq                             # Sign-extend eax into rax
  movb    $45, -48(%rbp,%rax)      # Store ASCII '-' at calculated stack location
  movsd   -8(%rbp), %xmm0          # Load the double from stack into xmm0
  movq    %xmm0, %rax              # Move the lower 64 bits of xmm0 into rax
  movq    $-9223372036854775808, %rcx   # Move constant into rcx
  xorq    %rcx, %rax               # XOR rax with rcx
  movq    %rax, %xmm0              # Move rax back into xmm0
  movsd   %xmm0, -8(%rbp)          # Store xmm0 back into the stack
  .LBB0_2:
  cvttsd2si -8(%rbp), %eax         # Convert float to integer
  movl    %eax, -56(%rbp)          # Store result in local variable
  movsd   -8(%rbp), %xmm0          # Load double from stack
  cvtsi2sd -56(%rbp), %xmm1        # Convert integer back to double
  subsd   %xmm1, %xmm0             # Subtract converted value from original double
  movsd   %xmm0, -64(%rbp)         # Store the fractional part
  cmpl    $0, -56(%rbp)            # Compare the integer value with 0
  jne     .LBB0_4                  # Jump if not equal
  movl    -52(%rbp), %eax          # Load local variable into eax
  movl    %eax, %ecx               # Copy eax to ecx
  incl    %ecx                     # Increment ecx
  movl    %ecx, -52(%rbp)          # Store the result back
  cltq                             # Sign-extend eax into rax
  movb    $48, -48(%rbp,%rax)      # Store ASCII '0' at calculated stack location
  jmp     .LBB0_12                 # Jump to the end
  .LBB0_4:
  movl    $0, -116(%rbp)           # Initialize another local variable
  .LBB0_5:
  cmpl    $0, -56(%rbp)            # Compare integer with 0
  jle     .LBB0_7                  # Jump if less or equal
  movl    -56(%rbp), %eax          # Load the integer into eax
  movl    $10, %ecx                # Move 10 into ecx
  cltd                             # Sign-extend eax into edx:eax
  idivl   %ecx                     # Divide edx:eax by 10
  movl    -116(%rbp), %eax         # Load local variable
  movl    %eax, %ecx               # Copy to ecx
  incl    %ecx                     # Increment ecx
  movl    %ecx, -116(%rbp)         # Store back the incremented value
  cltq                             # Sign-extend eax into rax
  movl    %edx, -112(%rbp,%rax,4)  # Store remainder in stack array
  movl    %eax, -56(%rbp)          # Update quotient
  jmp     .LBB0_5                  # Repeat loop
  .LBB0_7:
  movl    -116(%rbp), %eax         # Load the count
  decl    %eax                     # Decrement eax
  movl    %eax, -120(%rbp)         # Store back
  .LBB0_8:
  cmpl    $0, -120(%rbp)           # Compare the count with 0
  jl      .LBB0_11                 # Jump if less than 0
  movslq  -120(%rbp), %rax         # Sign-extend local var to rax
  movl    -112(%rbp,%rax,4), %eax  # Load the stored digit
  addl    $48, %eax                # Convert to ASCII
  movb    %al, %cl                 # Copy to cl
  movl    -52(%rbp), %eax          # Load local variable into eax
  movl    %eax, %edx               # Copy eax to edx
  incl    %edx                     # Increment edx
  movl    %edx, -52(%rbp)          # Store back the incremented value
  cltq                             # Sign-extend eax into rax
  movb    %cl, -48(%rbp,%rax)      # Store ASCII digit
  decl    -120(%rbp)               # Decrement the counter
  jmp     .LBB0_8                  # Repeat
  .LBB0_11:
  jmp     .LBB0_12                 # Continue
  .LBB0_12:
  movl    -52(%rbp), %eax          # Load local variable into eax
  movl    %eax, %ecx               # Copy eax to ecx
  incl    %ecx                     # Increment ecx
  movl    %ecx, -52(%rbp)          # Store back
  cltq                             # Sign-extend eax into rax
  movb    $46, -48(%rbp,%rax)      # Store ASCII '.'
  movl    $0, -124(%rbp)           # Initialize loop counter
  .LBB0_13:
  cmpl    $10, -124(%rbp)          # Compare loop counter with 10
  jge     .LBB0_16                 # Jump if greater or equal
  movq    $0x4024000000000000, %rax    # Load constant (10.0 as double)
  movq    %rax, %xmm0              # Move it into xmm0
  mulsd   -64(%rbp), %xmm0         # Multiply with the fractional part
  movsd   %xmm0, -64(%rbp)         # Store the result
  cvttsd2si -64(%rbp), %eax        # Convert fractional part to int
  movl    %eax, -128(%rbp)         # Store it
  movl    -128(%rbp), %eax         # Load the result
  addl    $48, %eax                # Convert to ASCII
  movb    %al, %cl                 # Copy to cl
  movl    -52(%rbp), %eax          # Load local variable
  movl    %eax, %edx               # Copy eax to edx
  incl    %edx                     # Increment edx
  movl    %edx, -52(%rbp)          # Store back the incremented value
  cltq                             # Sign-extend eax into rax
  movb    %cl, -48(%rbp,%rax)      # Store the ASCII digit
  cvtsi2sd -128(%rbp), %xmm1       # Convert integer back to double
  movsd   -64(%rbp), %xmm0         # Load fractional part
  subsd   %xmm1, %xmm0             # Subtract integer part
  movsd   %xmm0, -64(%rbp)         # Store remaining fractional part
  incl    -124(%rbp)               # Increment loop counter
  jmp     .LBB0_13                 # Repeat loop
  .LBB0_16:
  movl    -52(%rbp), %eax          # Load local variable
  movl    %eax, %ecx               # Copy to ecx
  incl    %ecx                     # Increment ecx
  movl    %ecx, -52(%rbp)          # Store back
  cltq                             # Sign-extend eax into rax
  movb    $10, -48(%rbp,%rax)      # Store newline
  leaq    -48(%rbp), %rsi          # Load address of string into rsi
  movslq  -52(%rbp), %rdx          # Load string length
  testb   %r15b, %r15b             # Test if r15b is zero
  jz      .not_newline             # If zero, skip newline handling
  jmp     .write                   # Otherwise, jump to write
  .not_newline:
  decq    %rdx                     # Decrement length
  .write:
  movq    $1, %rax                 # Syscall number for write
  movq    $1, %rdi                 # File descriptor 1 (stdout)
  syscall                          # Perform syscall
  addq    $128, %rsp               # Restore stack
  popq    %rbp                     # Restore base pointer
  ret                              # Return from function
