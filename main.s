.data
	word: .word 10

.text
.globl __start

__start:
	lw $s0, word
	addi $t1, $s0, 1
	sw $t1, word
	nop
	j __start

	# Exit
	li $v0, 10
	syscall
