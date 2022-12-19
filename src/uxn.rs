
static PAGE_PROGRAM: u32 = 0x0100;
static LIMIT: u32 = 0x40000;


#[derive(Default)]
pub struct Uxn {
    ram: [u8],
    wst: Stack,
    rst: Stack,
    dev: [Device; 16],

}

pub struct Stack {
    ptr: u8,
    dat: [u8; 255],
}

pub struct Device    {
    //uxn: Uxn,
    dat: [u8; 16],
    index: u8,
    u: &mut Uxn,
}


pub fn uxn_eval(uxn: *mut Uxn, pc: u16) {
    let mut a;
	let mut b;
	let mut c;
	let mut j;
	let mut k;
	let mut bs; 
	let mut errcode; 
	let mut instr;
    let limit = LIMIT;
    let mut kptr: u8;
    let sp: *mut u8;
    let dev: *mut Device;

    //TODO: figure out what this bool is supposed to express
    if pc == 0 || uxn.dev[0].dat[0xf] != 0 { return 0 };

    //main loop
    while (uxn.ram[pc] != 0) {
        instr = uxn.ram[pc];

        //if (limit - 1) != 0


        pc += 1;


        match instr {
            /* Stack */
		0x00 => /* LIT */ PEEK(a, pc) PUSH(src, a) pc += 1 + bs,
		case 0x01: /* INC */ POP(a) PUSH(src, a + 1) ,
		case 0x02: /* POP */ POP(a) ,
		case 0x03: /* NIP */ POP(a) POP(b) PUSH(src, a) ,
		case 0x04: /* SWP */ POP(a) POP(b) PUSH(src, a) PUSH(src, b) ,
		case 0x05: /* ROT */ POP(a) POP(b) POP(c) PUSH(src, b) PUSH(src, a) PUSH(src, c) ,
		case 0x06: /* DUP */ POP(a) PUSH(src, a) PUSH(src, a) ,
		case 0x07: /* OVR */ POP(a) POP(b) PUSH(src, b) PUSH(src, a) PUSH(src, b) ,
		/* Logic */
		case 0x08: /* EQU */ POP(a) POP(b) PUSH8(src, b == a) ,
		case 0x09: /* NEQ */ POP(a) POP(b) PUSH8(src, b != a) ,
		case 0x0a: /* GTH */ POP(a) POP(b) PUSH8(src, b > a) ,
		case 0x0b: /* LTH */ POP(a) POP(b) PUSH8(src, b < a) ,
		case 0x0c: /* JMP */ POP(a) WARP(a) ,
		case 0x0d: /* JCN */ POP(a) POP8(b) if(b) WARP(a) ,
		case 0x0e: /* JSR */ POP(a) PUSH16(dst, pc) WARP(a) ,
		case 0x0f: /* STH */ POP(a) PUSH(dst, a) ,
		/* Memory */
		case 0x10: /* LDZ */ POP8(a) PEEK(b, a) PUSH(src, b) ,
		case 0x11: /* STZ */ POP8(a) POP(b) POKE(a, b) ,
		case 0x12: /* LDR */ POP8(a) PEEK(b, pc + (Sint8)a) PUSH(src, b) ,
		case 0x13: /* STR */ POP8(a) POP(b) c = pc + (Sint8)a; POKE(c, b) ,
		case 0x14: /* LDA */ POP16(a) PEEK(b, a) PUSH(src, b) ,
		case 0x15: /* STA */ POP16(a) POP(b) POKE(a, b) ,
		case 0x16: /* DEI */ POP8(a) DEVR(b, &u->dev[a >> 4], a) PUSH(src, b) ,
		case 0x17: /* DEO */ POP8(a) POP(b) DEVW(&u->dev[a >> 4], a, b) ,
		/* Arithmetic */
		case 0x18: /* ADD */ POP(a) POP(b) PUSH(src, b + a) ,
		case 0x19: /* SUB */ POP(a) POP(b) PUSH(src, b - a) ,
		case 0x1a: /* MUL */ POP(a) POP(b) PUSH(src, (Uint32)b * a) ,
		case 0x1b: /* DIV */ POP(a) POP(b) if(a == 0) { errcode = 4; goto err; } PUSH(src, b / a) ,
		case 0x1c: /* AND */ POP(a) POP(b) PUSH(src, b & a) ,
		case 0x1d: /* ORA */ POP(a) POP(b) PUSH(src, b | a) ,
		case 0x1e: /* EOR */ POP(a) POP(b) PUSH(src, b ^ a) ,
		case 0x1f: /* SFT */ POP8(a) POP(b) c = b >> (a & 0x0f) << ((a & 0xf0) >> 4); PUSH(src, c) ,
		}

}

//set the ram
//fields are all zeroed by default
pub fn uxn_boot(Uxn u, u8 ram) {
    u.ram = ram;
}
