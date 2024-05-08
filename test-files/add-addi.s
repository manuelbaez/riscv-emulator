main:
  addi x4, x0, 3
  addi x8, x0, 4
  add x12, x4, x8
  addi x16, x12, -6
  addi gp, sp, -7
  sb a2, 0(gp) 
  lb t1, 0(gp) 
  lbu t2, 0(gp) 
  sltiu a1, t2, 8
  sltiu a5, t2, -8
  slti s3, t2, 8
  sub a0, t1, a1 
  add a5, a5, a1 
  call calc
  addi s7, x0, 1
  jalr x0, sp, -4 # End the program, for now...
  ret

notexec:
  addi s11, x0, 5

calc:
  addi t6, x0, 5
  ret
  