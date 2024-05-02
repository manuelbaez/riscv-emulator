main:
  addi x4, x0, 3
  addi x8, x0, 4
  add x12, x4, x8
  addi x16, x12, -6
  addi gp, sp, -7
  sb a2, 0(gp) 
  lb t1, 0(gp) 
  lbu t2, 0(gp) 
  