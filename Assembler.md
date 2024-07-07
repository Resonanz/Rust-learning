### Pass-by-VALUE to function
```
pub fn main() {
    let val = 123;
    let _res = calculate(val);
}

pub fn calculate(val: u32) -> u32 {
    val + 3
}


playground::main:
	pushq	%rax
	movl	$123, (%rsp)
	movl	$123, %edi
	callq	playground::calculate
	movl	%eax, 4(%rsp)
	popq	%rax
	retq

playground::calculate:
	pushq	%rax
	movl	%edi, 4(%rsp)
	addl	$3, %edi
	movl	%edi, (%rsp)
	setb	%al
	testb	$1, %al
	jne	    .LBB9_2
	movl	(%rsp), %eax
	popq	%rcx
	retq
```

### Pass-by-REFERENCE to function
```
pub fn main() {
    let val = 123;
    let _res = calculate(&val);
}

pub fn calculate(val: &u32) -> u32 {
    val + 3
}


playground::main:
	subq	$24, %rsp
	movl	$123, 12(%rsp)
	movl	$123, 16(%rsp)
	leaq	16(%rsp), %rdi
	callq	playground::calculate
	movl	%eax, 20(%rsp)
	addq	$24, %rsp
	retq

playground::calculate:
	pushq	%rax
	movq	%rdi, (%rsp)
	movl	$3, %esi
	leaq	.L__unnamed_2(%rip), %rdx
	callq	<&u32 as core::ops::arith::Add<u32>>::add
	popq	%rcx
	retq
```

### Pass-by-REFERENCE to function, then increment using pointer
```
pub fn main() {
    let val = 123;
    let _res = calculate(&val);
}

pub fn calculate(val: &u32) -> u32 {
    let y = *val + 7;
    y
}


playground::main:
	subq	$24, %rsp
	movl	$123, 12(%rsp)
	movl	$123, 16(%rsp)
	leaq	16(%rsp), %rdi
	callq	playground::calculate
	movl	%eax, 20(%rsp)
	addq	$24, %rsp
	retq

playground::calculate:
	subq	$24, %rsp
	movq	%rdi, 8(%rsp)
	movl	(%rdi), %eax
	addl	$7, %eax
	movl	%eax, 4(%rsp)
	setb	%al
	testb	$1, %al
	jne	.LBB9_2
	movl	4(%rsp), %eax
	movl	%eax, 20(%rsp)
	addq	$24, %rsp
	retq
```

### Pass-by-REFERENCE to function, then increment WITHOUT using pointer
```
pub fn main() {
    let val = 123;
    let _res = calculate(&val);
}

pub fn calculate(val: &u32) -> u32 {
    let y = val + 7;
    y
}


playground::main:
	subq	$24, %rsp
	movl	$123, 12(%rsp)
	movl	$123, 16(%rsp)
	leaq	16(%rsp), %rdi
	callq	playground::calculate
	movl	%eax, 20(%rsp)
	addq	$24, %rsp
	retq

playground::calculate:
	subq	$24, %rsp
	movq	%rdi, 8(%rsp)
	movl	$7, %esi
	leaq	.L__unnamed_2(%rip), %rdx
	callq	<&u32 as core::ops::arith::Add<u32>>::add
	movl	%eax, 20(%rsp)
	addq	$24, %rsp
	retq
```