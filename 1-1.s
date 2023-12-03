.equ SYS_READ, 63
.equ SYS_EXIT, 94
.equ SYS_WRITE, 64
.equ STDIN_FILENO, 0
.equ STDOUT_FILENO, 1

.align 2
.global _start

_start:
	addi sp, sp, -16
	li s2, 0

new_line:
	li s0, -1

line_loop:
	li a0, STDIN_FILENO
	mv a1, sp
	li a2, 1
	li a7, SYS_READ
	ecall
	beqz a0, print

	lbu t0, 0(sp)

	li t1, '0'
	bltu t0, t1, not_digit
	li t1, '9'
	bgtu t0, t1, not_digit

	bgez s0, no_store_first_digit
	mv s0, t0
no_store_first_digit:
	mv s1, t0
	j line_loop

not_digit:
	li t1, '\n'
	bne t0, t1, line_loop

sum:
	addi s0, s0, -'0'
	addi s1, s1, -'0'
	li t0, 10
	mul s0, s0, t0
	add s0, s0, s1
	add s2, s2, s0

	j new_line

print:
	addi t0, sp, 14
	li t1, '\n'
	sb t1, 1(t0)
	li t1, 1
	li t2, 10

print_loop:
	remu t3, s2, t2
	addi t3, t3, '0'
	sb t3, 0(t0)
	addi t0, t0, -1
	addi t1, t1, 1

	divu s2, s2, t2
	bgtz s2, print_loop

	li a0, STDOUT_FILENO
	addi a1, t0, 1
	mv a2, t1
	li a7, SYS_WRITE
	ecall

	li a0, 0
	li a7, SYS_EXIT
	ecall

.data
