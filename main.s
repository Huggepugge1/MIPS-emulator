.data
	word: .word 100

.text
.globl __start

__start:
	# Add two numbers
	li $t0, 5
	li $t1, 10
	add $t2, $t0, $t1

	# Subtract two numbers
	li $t0, 15
	li $t1, 10
	sub $t2, $t0, $t1

	# Multiply two numbers
	li $t0, 5
	li $t1, 10
	mul $t2, $t0, $t1

	# Divide two numbers
	li $t0, 5
	li $t1, 2
	div $t0, $t1

	# Load immediate
	li $t0, 5

	# Load address
	la $t0, word

	# Store word
	li $t0, 5
	sw $t0, word

	# Load word
	lw $t0, word

	# Exit
	li $v0, 10
	syscall
