use std::ops::Range;

// TODO: we define REGISTER_NUM here for now, need to import it from core.
pub const REGISTER_NUM: usize = 9;

// The Olavm trace for starky:
// There are two kinds of traces, one for main trace, one for sub-table.

// 1. Main(CPU) trace.
// There are 68 columns in main trace.
//
// Context related columns(12):
// ┌───────┬───────┬──────┬───────┬───────┬───────┬───────┬───────┬───────┬───────┬───────┬───────┬
// │  clk  │   pc  │ flag │ reg_0 │ reg_1 │ reg_2 | reg_3 | reg_4 │ reg_5 | reg_6 | reg_7 │ reg_8 │
// ├───────┼───────┼──────┼───────┼───────┼───────┼───────┼───────┼───────┼───────|───────┼───────┼
// │   1   │   0   │  0   │   0   │   0   │   0   │   0   │   0   │   0   │   0   |   0   │   0   │
// └───────┴───────┴──────┴───────┴───────┴───────┴───────┴───────┴───────┴───────┴───────┴───────┴
pub(crate) const COL_CLK: usize = 0;
pub(crate) const COL_PC: usize = COL_CLK + 1;
pub(crate) const COL_FLAG: usize = COL_PC + 1;
pub(crate) const COL_START_REG: usize = COL_FLAG + 1;
pub(crate) const COL_REGS: Range<usize> = COL_START_REG..COL_START_REG + REGISTER_NUM;

// Instruction related columns(4):
// ┬────────┬─────────┬────────┬─────────┬
// │  inst  │ op1_imm │ opcode │ imm_val │
// ┼────────┼─────────┼────────┼─────────┼
// │    0   │    0    │    0   │    0    │
// ┴────────┴─────────┴────────┴─────────┴
pub(crate) const COL_INST: usize = COL_REGS.end;
pub(crate) const COL_OP1_IMM: usize = COL_INST + 1;
pub(crate) const COL_OPCODE: usize = COL_OP1_IMM + 1;
pub(crate) const COL_IMM_VAL: usize = COL_OPCODE + 1;

// Selectors of register related columns(31):
// ┬───────┬───────┬───────┬───────┬──────────┬──────────┬─────┬──────────┬──────────┬──────────┬─────┬──────────┬──────────┬──────────┬─────┬──────────┬
// │  op0  │  op1  │  dst  │  aux0 │ s_op0_r0 │ s_op0_r1 │ ... │ s_op0_r8 │ s_op1_r0 │ s_op1_r1 │ ... │ s_op1_r8 │ s_dst_r0 │ s_dst_r1 │ ... │ s_dst_r8 │
// ┼───────┼───────┼───────┼───────┼──────────┼──────────┼─────┼──────────┼──────────┼──────────┼─────┼──────────┼──────────┼──────────┼─────┼──────────┼
// │  10   │  123  │   0   │   0   │     1    │     0    │     │    0     │     0    │     1    │  0  │     0    │     1    │     0    │  0  │     0    │
// ┴───────┴───────┴───────┴───────┴──────────┴──────────┴─────┴──────────┴──────────┴──────────┴─────┴──────────┴──────────┴──────────┴─────┴──────────┴
pub(crate) const COL_OP0: usize = COL_IMM_VAL + 1;
pub(crate) const COL_OP1: usize = COL_OP0 + 1;
pub(crate) const COL_DST: usize = COL_OP1 + 1;
pub(crate) const COL_AUX0: usize = COL_DST + 1;
pub(crate) const COL_S_OP0_START: usize = COL_AUX0 + 1;
pub(crate) const COL_S_OP0: Range<usize> = COL_S_OP0_START..COL_S_OP0_START + REGISTER_NUM;
pub(crate) const COL_S_OP1_START: usize = COL_S_OP0.end;
pub(crate) const COL_S_OP1: Range<usize> = COL_S_OP1_START..COL_S_OP1_START + REGISTER_NUM;
pub(crate) const COL_S_DST_START: usize = COL_S_OP1.end;
pub(crate) const COL_S_DST: Range<usize> = COL_S_DST_START..COL_S_DST_START + REGISTER_NUM;

// Selectors of opcode related columns(12):
// ┬───────┬───────┬───────┬──────────┬───────┬───────┬────────┬────────┬───────┬─────────┬──────────┬───────┬
// │ s_add │ s_mul │  s_eq │ s_assert │ s_mov | s_jmp | s_cjmp │ s_call | s_ret | s_mload │ s_mstore │ s_end |
// ┼───────┼───────┼───────┼──────────┼───────┼───────┼────────┼────────┼───────|─────────┼──────────┼───────┼
// │   0   │   0   │   0   │     0    │   0   │   0   │    0   │    0   │   0   |     0   │     0    │   0   │
// ┴───────┴───────┴───────┴──────────┴───────┴───────┴────────┴────────┴───────┴─────────┴──────────┴───────┴
pub(crate) const COL_S_ADD: usize = COL_S_DST.end;
pub(crate) const COL_S_MUL: usize = COL_S_ADD + 1;
pub(crate) const COL_S_EQ: usize = COL_S_MUL + 1;
pub(crate) const COL_S_ASSERT: usize = COL_S_EQ + 1;
pub(crate) const COL_S_MOV: usize = COL_S_ASSERT + 1;
pub(crate) const COL_S_JMP: usize = COL_S_MOV + 1;
pub(crate) const COL_S_CJMP: usize = COL_S_JMP + 1;
pub(crate) const COL_S_CALL: usize = COL_S_CJMP + 1;
pub(crate) const COL_S_RET: usize = COL_S_CALL + 1;
pub(crate) const COL_S_MLOAD: usize = COL_S_RET + 1;
pub(crate) const COL_S_MSTORE: usize = COL_S_MLOAD + 1;
pub(crate) const COL_S_END: usize = COL_S_MSTORE + 1;

// Selectors of Builtins related columns(9):
// ┬───────┬───────┬───────┬───────┬───────┬───────┬───────┬────────────┬─────────┐
// │  s_rc │ s_and │ s_or  │ s_xor │ s_not │ s_neq │ s_gte │ s_poseidon │ s_ecdsa |
// ┼───────┼───────┼───────┼───────┼───────┼───────┼───────┼────────────┼─────────|
// │   0   │   1   │   0   │   0   │   0   │   0   │   0   │      0     │    0    |
// ┴───────┴───────┴───────┴───────┴───────┴───────┴───────┴────────────┴─────────┘
pub(crate) const COL_S_RC: usize = COL_S_END + 1;
pub(crate) const COL_S_AND: usize = COL_S_RC + 1;
pub(crate) const COL_S_OR: usize = COL_S_AND + 1;
pub(crate) const COL_S_XOR: usize = COL_S_OR + 1;
pub(crate) const COL_S_NOT: usize = COL_S_XOR + 1;
pub(crate) const COL_S_NEQ: usize = COL_S_NOT + 1;
pub(crate) const COL_S_GTE: usize = COL_S_NEQ + 1;
pub(crate) const COL_S_PSDN: usize = COL_S_GTE + 1;
pub(crate) const COL_S_ECDSA: usize = COL_S_PSDN + 1;

pub(crate) const NUM_CPU_COLS: usize = COL_S_ECDSA + 1;

// 2. Memory Trace.
// ┌─────────────┬───────────────┬──────┬───────┬───────┬──────────┬───────┬───────────┬──────────┬─────────────┬──────┬───────┬
// │ end_rw_diff │ is_write_once │ addr │  clk  │   op  │ is_write | value | diff_addr │ diff_clk | diff_addr_p | f_rc │ f_main│
// ├─────────────┼───────────────┼──────┼───────┼───────┼──────────┼───────┼───────────┼──────────┼─────────────|──────┼───────┼
// │      0      │       0       │  0   │   0   │   0   │     0    │   0   │     0     │     0    │      0      |   0  │   0   │
// └─────────────┴───────────────┴──────┴───────┴───────┴──────────┴───────┴───────────┴──────────┴─────────────┴──────┴───────┴
pub(crate) const COL_END_RW_DIFF: usize = 0;
pub(crate) const COL_IS_WRITE_ONCE: usize = COL_END_RW_DIFF + 1;
pub(crate) const COL_ADDR: usize = COL_IS_WRITE_ONCE + 1;
pub(crate) const COL_RAM_CLK: usize = COL_ADDR + 1;
pub(crate) const COL_RAM_OP: usize = COL_RAM_CLK + 1;
pub(crate) const COL_IS_WRITE: usize = COL_RAM_OP + 1;
pub(crate) const COL_VALUE: usize = COL_IS_WRITE + 1;
pub(crate) const COL_DIFF_ADDR: usize = COL_VALUE + 1;
pub(crate) const COL_DIFF_CLK: usize = COL_DIFF_ADDR + 1;
pub(crate) const COL_DIFF_ADDR_P: usize = COL_DIFF_CLK + 1;
pub(crate) const COL_F_RC: usize = COL_DIFF_ADDR_P + 1;
pub(crate) const COL_F_MAIN: usize = COL_F_RC + 1;

pub(crate) const NUM_RAM_COLS: usize = COL_F_MAIN + 1;