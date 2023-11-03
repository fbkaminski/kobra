// represents an external native reference
struct ExternalReference {
    address: *const u8
}

enum Immediate {
    Number(i32),
    String(&str),
    Reference(ExternalReference)
}

// x86 registers
enum Register {
    None,
    Rax,
    Rcx,
    Rdx,
    Rbx,
    Rsp,
    Rbp,
    Rsi,
    Rdi
}

// x86 xmm registers
enum XmmRegister {
    Xmm0,
    Xmm1,
    Xmm2,
    Xmm3,
    Xmm4,
    Xmm5,
    Xmm6,
    Xmm7
}

enum Condition {
    Overflow,
    NoOverflow,
    Below,
    AboveEqual,
    Equal,
    NotEqual,
    BelowEqual,
    Above,
    Sign,
    NotSign,
    ParityEven,
    ParityOdd,
    Less,
    GreaterEqual,
    LessEqual,
    Greater,
    // aliases
    Zero  = Equal,
    NotZero = NotEqual,
    Negative = Sign,
    Positive = NotSign
}

enum Operand {

}

enum CpuFeatures {
    SSE2,
    CPUID
}

enum OpcodeArguments {
    RegAndOp(Register, Operand),
    OpAndImm(Operand, Immediate)
}

struct Assembler {
    buffer: *const u8,
    size: usize
}

impl Assembler {
    
    pub fn new(buffer: *const u8, size: usize) -> Self {
        return Assembler {
            buffer: buffer,
            size: size
        }
    }

    pub fn add(&mut self, dst: Register, src: Operand) {

    }
        
    pub fn add(&mut self, dst: Operand, x: Immediate) {
        
    }
    
    pub fn or(&mut self, dst: Register, imm: i32) {

    }
    
    pub fn or(&mut self, dst: Register, src: Operand) {

    }
    
    pub fn or(&mut self, dst: Operand, src: Register) {

    }
    
    pub fn or(dst: Operand, x: Immediate) {

    }

    pub fn adc(dst: Register, imm: i32) {

    }
    
    pub fn adc(dst: Register, src: Operand) {
        
    }

    pub fn sbb(dst: Register, src: Operand) {

    }

    pub fn and(dst: Register, imm: i32) {

    }
    
    pub fn and(dst: Register, src: Operand) {

    }
    
    pub fn and(src: Operand, dst: Register) {

    }
    
    pub fn and(dst: Operand, x: Immediate) {

    }

    // fixme: fill
    pub fn daa() {

    }

    pub fn sub(dst: Operand, x: Immediate) {

    }

    pub fn sub(dst: Register, src: Operand) {

    }
    
    pub fn sub(dst: Operand, src: Register) {

    }

    pub fn das() {

    }

    pub fn xor(dst: Register, imm: i32) {

    }
    
    pub fn xor(dst: Register, src: Operand) {

    }
    
    pub fn xor(src: Operand, dst: Register) {

    }
    
    pub fn xor(dst: Operand, x: Immediate) {
        
    }
    
    // fixme
    pub fn aaa() {
        
    }

    pub fn cmp(reg: Register, imm: i32) {

    }

    pub fn cmp(reg: Register, op: Operand) {

    }

    pub fn cmp(op: Operand, imm: Immediate) {

    }

    // fixme
    pub fn aas() {
        
    }

    pub fn inc(dst: Register) {

    }
    
    pub fn inc(dst: Operand) {

    }

    pub fn dec(dst: Register) {

    }
    
    pub fn dec(dst: Operand) {

    }

    pub fn dec_b(dst: Register) {

    }

    pub fn push(im: Immediate) {
        
    }

    pub fn push(reg: Register) {

    }

    pub fn push(op: Operand) {

    }

    pub fn pusha() {
        
    }

    pub fn pop(reg: Register) {

    }

    pub fn pop(op: Operand) {

    }
    
    pub fn popa() {
        // emit(0x61);
    }

    pub fn bound() {

    }

    pub fn imul(dst: Register, src: Operand) {

    }
    
    pub fn imul(dst: Register, src: Register, imm: i32) {

    }

    pub fn arpl() {

    }

    pub fn jo() {

    }

    pub fn jno() {

    }

    pub fn jb() {

    }

    pub fn jnb() {

    }

    pub fn jz() {

    }

    pub fn jnz() {

    }

    pub fn jbe() {

    }

    pub fn jnbe() {

    }

    pub fn js() {

    }

    pub fn jns() {

    }

    pub fn jp() {

    }

    pub fn jnp() {

    }

    pub fn jl() {

    }

    pub fn jnl() {

    }

    pub fn jle() {

    }

    pub fn jnle() {

    }

    pub fn jmp() {

    }

    pub fn call() {

    }

    pub fn loopne() {

    }

    pub fn loope() {

    }

    pub fn loop_() {

    }

    pub fn jecxz() {

    }
    
    pub fn test(reg: Register, imm: Immediate) {

    }
    
    pub fn test(reg: Register, op: Operand) {

    }
    
    pub fn test(op: Operand, imm: Immediate) {

    }

    pub fn lea(dst: Register, src: Operand) {

    }

    pub fn xchg() {

    }

    pub fn cwde() {

    }

    pub fn cdq() {

    }

    pub fn fwait() {
        
    }

    pub fn pushf() {

    }

    pub fn popf() {

    }

    pub fn sahf() {

    }

    pub fn lahf() {

    }
    
    pub fn ret(imm: i32) {

    }

    pub fn les() {

    }

    pub fn lds() {

    }

    pub fn enter() {

    }

    pub fn leave() {

    }

    pub fn int3() {

    }

    pub fn int() {

    }

    pub fn into() {

    }

    pub fn iret() {

    }

    pub fn aam() {

    }

    pub fn aad() {

    }

    pub fn xlat() {

    }

    pub fn in_() {

    }

    pub fn out() {

    }

    pub fn hlt() {

    }

    pub fn cmc() {

    }

    pub fn clc() {

    }

    pub fn stc() {

    }

    pub fn cli() {

    }

    pub fn sti() {

    }

    pub fn cld() {

    }

    pub fn std() {

    }

    pub fn lar() {

    }

    pub fn lsl() {

    }

    pub fn syscall() {

    }

    pub fn clts() {

    }

    pub fn sysret() {

    }

    pub fn invd() {

    }

    pub fn wbinvd() {

    }

    pub fn ud2() {

    }

    pub fn nop_modrm() {

    }

    pub fn movntps() {
        
    } 

    pub fn movntpd() {

    }

    pub fn wrmsr() {

    }

    pub fn rdtsc() {

    }

    pub fn rdtsc() {

    }

    pub fn rdmsr() {

    }

    pub fn rdpmc() {

    }

    pub fn sysenter() {

    }

    pub fn sysexit() {
        
    }

    pub fn cmovo() {

    }

    pub fn push_ad() {
        // fixme: have proper enum for the instructions
        // and emit that instead
        emit(0x60);
    }

    pub fn push_fd() {

    }

    pub fn cmov(cc: Condition, dst: Register, imm32: i32) {

    }

    pub fn cmov(cc: Condition, dst: Register, src: Operand) {

    }

    pub fn cdq() {

    }

    pub fn idiv(src: Register) {

    }

    pub fn mul(src: Register) {

    }

    pub fn neg(dst: Register) {

    }

    pub fn not(dst: Register) {

    }

    pub fn rcl(dst: Register, imm: u8) {

    }

    pub fn sar(dst: Register, imm: u8) {

    }
    
    pub fn sar(dst: Register) {

    }

    pub fn shld(dst: Register, src: Operand) {

    }

    pub fn shl(dst: Register, imm: u8) {

    }
    
    pub fn shl(dst: Register) {

    }

    pub fn shrd(dst: Register, src: Operand) {

    }

    pub fn shr(dst: Register, imm: u8) {

    }
    
    pub fn shr(dst: Register) {

    }

    pub fn bts(dst: Operand, src: Register) {

    }

    pub fn hlt() {

    }
    
    pub fn int3() {

    }
    
    pub fn nop() {

    }
    
    pub fn rdtsc() {

    }
    
    pub fn movno() {

    }

    pub fn cmovb() {

    }

    pub fn cmovnb() {

    }

    pub fn cmovz() {

    }
    
    pub fn cmovnz() {

    }
    
    pub fn cmovbe() {

    }
    
    pub fn cmovnbe() {

    }
    
    pub fn cmovs() {

    }
    
    pub fn cmovns() {

    }
    
    pub fn cmovp() {

    }
    
    pub fn cmovnp() {

    }
    
    pub fn cmovl() {

    }
    
    pub fn cmovnl() {

    }
    
    pub fn cmovle() {

    }
    
    pub fn cmovnle() {

    }
    
    pub fn punpcklbw() {

    }

    pub fn punpcklwd() {

    }

    pub fn punpckldq() {

    }

    pub fn packsswb() {

    }

    pub fn pcmpgtb() {

    }
    
    pub fn pcmpgtw() {

    }
    
    pub fn pcmpgtd() {

    }

    pub fn packuswb() {

    }

    pub fn punpckhbw() {

    }
    
    pub fn punpckhwd() {

    }
    
    pub fn punpckhdq() {

    }
    
    pub fn packssdw() {

    }
    
    pub fn punpcklqdq() {

    }
    
    pub fn punpckhqdq() {

    }
    
    pub fn movd() {

    }
    
    pub fn movq() {

    }
    
    pub fn movdqu() {

    }
    
    pub fn movdqa() {

    }
    
    pub fn pshufw() {

    }
    
    pub fn pshufd() {

    }
    
    pub fn pshufhw() {

    }
    
    pub fn pshuflw() {

    }

    pub fn pcmpeqb() {

    }
    
    pub fn pcmpeqw() {

    }
    
    pub fn pcmpeqd() {

    }
    
    pub fn emms() {

    }
    
    pub fn seto() {

    }
    
    pub fn setno() {

    }
    
    pub fn setb() {

    }
    
    pub fn setnb() {

    }
    
    pub fn setz() {

    }
    
    pub fn setnz() {

    }
    
    pub fn setbe() {

    }
    
    pub fn setnbe() {

    }
    
    pub fn sets() {

    }
    
    pub fn setns() {

    }
    
    pub fn setp() {

    }
    
    pub fn setnp() {

    }
    
    pub fn setl() {

    }
    
    pub fn setnl() {

    }
    
    pub fn setle() {

    }
    
    pub fn setnle() {

    }
    
    pub fn cpuid() {
        emit(0x0F);
        emit(0xA2);
    }

    pub fn bt() {

    }
    
    pub fn shld() {

    }
    
    pub fn rsm() {

    }
    
    pub fn bts() {

    }
    
    pub fn shrd() {

    }

    pub fn cmpxchg() {

    }
    
    pub fn lss() {

    }
    
    pub fn btr() {

    }
    
    pub fn lfs() {

    }
    
    pub fn lgs() {

    }
    
    pub fn movzx() {

    }
    
    pub fn ud1() {

    }
    
    pub fn btc() {

    }
    
    pub fn bsf() {

    }
    
    pub fn bsr() {

    }
    
    pub fn movsx() {

    }
    
    pub fn xadd() {

    }
    
    pub fn movnti() {

    }
    
    pub fn pinsrw() {

    }
    
    pub fn pextrw() {

    }
    
    pub fn bswap() {

    }
    
    pub fn psrlw() {

    }
    
    pub fn psrld() {

    }
    
    pub fn psrlq() {

    }
    
    pub fn paddq() {

    }
    
    pub fn pmullw() {

    }
    
    pub fn pmovmskb() {

    }
    
    pub fn psubusb() {

    }

    pub fn psubusw() {

    }
    
    pub fn pminub() {

    }
    
    pub fn pand() {

    }
    
    pub fn paddusb() {

    }
    
    pub fn paddusw() {

    }
    
    pub fn pmaxub() {

    }
    
    pub fn pandn() {

    }

    pub fn pavgb() {

    }
    
    pub fn psraw() {

    }
    
    pub fn psrad() {

    }
    
    pub fn pavgw() {

    }
    
    pub fn pmulhuw() {

    }
    
    pub fn pmulhw() {

    }
    
    pub fn movntq() {

    }
    
    pub fn movntdq() {

    }
    
    pub fn psubsb() {

    }
    
    pub fn psubsw() {

    }
    
    pub fn pminsw() {

    }

    pub fn por() {

    }
    
    pub fn paddsb() {

    }
    
    pub fn paddsw() {

    }
    
    pub fn pmaxsw() {

    }
    
    pub fn pxor() {

    }
    
    pub fn psllw() {

    }
    
    pub fn pslld() {

    }
    
    pub fn psllq() {

    }
    
    pub fn pmuludq() {

    }
    
    pub fn pmaddwd() {

    }
    
    pub fn psadbw() {

    }
    
    pub fn maskmovq() {

    }
    
    pub fn maskmovdqu() {

    }
    
    pub fn psubb() {

    }
    
    pub fn psubw() {

    }
    
    pub fn psubd() {

    }
    
    pub fn psubq() {

    }
    
    pub fn paddb() {

    }
    
    pub fn paddw() {

    }
    
    pub fn paddd() {

    }
    
    pub fn psrldq() {

    }
    
    pub fn pslldq() {

    }
    
    pub fn rol() {

    }
    
    pub fn ror() {

    }
    
    pub fn rcl() {

    }
    
    pub fn rcr() {

    }
    
    pub fn shl() {

    }
    
    pub fn shr() {

    }
   
    pub fn sar() {

    }

    pub fn not() {

    }
    
    pub fn neg() {

    }
    
    pub fn mul() {

    }
    
    pub fn div() {

    }
    
    pub fn sldt() {

    }
    
    pub fn str_() {

    }
    
    pub fn lldt() {

    }
    
    pub fn ltr() {

    }
    
    pub fn verr() {

    }
    
    pub fn verw() {

    }
    
    pub fn sgdt() {

    }

    pub fn sidt() {

    }
    
    pub fn lgdt() {

    }
    
    pub fn lidt() {

    }
    
    pub fn smsw() {

    }
    
    pub fn lmsw() {

    }
    
    pub fn invlpg() {

    }
    
    pub fn cmpxchg8b() {

    }
    
    pub fn fxsave32() {

    }
    
    pub fn fxrstor32() {

    }
    
    pub fn ldmxcsr() {

    }
    
    pub fn stmxcsr() {

    }
    
    pub fn lfence() {

    }
    
    pub fn mfence() {

    }
    
    pub fn clflush() {

    }

    pub fn sfence() {

    }
    
    pub fn prefetchnta() {

    }
    
    pub fn prefetcht0() {

    }
    
    pub fn prefetcht1() {

    }
    
    pub fn prefetcht2() {

    }
    
    pub fn prefetch() {

    }
    
    pub fn prefetchw() {

    }
    
    pub fn movups() {

    }
    
    pub fn movss() {

    }
    
    pub fn movupd() {

    }
    
    pub fn movsd() {

    }
    
    pub fn movlps() {

    }
    
    pub fn movlpd() {

    }
    
    pub fn unpcklps() {

    }
    
    pub fn unpcklpd() {

    }
    
    pub fn unpckhps() {

    }
    
    pub fn unpckhpd() {

    }
    
    pub fn movhps() {

    }
    
    pub fn movhpd() {

    }
    
    pub fn movaps() {

    }
    
    pub fn movapd() {

    }
    
    pub fn cvtpi2ps() {

    }
    
    pub fn cvtsi2ss() {

    }
    
    pub fn cvtpi2pd() {

    }
    
    pub fn cvtsi2sd() {

    }
    
    pub fn cvttps2pi() {

    }
    
    pub fn cvttss2si() {

    }
    
    pub fn cvttpd2pi() {

    }
    
    pub fn cvttsd2si() {

    }
    
    pub fn cvtps2pi() {

    }
    
    pub fn cvtss2si() {

    }
    
    pub fn cvtpd2pi() {

    }
    
    pub fn cvtsd2si() {

    }
    
    pub fn ucomiss() {

    }
    
    pub fn ucomisd() {

    }
    
    pub fn comiss() {

    }
    
    pub fn comisd() {

    }
    
    pub fn movmskps() {

    }
    
    pub fn movmskpd() {

    }
    
    pub fn sqrtps() {

    }
    
    pub fn sqrtss() {

    }
    
    pub fn sqrtpd() {

    }
    
    pub fn sqrtsd() {

    }
    
    pub fn rsqrtps() {

    }

    pub fn rsqrtss() {

    }

    pub fn rcpps() {

    }

    pub fn rcpss() {

    }

    pub fn andps() {

    }

    pub fn andpd() {

    }

    pub fn andnps() {

    }

    pub fn andnpd() {

    }

    pub fn orps() {

    }

    pub fn orpd() {

    }

    pub fn xorps() {

    }

    pub fn xorpd() {

    }

    pub fn addps() {

    }

    pub fn addss() {

    }

    pub fn addpd() {

    }

    pub fn addsd() {

    }

    pub fn mulps() {

    }

    pub fn mulss() {

    }
    
    pub fn mulpd() {

    }

    pub fn mulsd() {

    }

    pub fn cvtps2pd() {

    }

    pub fn cvtss2sd() {

    }

    pub fn cvtpd2ps() {

    }

    pub fn cvtsd2ss() {

    }

    pub fn cvtdq2ps() {

    }

    pub fn cvttps2dq() {

    }

    pub fn cvtps2dq() {

    }

    pub fn subps() {

    }

    pub fn subss() {

    }
    
    pub fn subpd() {

    }

    pub fn subsd() {

    }

    pub fn minps() {

    }

    pub fn minss() {

    }

    pub fn minpd() {

    }

    pub fn minsd() {

    }

    pub fn divps() {

    }

    pub fn divss() {

    }

    pub fn divpd() {

    }

    pub fn divsd() {

    }

    pub fn maxps() {

    }

    pub fn maxss() {

    }

    pub fn maxpd() {

    }

    pub fn maxsd() {

    }

    pub fn cmpps() {

    }

    pub fn cmpss() {

    }

    pub fn cmppd() {

    }

    pub fn cmpsd() {

    }

    pub fn shufps() {

    }

    pub fn shufpd() {

    }

    pub fn cvtdq2pd() {

    }

    pub fn cvttpd2dq() {

    }

    pub fn cvtpd2dq() {

    }

    pub fn pause() {

    }

    pub fn ins() {

    }

    pub fn rep_ins() {

    }

    pub fn outs() {

    }

    pub fn rep_outs() {

    }

    pub fn movs() {

    }
    
    pub fn rep_movs() {

    }

    pub fn stos() {

    }

    pub fn rep_stos() {

    }

    pub fn lods() {

    }

    pub fn rep_lods() {

    }

    pub fn cmps() {

    }

    pub fn rep_cmps() {

    }

    pub fn repne_cmps() {

    }

    pub fn scas() {

    }

    pub fn rep_scas() {

    }

    pub fn repne_scas() {

    }

    pub fn fadd() {

    }

    pub fn fmul() {

    }

    pub fn fcom() {

    }

    pub fn fcomp() {

    }

    pub fn fsub() {

    }

    pub fn fsubr() {

    }

    pub fn fdiv() {

    }

    pub fn fdivr() {

    }
    
    pub fn fld() {

    }

    pub fn fst() {

    }

    pub fn fstp() {

    }
    
    pub fn fldenv() {

    }

    pub fn fldcw() {

    }

    pub fn fnstenv() {

    }

    pub fn fnstcw() {

    }

    pub fn fiadd() {

    }

    pub fn fimul() {

    }

    pub fn ficom() {

    }

    pub fn ficomp() {

    }
    
    pub fn fisub() {

    }
    
    pub fn fisubr() {

    }
    
    pub fn fidiv() {

    }
    
    pub fn fidivr() {

    }
    
    pub fn fild() {

    }
    
    pub fn fist() {

    }
    
    pub fn fistp() {

    }
    
    pub fn frstor() {

    }
    
    pub fn fnsave() {

    }
    
    pub fn fnstsw() {

    }
    
    pub fn fbld() {

    }
    
    pub fn fbstp() {

    }
    
    pub fn fxch() {

    }
    
    pub fn fnop() {

    }
    
    pub fn fchs() {

    }
    
    pub fn fabs() {

    }
    
    pub fn ftst() {

    }
    
    pub fn fxam() {

    }
    
    pub fn fld1() {

    }
    
    pub fn fldl2t() {

    }
    
    pub fn fldl2e() {

    }
    
    pub fn fldpi() {

    }
    
    pub fn fldlg2() {

    }
    
    pub fn fldln2() {

    }
    
    pub fn fldz() {

    }
    
    pub fn f2xm1() {

    }
    
    pub fn fyl2x() {

    }
    
    pub fn fptan() {

    }
    
    pub fn fpatan() {

    }
    
    pub fn fxtract() {

    }
    
    pub fn fprem1() {

    }
    
    pub fn fdecstp() {

    }
    
    pub fn fincstp() {

    }
    
    pub fn fprem() {

    }
    
    pub fn fyl2xp1() {

    }
    
    pub fn fsqrt() {

    }
    
    pub fn fsincos() {

    }
    
    pub fn frndint() {

    }
    
    pub fn fscale() {

    }
    
    pub fn fsin() {

    }
    
    pub fn fcos() {

    }
    
    pub fn fcmovb() {

    }
    
    pub fn fcmove() {

    }
    
    pub fn fcmovbe() {

    }
    
    pub fn fcmovu() {

    }
    
    pub fn fucompp() {

    }
    
    pub fn fcmovnb() {

    }
    
    pub fn fcmovne() {

    }
    
    pub fn fcmovnbe() {

    }
    
    pub fn fcmovnu() {

    }
    
    pub fn fnclex() {

    }
    
    pub fn fninit() {

    }
    
    pub fn fucomi() {

    }
    
    pub fn fcomi() {

    }
    
    pub fn ffree() {

    }
    
    pub fn fucom() {

    }
    
    pub fn fucomp() {

    }
    
    pub fn faddp() {

    }
    
    pub fn fmulp() {

    }
    
    pub fn fcompp() {

    }
    
    pub fn fsubrp() {

    }
    
    pub fn fsubp() {

    }
    
    pub fn fdivrp() {

    } 
    
    pub fn fdivp() {

    }
    
    pub fn fucomip() {

    }
    
    pub fn fcomip() {

    }

    // SSE3
    pub fn fisttp() {

    }

    pub fn haddpd() {

    }

    pub fn haddps() {

    }

    pub fn hsubpd() {

    }

    pub fn hsubps() {

    }

    pub fn addsubpd() {

    }

    pub fn addsubps() {

    }

    pub fn lddqu() {

    }

    pub fn monitor() {

    }

    pub fn mwait() {

    }

    pub fn movsldup() {

    }

    pub fn movshdup() {

    }

    pub fn movddup() {

    }
    
    // 3D-Now!
    pub fn femms() {

    }

    pub fn unknown_3dnow() {

    }

    pub fn pavgusb() {

    }
    
    pub fn pfadd() {

    }
    
    pub fn pfacc() {

    }
    
    pub fn pfcmpge() {

    }
    
    pub fn pfcmpgt() {

    }
    
    pub fn pfcmpeq() {

    }
    
    pub fn pfmin() {

    }
    
    pub fn pfmax() {

    }
    
    pub fn pfmul() {

    }
    
    pub fn pfrcp() {

    }
    
    pub fn pfrcpit1() {

    }
    
    pub fn pfrcpit2() {

    }
    
    pub fn pfrsqrt() {

    }
    
    pub fn pfrsqit1() {

    }
    
    pub fn pmulhrw() {

    }
    
    pub fn pfsub() {

    }
    
    pub fn pfsubr() {

    }
    

    pub fn pi2fd() {

    }
    
    pub fn pf2id() {

    }
    
    pub fn pi2fw() {

    }
    
    pub fn pf2iw() {

    }
    
    pub fn pfnacc() {

    }
    
    pub fn pfpnacc() {

    }
    
    pub fn pswapd() {

    }
    
    /* SSSE3 */
    pub fn pshufb() {

    }
    
    pub fn phaddw() {

    }
    
    pub fn phaddd() {

    }
    
    pub fn phaddsw() {

    }
    
    pub fn pmaddubsw() {

    }
    
    pub fn phsubw() {

    }
    
    pub fn phsubd() {

    }
    
    pub fn phsubsw() {

    }
    
    pub fn psignb() {

    }
    
    pub fn psignw() {

    }
    
    pub fn psignd() {

    }
    
    pub fn pmulhrsw() {

    }
    
    pub fn pabsb() {

    }
    
    pub fn pabsw() {

    }
    
    pub fn pabsd() {

    }
    
    pub fn palignr() {

    }
    
    // SSE4
    pub fn popcnt() {

    }
    
    pub fn movntss() {

    }
    
    pub fn movntsd() {

    }
    
    pub fn extrq() {

    }
    
    pub fn insertq() {

    }
    
    pub fn lzcnt() {

    }
    
    pub fn pblendvb() {

    }
    
    pub fn blendvps() {

    }
    
    pub fn blendvpd() {

    }
    
    pub fn ptest() {

    }
    
    pub fn pmovsxbw() {

    }
    
    pub fn pmovsxbd() {

    }
    
    pub fn pmovsxbq() {

    }
    
    pub fn pmovsxwd() {

    }
    
    pub fn pmovsxwq() {

    }
    
    pub fn pmovsxdq() {

    }
    
    pub fn pmuldq() {

    }
    
    pub fn pcmpeqq() {

    }
    
    pub fn movntdqa() {

    }
    
    pub fn packusdw() {

    }
    
    pub fn pmovzxbw() {

    }
    
    pub fn pmovzxbd() {

    }
    
    pub fn pmovzxbq() {

    }
    
    pub fn pmovzxwd() {

    }
    
    pub fn pmovzxwq() {

    }
    
    pub fn pmovzxdq() {

    }
    
    pub fn pcmpgtq() {

    }
    
    pub fn pminsb() {

    }
    
    pub fn pminsd() {

    }
    
    pub fn pminuw() {

    }
    
    pub fn pminud() {

    }
    
    pub fn pmaxsb() {

    }
    
    pub fn pmaxsd() {

    }
    
    pub fn pmaxuw() {

    }
    
    pub fn pmaxud() {

    }
    
    pub fn pmulld() {

    }
    
    pub fn phminposuw() {

    }
    
    pub fn crc32() {

    }
    
    pub fn pextrb() {

    }
    
    pub fn pextrd() {

    }
    
    pub fn extractps() {

    }
    
    pub fn roundps() {

    }
    
    pub fn roundpd() {

    }
    
    pub fn roundss() {

    }
    
    pub fn roundsd() {

    }
    
    pub fn blendps() {

    }
    
    pub fn blendpd() {

    }
    
    pub fn pblendw() {

    }

    pub fn pinsrb() {

    }
    
    pub fn insertps() {

    }
    
    pub fn pinsrd() {

    }
    
    pub fn dpps() {

    }
    
    pub fn dppd() {

    }
    
    pub fn mpsadbw() {

    }
    
    pub fn pcmpestrm() {

    }
    
    pub fn pcmpestri() {

    }
    
    pub fn pcmpistrm() {

    }
    
    pub fn pcmpistri() {

    }
        
    // x64
    pub fn movsxd() {

    }
    
    pub fn swapgs() {

    }
        
    // VMX
    pub fn vmcall() {

    }
    
    pub fn vmlaunch() {

    }
    
    pub fn vmresume() {

    }
    
    pub fn vmxoff() {

    }
    
    pub fn vmptrst() {

    }
    
    pub fn vmptrld() {

    }
    
    pub fn vmxon() {

    }
    
    pub fn vmclear() {

    }
    
    pub fn vmread() {

    }
    
    pub fn vmwrite() {

    }
        
    pub fn int1() {

    }
    
    pub fn salc() {

    }
    
    pub fn ffreep() {

    }
    
    // AMD SVM
    pub fn vmrun() {

    }
    
    pub fn vmmcall() {

    }
    
    pub fn vmload() {

    }
    
    pub fn vmsave() {

    }
    
    pub fn stgi() {

    }
    
    pub fn clgi() {

    }
    
    pub fn skinit() {

    }
    
    pub fn invlpga() {

    }
    
    pub fn rdtscp() {

    }
        
    // Intel VMX
    pub fn invept() {

    }
    
    pub fn invvpid() {

    }
        
    // Intel Westmere
    pub fn pclmulqdq() {

    }
    
    pub fn aesimc() {

    }
    
    pub fn aesenc() {

    }
    
    pub fn aesenclast() {

    }
    
    pub fn aesdec() {

    }
    
    pub fn aesdeclast() {

    }
    
    pub fn aeskeygenassist() {

    }
        
    // Intel Atom
    pub fn movbe() {

    }
        
    // Intel Sandy Bridge
    pub fn xgetbv() {

    }
    
    pub fn xsetbv() {

    }
    
    pub fn xsave32() {

    }
    
    pub fn xrstor32() {

    }
    
    pub fn xsaveopt32() {

    }
    
    // AVX
    pub fn vmovss() {

    }
    
    pub fn vmovsd() {

    }
    
    pub fn vmovups() {

    }
    
    pub fn vmovupd() {

    }
    
    pub fn vmovlps() {

    }
    
    pub fn vmovsldup() {

    }
    
    pub fn vmovlpd() {

    }
    
    pub fn vmovddup() {

    }
    
    pub fn vunpcklps() {

    }
    
    pub fn vunpcklpd() {

    }
    
    pub fn vunpckhps() {

    }
    
    pub fn vunpckhpd() {

    }
    
    pub fn vmovhps() {

    }
    
    pub fn vmovshdup() {

    }
    
    pub fn vmovhpd() {

    }
    
    pub fn vmovaps() {

    }
    
    pub fn vmovapd() {

    }
    
    pub fn vcvtsi2ss() {

    }
    
    pub fn vcvtsi2sd() {

    }
    
    pub fn vmovntps() {

    }
    
    pub fn vmovntpd() {

    }
    
    pub fn vcvttss2si() {

    }
    
    pub fn vcvttsd2si() {

    }
    
    pub fn vcvtss2si() {

    }
    
    pub fn vcvtsd2si() {

    }
    
    pub fn vucomiss() {

    }
    
    pub fn vucomisd() {

    }
    
    pub fn vcomiss() {

    }
    
    pub fn vcomisd() {

    }
    
    pub fn vmovmskps() {

    }
    
    pub fn vmovmskpd() {

    }
    
    pub fn vsqrtps() {

    }
    
    pub fn vsqrtss() {

    }
    
    pub fn vsqrtpd() {

    }
    
    pub fn vsqrtsd() {

    }
    
    pub fn vrsqrtps() {

    }
    
    pub fn vrsqrtss() {

    }
    
    pub fn vrcpps() {

    }
    
    pub fn vrcpss() {

    }
    
    pub fn vandps() {

    }
    
    pub fn vandpd() {

    }
    
    pub fn vandnps() {

    }
    
    pub fn vandnpd() {

    }
    
    pub fn vorps() {

    }
    
    pub fn vorpd() {

    }
    
    pub fn vxorps() {

    }
    
    pub fn vxorpd() {

    }
    
    pub fn vaddps() {

    }
    
    pub fn vaddss() {

    }
    
    pub fn vaddpd() {

    }
    
    pub fn vaddsd() {

    }
    
    pub fn vmulps() {

    }
    
    pub fn vmulss() {

    }
    
    pub fn vmulpd() {

    }
    
    pub fn vmulsd() {

    }
    
    pub fn vcvtps2pd() {

    }
    
    pub fn vcvtss2sd() {

    }
    
    pub fn vcvtpd2ps() {

    }
    
    pub fn vcvtsd2ss() {

    }
    
    pub fn vcvtdq2ps() {

    }
    
    pub fn vcvttps2dq() {

    }
    
    pub fn vcvtps2dq() {

    }
    
    pub fn vsubps() {

    }
    
    pub fn vsubss() {

    }
    
    pub fn vsubpd() {

    }
    
    pub fn vsubsd() {

    }
    
    pub fn vminps() {

    }
    
    pub fn vminss() {

    }
    
    pub fn vminpd() {

    }
    
    pub fn vminsd() {

    }
    
    pub fn vdivps() {

    }
    
    pub fn vdivss() {

    }
    
    pub fn vdivpd() {

    }
    
    pub fn vdivsd() {

    }
    
    pub fn vmaxps() {

    }
    
    pub fn vmaxss() {

    }
    
    pub fn vmaxpd() {

    }
    
    pub fn vmaxsd() {

    }
    
    pub fn vpunpcklbw() {

    }
    
    pub fn vpunpcklwd() {

    }
    
    pub fn vpunpckldq() {

    }
    
    pub fn vpacksswb() {

    }
    
    pub fn vpcmpgtb() {

    }
    
    pub fn vpcmpgtw() {

    }
    
    pub fn vpcmpgtd() {

    }
    
    pub fn vpackuswb() {

    }
    
    pub fn vpunpckhbw() {

    }
    
    pub fn vpunpckhwd() {

    }
    
    pub fn vpunpckhdq() {

    }
    
    pub fn vpackssdw() {

    }
    
    pub fn vpunpcklqdq() {

    }
    
    pub fn vpunpckhqdq() {

    }
    
    pub fn vmovd() {

    }
    
    pub fn vpshufhw() {

    }

    pub fn vpshufd() {

    }
    
    pub fn vpshuflw() {

    }
    
    pub fn vpcmpeqb() {

    }
    
    pub fn vpcmpeqw() {

    }
    
    pub fn vpcmpeqd() {

    }
    
    pub fn vmovq() {

    }
    
    pub fn vcmpps() {

    }
    
    pub fn vcmpss() {

    }
    
    pub fn vcmppd() {

    }
    
    pub fn vcmpsd() {

    }
    
    pub fn vpinsrw() {

    }
    
    pub fn vpextrw() {

    }
    
    pub fn vshufps() {

    }
    
    pub fn vshufpd() {

    }
    
    pub fn vpsrlw() {

    }
    
    pub fn vpsrld() {

    }
    
    pub fn vpsrlq() {

    }
    
    pub fn vpaddq() {

    }
    
    pub fn vpmullw() {

    }
    
    pub fn vpmovmskb() {

    }
    
    pub fn vpsubusb() {

    }
    
    pub fn vpsubusw() {

    }
    
    pub fn vpminub() {

    }
    
    pub fn vpand() {

    }
    
    pub fn vpaddusb() {

    }
    
    pub fn vpaddusw() {

    }
    
    pub fn vpmaxub() {

    }
    
    pub fn vpandn() {

    }
    
    pub fn vpavgb() {

    }
    
    pub fn vpsraw() {

    }
    
    pub fn vpsrad() {

    }
    
    pub fn vpavgw() {

    }
    
    pub fn vpmulhuw() {

    }
    
    pub fn vpmulhw() {

    }
    
    pub fn vcvtdq2pd() {

    }
    
    pub fn vcvttpd2dq() {

    }
    
    pub fn vcvtpd2dq() {

    }
    
    pub fn vmovntdq() {

    }
    
    pub fn vpsubsb() {

    }
    
    pub fn vpsubsw() {

    }
    
    pub fn vpminsw() {

    }
    
    pub fn vpor() {

    }
    
    pub fn vpaddsb() {

    }
    
    pub fn vpaddsw() {

    }
    
    pub fn vpmaxsw() {

    }
    
    pub fn vpxor() {

    }
    
    pub fn vpsllw() {

    }
    
    pub fn vpslld() {

    }
    
    pub fn vpsllq() {

    }
    
    pub fn vpmuludq() {

    }
    
    pub fn vpmaddwd() {

    }
    
    pub fn vpsadbw() {

    }
    
    pub fn vmaskmovdqu() {

    }
    
    pub fn vpsubb() {

    }
    
    pub fn vpsubw() {

    }
    
    pub fn vpsubd() {

    }
    
    pub fn vpsubq() {

    }
    
    pub fn vpaddb() {

    }
    
    pub fn vpaddw() {

    }
    
    pub fn vpaddd() {

    }
    
    pub fn vpsrldq() {

    }
    
    pub fn vpslldq() {

    }
    
    pub fn vmovdqu() {

    }
    
    pub fn vmovdqa() {

    }
    
    pub fn vhaddpd() {

    }
    
    pub fn vhaddps() {

    }
    
    pub fn vhsubpd() {

    }
    
    pub fn vhsubps() {

    }
    
    pub fn vaddsubpd() {

    }
    
    pub fn vaddsubps() {

    }
    
    pub fn vlddqu() {

    }
    
    pub fn vpshufb() {

    }
    
    pub fn vphaddw() {

    }
    
    pub fn vphaddd() {

    }
    
    pub fn vphaddsw() {

    }
    
    pub fn vpmaddubsw() {

    }
    
    pub fn vphsubw() {

    }
    
    pub fn vphsubd() {

    }
    
    pub fn vphsubsw() {

    }
    
    pub fn vpsignb() {

    }
    
    pub fn vpsignw() {

    }
    
    pub fn vpsignd() {

    }
    
    pub fn vpmulhrsw() {

    }
    
    pub fn vpabsb() {

    }
    
    pub fn vpabsw() {

    }
    
    pub fn vpabsd() {

    }
    
    pub fn vpalignr() {

    }
    
    pub fn vpblendvb() {

    }
    
    pub fn vblendvps() {

    }
    
    pub fn vblendvpd() {

    }
    
    pub fn vptest() {

    }
    
    pub fn vpmovsxbw() {

    }
    
    pub fn vpmovsxbd() {

    }
    
    pub fn vpmovsxbq() {

    }
    
    pub fn vpmovsxwd() {

    }
    
    pub fn vpmovsxwq() {

    }
    
    pub fn vpmovsxdq() {

    }
    
    pub fn vpmuldq() {

    }
    
    pub fn vpcmpeqq() {

    }
    
    pub fn vmovntdqa() {

    }
    
    pub fn vpackusdw() {

    }
    
    pub fn vpmovzxbw() {

    }
    
    pub fn vpmovzxbd() {

    }
    
    pub fn vpmovzxbq() {

    }
    
    pub fn vpmovzxwd() {

    }
    
    pub fn vpmovzxwq() {

    }
    
    pub fn vpmovzxdq() {

    }
    
    pub fn vpcmpgtq() {

    }
    
    pub fn vpminsb() {

    }
    
    pub fn vpminsd() {

    }
    
    pub fn vpminuw() {

    }
    
    pub fn vpminud() {

    }
    
    pub fn vpmaxsb() {

    }
    
    pub fn vpmaxsd() {

    }
    
    pub fn vpmaxuw() {

    }
    
    pub fn vpmaxud() {

    }
    
    pub fn vpmulld() {

    }
    
    pub fn vphminposuw() {

    }
    
    pub fn vaesimc() {

    }
    
    pub fn vaesenc() {

    }
    
    pub fn vaesenclast() {

    }
    
    pub fn vaesdec() {

    }
    
    pub fn vaesdeclast() {

    }
    
    pub fn vpextrb() {

    }
    
    pub fn vpextrd() {

    }
    
    pub fn vextractps() {

    }
    
    pub fn vroundps() {

    }
    
    pub fn vroundpd() {

    }
    
    pub fn vroundss() {

    }
    
    pub fn vroundsd() {

    }
    
    pub fn vblendps() {

    }
    
    pub fn vblendpd() {

    }
    
    pub fn vpblendw() {

    }
    
    pub fn vpinsrb() {

    }
    
    pub fn vinsertps() {

    }
    
    pub fn vpinsrd() {

    }
    
    pub fn vdpps() {

    }
    
    pub fn vdppd() {

    }
    
    pub fn vmpsadbw() {

    }
    
    pub fn vpcmpestrm() {

    }
    
    pub fn vpcmpestri() {

    }
    
    pub fn vpcmpistrm() {

    }
    
    pub fn vpcmpistri() {

    }
    
    pub fn vpclmulqdq() {

    }
    
    pub fn vaeskeygenassist() {

    }
    
    pub fn vtestps() {

    }
    
    pub fn vtestpd() {

    }
    
    pub fn vzeroupper() {

    }
    
    pub fn vzeroall() {

    }
    
    pub fn vldmxcsr() {

    }
    
    pub fn vstmxcsr() {

    }
    
    pub fn vbroadcastss() {

    }
    
    pub fn vbroadcastsd() {

    }
    
    pub fn vbroadcastf128() {

    }
    
    pub fn vmaskmovps() {

    }
    
    pub fn vmaskmovpd() {

    }
    
    pub fn vpermilps() {

    }

    pub fn vpermilpd() {

    }
    
    pub fn vperm2f128() {

    }
    
    pub fn vinsertf128() {

    }
    
    pub fn vextractf128() {

    }

    // under F16C cpuid flag
    pub fn vcvtph2ps() {

    }
    
    pub fn vcvtps2ph() {

    }

    pub fn vfmadd132ps() {

    }
    
    pub fn vfmadd132pd() {

    }
    
    pub fn vfmadd213ps() {

    }
    
    pub fn vfmadd213pd() {

    }
    
    pub fn vfmadd231ps() {

    }
    
    pub fn vfmadd231pd() {

    }
    
    pub fn vfmadd132ss() {

    }
    
    pub fn vfmadd132sd() {

    }
    
    pub fn vfmadd213ss() {

    }
    
    pub fn vfmadd213sd() {

    }

    pub fn vfmadd231ss() {

    }
    
    pub fn vfmadd231sd() {

    }
    
    pub fn vfmaddsub132ps() {

    }
    
    pub fn vfmaddsub132pd() {

    }

    pub fn vfmaddsub213ps() {

    }
    
    pub fn vfmaddsub213pd() {

    }
    
    pub fn vfmaddsub231ps() {

    }
    
    pub fn vfmaddsub231pd() {

    }
    
    pub fn vfmsubadd132ps() {

    }
    
    pub fn vfmsubadd132pd() {

    }
    
    pub fn vfmsubadd213ps() {

    }
    
    pub fn vfmsubadd213pd() {

    }
    
    pub fn vfmsubadd231ps() {

    }
    
    pub fn vfmsubadd231pd() {

    }
    
    pub fn vfmsub132ps() {

    }
    
    pub fn vfmsub132pd() {

    }
    
    pub fn vfmsub213ps() {

    }
    
    pub fn vfmsub213pd() {

    }
    
    pub fn vfmsub231ps() {

    }
    
    pub fn vfmsub231pd() {

    }
    
    pub fn vfmsub132ss() {

    }
    
    pub fn vfmsub132sd() {

    }
    
    pub fn vfmsub213ss() {

    }
    
    pub fn vfmsub213sd() {

    }
    
    pub fn vfmsub231ss() {

    }
    
    pub fn vfmsub231sd() {

    }
    
    pub fn vfnmadd132ps() {

    }
    
    pub fn vfnmadd132pd() {

    }
    
    pub fn vfnmadd213ps() {

    }
    
    pub fn vfnmadd213pd() {

    }
    
    pub fn vfnmadd231ps() {

    }
    
    pub fn vfnmadd231pd() {

    }
    
    pub fn vfnmadd132ss() {

    }
    
    pub fn vfnmadd132sd() {

    }
    
    pub fn vfnmadd213ss() {

    }
    
    pub fn vfnmadd213sd() {

    }
    
    pub fn vfnmadd231ss() {

    }
    
    pub fn vfnmadd231sd() {

    }
    
    pub fn vfnmsub132ps() {

    }
    
    pub fn vfnmsub132pd() {

    }
    
    pub fn vfnmsub213ps() {

    }
    
    pub fn vfnmsub213pd() {

    }
    
    pub fn vfnmsub231ps() {

    }
    
    pub fn vfnmsub231pd() {

    }
    
    pub fn vfnmsub132ss() {

    }
    
    pub fn vfnmsub132sd() {

    }
    
    pub fn vfnmsub213ss() {

    }
    
    pub fn vfnmsub213sd() {

    }
    
    pub fn vfnmsub231ss() {

    }
    
    pub fn vfnmsub231sd() {

    }

    pub fn movq2dq() {

    }
    
    pub fn movdq2q() {

    }

    pub fn fxsave64() {

    }
    
    pub fn fxrstor64() {

    }
    
    pub fn xsave64() {

    }
    
    pub fn xrstor64() {

    }
    
    pub fn xsaveopt64() {

    }
    
    //Intel Ivy Bridge: RDRAND/FSGSBASE cpuid flags */
    pub fn rdrand() {

    }
    
    pub fn rdfsbase() {

    }
    
    pub fn rdgsbase() {

    }
    
    pub fn wrfsbase() {

    }
    
    pub fn wrgsbase() {

    }
    
    pub fn rdseed() {

    }
    
    // AMD FMA4
    pub fn vfmaddsubps() {

    }
    
    pub fn vfmaddsubpd() {

    }
    
    pub fn vfmsubaddps() {

    }
    
    pub fn vfmsubaddpd() {

    }
    
    pub fn vfmaddps() {

    }
    
    pub fn vfmaddpd() {

    }
    
    pub fn vfmaddss() {

    }
    
    pub fn vfmaddsd() {

    }
    
    pub fn vfmsubps() {

    }
    
    pub fn vfmsubpd() {

    }
    
    pub fn vfmsubss() {

    }
    
    pub fn vfmsubsd() {

    }
    
    pub fn vfnmaddps() {

    }
    
    pub fn vfnmaddpd() {

    }
    
    pub fn vfnmaddss() {

    }
    
    pub fn vfnmaddsd() {

    }
    
    pub fn vfnmsubps() {

    }
    
    pub fn vfnmsubpd() {

    }
    
    pub fn vfnmsubss() {

    }
    
    pub fn vfnmsubsd() {

    }

    // AMD XOP
    pub fn vfrczps() {

    }
    
    pub fn vfrczpd() {

    }

    pub fn vfrczss() {

    }
    
    pub fn vfrczsd() {

    }
    
    pub fn vpcmov() {

    }
    
    pub fn vpcomb() {

    }
    
    pub fn vpcomw() {

    }
    
    pub fn vpcomd() {

    }
    
    pub fn vpcomq() {

    }
    
    pub fn vpcomub() {

    }
    
    pub fn vpcomuw() {

    }
    
    pub fn vpcomud() {

    }
    
    pub fn vpcomuq() {

    }
    
    pub fn vpermil2pd() {

    }
    
    pub fn vpermil2ps() {

    }
    
    pub fn vphaddbw() {

    }
    
    pub fn vphaddbd() {

    }
    
    pub fn vphaddbq() {

    }
    
    pub fn vphaddwd() {

    }
    
    pub fn vphaddwq() {

    }
    
    pub fn vphadddq() {

    }
    
    pub fn vphaddubw() {

    }
    
    pub fn vphaddubd() {

    }
    
    pub fn vphaddubq() {

    }
    
    pub fn vphadduwd() {

    }
    
    pub fn vphadduwq() {

    }
    
    pub fn vphaddudq() {

    }
    
    pub fn vphsubbw() {

    }
    
    pub fn vphsubwd() {

    }
    
    pub fn vphsubdq() {

    }
    
    pub fn vpmacssww() {

    }
    
    pub fn vpmacsswd() {

    }
    
    pub fn vpmacssdql() {

    }
    
    pub fn vpmacssdd() {

    }
    
    pub fn vpmacssdqh() {

    }
    
    pub fn vpmacsww() {

    }

    pub fn vpmacswd() {

    }
    
    pub fn vpmacsdql() {

    }
    
    pub fn vpmacsdd() {

    }
    
    pub fn vpmacsdqh() {

    }
    
    pub fn vpmadcsswd() {

    }
    
    pub fn vpmadcswd() {

    }
    
    pub fn vpperm() {

    }
    
    pub fn vprotb() {

    }
    
    pub fn vprotw() {

    }
    
    pub fn vprotd() {

    }
    
    pub fn vprotq() {

    }
    
    pub fn vpshlb() {

    }
    
    pub fn vpshlw() {

    }
    
    pub fn vpshld() {

    }
    
    pub fn vpshlq() {

    }
    
    pub fn vpshab() {

    }
    
    pub fn vpshaw() {

    }
    
    pub fn vpshad() {

    }
    
    pub fn vpshaq() {

    }

    // AMD TBM
    pub fn bextr() {

    }
    
    pub fn blcfill() {

    }
    
    pub fn blci() {

    }
    
    pub fn blcic() {

    }
    
    pub fn blcmsk() {

    }
    
    pub fn blcs() {

    }
    
    pub fn blsfill() {

    }
    
    pub fn blsic() {

    }
    
    pub fn t1mskc() {

    }
    
    pub fn tzmsk() {

    }

    // AMD LWP
    pub fn llwpcb() {

    }
    
    pub fn slwpcb() {

    }
    
    pub fn lwpins() {

    }
    
    pub fn lwpval() {

    }
    
    // Intel BMI1
    pub fn andn() {

    }
    
    pub fn blsr() {

    }
    
    pub fn blsmsk() {

    }
    
    pub fn blsi() {

    }
    
    pub fn tzcnt() {

    }

    // Intel BMI2
    pub fn bzhi() {

    }
    
    pub fn pext() {

    }
    
    pub fn pdep() {

    }
    
    pub fn sarx() {

    }
    
    pub fn shlx() {

    }
    
    pub fn shrx() {

    }
    
    pub fn rorx() {

    }
    
    pub fn mulx() {

    }

    // Intel Safer Mode Extensions
    pub fn getsec() {

    }

    // Misc Intel additions
    pub fn vmfunc() {

    }
    
    pub fn invpcid() {

    }

    // Intel TSX
    pub fn xabort() {

    }
    
    pub fn xbegin() {

    }
    
    pub fn xend() {

    }
    
    pub fn xtest() {

    }

    // AVX2
    pub fn vpgatherdd() {

    }
    
    pub fn vpgatherdq() {

    }
    
    pub fn vpgatherqd() {

    }
    
    pub fn vpgatherqq() {

    }
    
    pub fn vgatherdps() {

    }
    
    pub fn vgatherdpd() {

    }
    
    pub fn vgatherqps() {

    }
    
    pub fn vgatherqpd() {

    }
    
    pub fn vbroadcasti128() {

    }
    
    pub fn vinserti128() {

    }
    
    pub fn vextracti128() {

    }
    
    pub fn vpmaskmovd() {

    }
    
    pub fn vpmaskmovq() {

    }
    
    pub fn vperm2i128() {

    }
    
    pub fn vpermd() {

    }
    
    pub fn vpermps() {

    }
    
    pub fn vpermq() {

    }
    
    pub fn vpermpd() {

    }
    
    pub fn vpblendd() {

    }
    
    pub fn vpsllvd() {

    }
    
    pub fn vpsllvq() {

    }
    
    pub fn vpsravd() {

    }
    
    pub fn vpsrlvd() {

    }
    
    pub fn vpsrlvq() {

    }
    
    pub fn vpbroadcastb() {

    }
    
    pub fn vpbroadcastw() {

    }
    
    pub fn vpbroadcastd() {

    }
    
    pub fn vpbroadcastq() {

    }

    // Intel Skylake
    pub fn xsavec32() {

    }
    
    pub fn xsavec64() {

    }

    // Intel ADX
    pub fn adox() {

    }
    
    pub fn adcx() {

    }

    // Intel AVX-512 VEX
    pub fn kmovw() {

    }
    
    pub fn kmovb() {

    }
    
    pub fn kmovq() {

    }
    
    pub fn kmovd() {

    }
    
    pub fn kandw() {

    }
    
    pub fn kandb() {

    }
    
    pub fn kandq() {

    }
    
    pub fn kandd() {

    }
    
    pub fn kandnw() {

    }
    
    pub fn kandnb() {

    }
    
    pub fn kandnq() {

    }
    
    pub fn kandnd() {

    }
    
    pub fn kunpckbw() {

    }
    
    pub fn kunpckwd() {

    }
    
    pub fn kunpckdq() {

    }
    
    pub fn knotw() {

    }
    
    pub fn knotb() {

    }
    
    pub fn knotq() {

    }
    
    pub fn knotd() {

    }
    
    pub fn korw() {

    }
    
    pub fn korb() {

    }
    
    pub fn korq() {

    }
    
    pub fn kord() {

    }
    
    pub fn kxnorw() {

    }
    
    pub fn kxnorb() {

    }
    
    pub fn kxnorq() {

    }
    
    pub fn kxnord() {

    }
    
    pub fn kxorw() {

    }
    
    pub fn kxorb() {

    }
    
    pub fn kxorq() {

    }
    
    pub fn kxord() {

    }
    
    pub fn kaddw() {

    }
    
    pub fn kaddb() {

    }
    
    pub fn kaddq() {

    }
    
    pub fn kaddd() {

    }
    
    pub fn kortestw() {

    }
    
    pub fn kortestb() {

    }
    
    pub fn kortestq() {

    }
    
    pub fn kortestd() {

    }
    
    pub fn kshiftlw() {

    }
    
    pub fn kshiftlb() {

    }
    
    pub fn kshiftlq() {

    }

    pub fn kshiftld() {

    }
    
    pub fn kshiftrw() {

    }
    
    pub fn kshiftrb() {

    }
    
    pub fn kshiftrq() {

    }
    
    pub fn kshiftrd() {

    }
    
    pub fn ktestw() {

    }
    
    pub fn ktestb() {

    }
    
    pub fn ktestq() {

    }
    
    pub fn ktestd() {

    }
    
    // Intel AVX-512 EVEX
    pub fn valignd() {

    }
    
    pub fn valignq() {

    }
    
    pub fn vblendmpd() {

    }
    
    pub fn vblendmps() {

    }
    
    pub fn vbroadcastf32x2() {

    }
    
    pub fn vbroadcastf32x4() {

    }
    
    pub fn vbroadcastf32x8() {

    }
    
    pub fn vbroadcastf64x2() {

    }
    
    pub fn vbroadcastf64x4() {

    }
    
    pub fn vbroadcasti32x2() {

    }
    
    pub fn vbroadcasti32x4() {

    }
    
    pub fn vbroadcasti32x8() {

    }
    
    pub fn vbroadcasti64x2() {

    }
    
    pub fn vbroadcasti64x4() {

    }
    
    pub fn vcompresspd() {

    }
    
    pub fn vcompressps() {

    }
    
    pub fn vcvtpd2qq() {

    }
    
    pub fn vcvtpd2udq() {

    }
    
    pub fn vcvtpd2uqq() {

    }
    
    pub fn vcvtps2qq() {

    }
    
    pub fn vcvtps2udq() {

    }
    
    pub fn vcvtps2uqq() {

    }
    
    pub fn vcvtqq2pd() {

    }
    
    pub fn vcvtqq2ps() {

    }
    
    pub fn vcvtsd2usi() {

    }
    
    pub fn vcvtss2usi() {

    }
    
    pub fn vcvttpd2qq() {

    }
    
    pub fn vcvttpd2udq() {

    }
    
    pub fn vcvttpd2uqq() {

    }
    
    pub fn vcvttps2qq() {

    }
    
    pub fn vcvttps2udq() {

    }
    
    pub fn vcvttps2uqq() {

    }
    
    pub fn vcvttsd2usi() {

    }
    
    pub fn vcvttss2usi() {

    }
    
    pub fn vcvtudq2pd() {

    }
    
    pub fn vcvtudq2ps() {

    }
    
    pub fn vcvtuqq2pd() {

    }
    
    pub fn vcvtuqq2ps() {

    }
    
    pub fn vcvtusi2sd() {

    }
    
    pub fn vcvtusi2ss() {

    }
    
    pub fn vdbpsadbw() {

    }
    
    pub fn vexp2pd() {

    }
    
    pub fn vexp2ps() {

    }
    
    pub fn vexpandpd() {

    }
    
    pub fn vexpandps() {

    }
    
    pub fn vextractf32x4() {

    }
    
    pub fn vextractf32x8() {

    }
    
    pub fn vextractf64x2() {

    }
    
    pub fn vextractf64x4() {

    }
    
    pub fn vextracti32x4() {

    }
    
    pub fn vextracti32x8() {

    }
    
    pub fn vextracti64x2() {

    }
    
    pub fn vextracti64x4() {

    }
    
    pub fn vfixupimmpd() {

    }
    
    pub fn vfixupimmps() {

    }
    
    pub fn vfixupimmsd() {

    }
    
    pub fn vfixupimmss() {

    }
    
    pub fn vfpclasspd() {

    }
    
    pub fn vfpclassps() {

    }
    
    pub fn vfpclasssd() {

    }
    
    pub fn vfpclassss() {

    }
    
    pub fn vgatherpf0dpd() {

    }
    
    pub fn vgatherpf0dps() {

    }
    
    pub fn vgatherpf0qpd() {

    }
    
    pub fn vgatherpf0qps() {

    }
    
    pub fn vgatherpf1dpd() {

    }
    
    pub fn vgatherpf1dps() {

    }
    
    pub fn vgatherpf1qpd() {

    }
    
    pub fn vgatherpf1qps() {

    }
    
    pub fn vgetexppd() {

    }
    
    pub fn vgetexpps() {

    }
    
    pub fn vgetexpsd() {

    }
    
    pub fn vgetexpss() {

    }
    
    pub fn vgetmantpd() {

    }
    
    pub fn vgetmantps() {

    }
    
    pub fn vgetmantsd() {

    }
    
    pub fn vgetmantss() {

    }
    
    pub fn vinsertf32x4() {

    }
    
    pub fn vinsertf32x8() {

    }
    
    pub fn vinsertf64x2() {

    }
    
    pub fn vinsertf64x4() {

    }
    
    pub fn vinserti32x4() {

    }
    
    pub fn vinserti32x8() {

    }
    
    pub fn vinserti64x2() {

    }
    
    pub fn vinserti64x4() {

    }
    
    pub fn vmovdqa32() {

    }
    
    pub fn vmovdqa64() {

    }
    
    pub fn vmovdqu16() {

    }
    
    pub fn vmovdqu32() {

    }
    
    pub fn vmovdqu64() {

    }
    
    pub fn vmovdqu8() {

    }
    
    pub fn vpabsq() {

    }
    
    pub fn vpandd() {

    }
    
    pub fn vpandnd() {

    }
    
    pub fn vpandnq() {

    }
    
    pub fn vpandq() {

    }
    
    pub fn vpblendmb() {

    }
    
    pub fn vpblendmd() {

    }
    
    pub fn vpblendmq() {

    }
    
    pub fn vpblendmw() {

    }
    
    pub fn vpbroadcastmb2q() {

    }
    
    pub fn vpbroadcastmw2d() {

    }
    
    pub fn vpcmpb() {

    }
    
    pub fn vpcmpd() {

    }
    
    pub fn vpcmpq() {

    }
    
    pub fn vpcmpub() {

    }
    
    pub fn vpcmpud() {

    }
    
    pub fn vpcmpuq() {

    }
    
    pub fn vpcmpuw() {

    }
    
    pub fn vpcmpw() {

    }
    
    pub fn vpcompressd() {

    }
    
    pub fn vpcompressq() {

    }
    
    pub fn vpconflictd() {

    }
    
    pub fn vpconflictq() {

    }
    
    pub fn vpermb() {

    }
    
    pub fn vpermi2b() {

    }
    
    pub fn vpermi2d() {

    }
    
    pub fn vpermi2pd() {

    }
    
    pub fn vpermi2ps() {

    }
    
    pub fn vpermi2q() {

    }
    
    pub fn vpermi2w() {

    }
    
    pub fn vpermt2b() {

    }
    
    pub fn vpermt2d() {

    }
    
    pub fn vpermt2pd() {

    }
    
    pub fn vpermt2ps() {

    }
    
    pub fn vpermt2q() {

    }
    
    pub fn vpermt2w() {

    }
    
    pub fn vpermw() {

    }
    
    pub fn vpexpandd() {

    }
    
    pub fn vpexpandq() {

    }
    
    pub fn vpextrq() {

    }
    
    pub fn vpinsrq() {

    }
    
    pub fn vplzcntd() {

    }
    
    pub fn vplzcntq() {

    }
    
    pub fn vpmadd52huq() {

    }
    
    pub fn vpmadd52luq() {

    }
    
    pub fn vpmaxsq() {

    }
    
    pub fn vpmaxuq() {

    }
    
    pub fn vpminsq() {

    }
    
    pub fn vpminuq() {

    }
    
    pub fn vpmovb2m() {

    }
    
    pub fn vpmovd2m() {

    }
    
    pub fn vpmovdb() {

    }
    
    pub fn vpmovdw() {

    }
    
    pub fn vpmovm2b() {

    }
    
    pub fn vpmovm2d() {

    }
    
    pub fn vpmovm2q() {

    }
    
    pub fn vpmovm2w() {

    }
    
    pub fn vpmovq2m() {

    }
    
    pub fn vpmovqb() {

    }
    
    pub fn vpmovqd() {

    }
    
    pub fn vpmovqw() {

    }
    
    pub fn vpmovsdb() {

    }
    
    pub fn vpmovsdw() {

    }
    
    pub fn vpmovsqb() {

    }
    
    pub fn vpmovsqd() {

    }
    
    pub fn vpmovsqw() {

    }
    
    pub fn vpmovswb() {

    }
    
    pub fn vpmovusdb() {

    }
    
    pub fn vpmovusdw() {

    }
    
    pub fn vpmovusqb() {

    }
    
    pub fn vpmovusqd() {

    }
    
    pub fn vpmovusqw() {

    }
    
    pub fn vpmovuswb() {

    }
    
    pub fn vpmovw2m() {

    }
    
    pub fn vpmovwb() {

    }
    
    pub fn vpmullq() {

    }
    
    pub fn vpord() {

    }
    
    pub fn vporq() {

    }
    
    pub fn vprold() {

    }
    
    pub fn vprolq() {

    }
    
    pub fn vprolvd() {

    }
    
    pub fn vprolvq() {

    }
    
    pub fn vprord() {

    }
    
    pub fn vprorq() {

    }
    
    pub fn vprorvd() {

    }
    
    pub fn vprorvq() {

    }
    
    pub fn vpscatterdd() {

    }
    
    pub fn vpscatterdq() {

    }
    
    pub fn vpscatterqd() {

    }
    
    pub fn vpscatterqq() {

    }
    
    pub fn vpsllvw() {

    }
    
    pub fn vpsraq() {

    }
    
    pub fn vpsravq() {

    }
    
    pub fn vpsravw() {

    }
    
    pub fn vpsrlvw() {

    }
    
    pub fn vpternlogd() {

    }
    
    pub fn vpternlogq() {

    }
    
    pub fn vptestmb() {

    }
    
    pub fn vptestmd() {

    }
    
    pub fn vptestmq() {

    }
    
    pub fn vptestmw() {

    }
    
    pub fn vptestnmb() {

    }
    
    pub fn vptestnmd() {

    }
    
    pub fn vptestnmq() {

    }
    
    pub fn vptestnmw() {

    }
    
    pub fn vpxord() {

    }
    
    pub fn vpxorq() {

    }
    
    pub fn vrangepd() {

    }
    
    pub fn vrangeps() {

    }
    
    pub fn vrangesd() {

    }
    
    pub fn vrangess() {

    }
    
    pub fn vrcp14pd() {

    }
    
    pub fn vrcp14ps() {

    }
    
    pub fn vrcp14sd() {

    }
    
    pub fn vrcp14ss() {

    }
    
    pub fn vrcp28pd() {

    }
    
    pub fn vrcp28ps() {

    }
    
    pub fn vrcp28sd() {

    }
    
    pub fn vrcp28ss() {

    }
    
    pub fn vreducepd() {

    }
    
    pub fn vreduceps() {

    }
    
    pub fn vreducesd() {

    }
    
    pub fn vreducess() {

    }
    
    pub fn vrndscalepd() {

    }
    
    pub fn vrndscaleps() {

    }
    
    pub fn vrndscalesd() {

    }
    
    pub fn vrndscaless() {

    }
    
    pub fn vrsqrt14pd() {

    }
    
    pub fn vrsqrt14ps() {

    }
    
    pub fn vrsqrt14sd() {

    }
    
    pub fn vrsqrt14ss() {

    }
    
    pub fn vrsqrt28pd() {

    }
    
    pub fn vrsqrt28ps() {

    }
    
    pub fn vrsqrt28sd() {

    }
    
    pub fn vrsqrt28ss() {

    }
    
    pub fn vscalefpd() {

    }
    
    pub fn vscalefps() {

    }
    
    pub fn vscalefsd() {

    }
    
    pub fn vscalefss() {

    }
    
    pub fn vscatterdpd() {

    }
    
    pub fn vscatterdps() {

    }
    
    pub fn vscatterqpd() {

    }
    
    pub fn vscatterqps() {

    }
    
    pub fn vscatterpf0dpd() {

    }
    
    pub fn vscatterpf0dps() {

    }
    
    pub fn vscatterpf0qpd() {

    }
    
    pub fn vscatterpf0qps() {

    }
    
    pub fn vscatterpf1dpd() {

    }
    
    pub fn vscatterpf1dps() {

    }
    
    pub fn vscatterpf1qpd() {

    }
    
    pub fn vscatterpf1qps() {

    }
    
    pub fn vshuff32x4() {

    }
    
    pub fn vshuff64x2() {

    }
    
    pub fn vshufi32x4() {

    }
    
    pub fn vshufi64x2() {

    }
        
    // Intel SHA extensions
    pub fn sha1msg1() {

    }
    
    pub fn sha1msg2() {

    }
    
    pub fn sha1nexte() {

    }
    
    pub fn sha1rnds4() {

    }
    
    pub fn sha256msg1() {

    }
    
    pub fn sha256msg2() {

    }
    
    pub fn sha256rnds2() {

    }
        
    // Intel MPX extensions
    pub fn bndcl() {

    }
    
    pub fn bndcn() {

    }
    
    pub fn bndcu() {

    }
    
    pub fn bndldx() {

    }
    
    pub fn bndmk() {

    }
    
    pub fn bndmov() {

    }
    
    pub fn bndstx() {

    }
        
    // Intel PT extensions
    pub fn ptwrite() {

    }
        
    // AMD monitor extensions
    pub fn monitorx() {

    }

    pub fn mwaitx() {

    }
    
    // Intel MPK extensions
    pub fn rdpkru() {

    }
    
    pub fn wrpkru() {

    }
        
    // Intel Software Guard eXtension
    pub fn encls() {

    }
    
    pub fn enclu() {

    }
    
    pub fn enclv() {

    }
        
    // AVX512 VNNI
    pub fn vpdpbusd() {

    }
    
    pub fn vpdpbusds() {

    }
    
    pub fn vpdpwssd() {

    }
    
    pub fn vpdpwssds() {

    }
        
    // AVX512 BF16
    pub fn vcvtne2ps2bf16() {

    }
    
    pub fn vcvtneps2bf16() {

    }
    
    pub fn vdpbf16ps() {

    }
        
    pub fn vpopcntd() {

    }
    
    pub fn vpopcntq() {

    } 
    
}