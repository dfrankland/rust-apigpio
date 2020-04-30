// DO NOT EDIT - this file is autogenerated
//
// Regenerate by copying new pigpio.h and running `make` here
//
// SPDX-License-Identifier: AGPL-3.0-or-later
// There is NO WARRANTY.
// Generated from:
//   Makefile  converter   Copyright 2020 Ian Jackson
//   pigpio.h              Copyright ...-2020 pigpio contributors
// The "Do not edit" statement above is advice, not a legal restriction!

#![allow(dead_code)]
#![allow(unused_parens)]

//! Constants from C pigpio.h, converted into Rust `const`s.
/* BSC FIFO size */
/* gpio: 0-53 */
pub const PI_MIN_GPIO : u32 = 0;
pub const PI_MAX_GPIO : u32 = 53;
/* user_gpio: 0-31 */
pub const PI_MAX_USER_GPIO : u32 = 31;
/* level: 0-1 */
pub const PI_OFF : u32 = 0;
pub const PI_ON : u32 = 1;
pub const PI_CLEAR : u32 = 0;
pub const PI_SET : u32 = 1;
pub const PI_LOW : u32 = 0;
pub const PI_HIGH : u32 = 1;
/* level: only reported for GPIO time-out, see gpioSetWatchdog */
pub const PI_TIMEOUT : u32 = 2;
/* mode: 0-7 */
pub const PI_INPUT : u32 = 0;
pub const PI_OUTPUT : u32 = 1;
pub const PI_ALT0 : u32 = 4;
pub const PI_ALT1 : u32 = 5;
pub const PI_ALT2 : u32 = 6;
pub const PI_ALT3 : u32 = 7;
pub const PI_ALT4 : u32 = 3;
pub const PI_ALT5 : u32 = 2;
/* pud: 0-2 */
pub const PI_PUD_OFF : u32 = 0;
pub const PI_PUD_DOWN : u32 = 1;
pub const PI_PUD_UP : u32 = 2;
/* dutycycle: 0-range */
pub const PI_DEFAULT_DUTYCYCLE_RANGE : u32 = 255;
/* range: 25-40000 */
pub const PI_MIN_DUTYCYCLE_RANGE : u32 = 25;
pub const PI_MAX_DUTYCYCLE_RANGE : u32 = 40000;
/* pulsewidth: 0, 500-2500 */
pub const PI_SERVO_OFF : u32 = 0;
pub const PI_MIN_SERVO_PULSEWIDTH : u32 = 500;
pub const PI_MAX_SERVO_PULSEWIDTH : u32 = 2500;
/* hardware PWM */
pub const PI_HW_PWM_MIN_FREQ : u32 = 1;
pub const PI_HW_PWM_MAX_FREQ : u32 = 125000000;
pub const PI_HW_PWM_MAX_FREQ_2711 : u32 = 187500000;
pub const PI_HW_PWM_RANGE : u32 = 1000000;
/* hardware clock */
pub const PI_HW_CLK_MIN_FREQ : u32 = 4689;
pub const PI_HW_CLK_MIN_FREQ_2711 : u32 = 13184;
pub const PI_HW_CLK_MAX_FREQ : u32 = 250000000;
pub const PI_HW_CLK_MAX_FREQ_2711 : u32 = 375000000;
pub const PI_NOTIFY_SLOTS : u32 = 32;
pub const PI_NTFY_FLAGS_EVENT : u32 = (1 <<7);
pub const PI_NTFY_FLAGS_ALIVE : u32 = (1 <<6);
pub const PI_NTFY_FLAGS_WDOG : u32 = (1 <<5);
pub const PI_WAVE_BLOCKS : u32 = 4;
pub const PI_WAVE_MAX_PULSES : u32 = (PI_WAVE_BLOCKS * 3000);
pub const PI_WAVE_MAX_CHARS : u32 = (PI_WAVE_BLOCKS *  300);
pub const PI_BB_I2C_MIN_BAUD : u32 = 50;
pub const PI_BB_I2C_MAX_BAUD : u32 = 500000;
pub const PI_BB_SPI_MIN_BAUD : u32 = 50;
pub const PI_BB_SPI_MAX_BAUD : u32 = 250000;
pub const PI_BB_SER_MIN_BAUD : u32 = 50;
pub const PI_BB_SER_MAX_BAUD : u32 = 250000;
pub const PI_BB_SER_NORMAL : u32 = 0;
pub const PI_BB_SER_INVERT : u32 = 1;
pub const PI_WAVE_MIN_BAUD : u32 = 50;
pub const PI_WAVE_MAX_BAUD : u32 = 1000000;
pub const PI_SPI_MIN_BAUD : u32 = 32000;
pub const PI_SPI_MAX_BAUD : u32 = 125000000;
pub const PI_MIN_WAVE_DATABITS : u32 = 1;
pub const PI_MAX_WAVE_DATABITS : u32 = 32;
pub const PI_MIN_WAVE_HALFSTOPBITS : u32 = 2;
pub const PI_MAX_WAVE_HALFSTOPBITS : u32 = 8;
pub const PI_WAVE_MAX_MICROS : u32 = (30 * 60 * 1000000); /* half an hour */
pub const PI_MAX_WAVES : u32 = 250;
pub const PI_MAX_WAVE_CYCLES : u32 = 65535;
pub const PI_MAX_WAVE_DELAY : u32 = 65535;
pub const PI_WAVE_COUNT_PAGES : u32 = 10;
/* wave tx mode */
pub const PI_WAVE_MODE_ONE_SHOT : u32 = 0;
pub const PI_WAVE_MODE_REPEAT : u32 = 1;
pub const PI_WAVE_MODE_ONE_SHOT_SYNC : u32 = 2;
pub const PI_WAVE_MODE_REPEAT_SYNC : u32 = 3;
/* special wave at return values */
pub const PI_WAVE_NOT_FOUND : u32 = 9998; /* Transmitted wave not found. */
pub const PI_NO_TX_WAVE : u32 = 9999; /* No wave being transmitted. */
/* Files, I2C, SPI, SER */
pub const PI_FILE_SLOTS : u32 = 16;
pub const PI_I2C_SLOTS : u32 = 512;
pub const PI_SPI_SLOTS : u32 = 32;
pub const PI_SER_SLOTS : u32 = 16;
pub const PI_MAX_I2C_ADDR : u32 = 0x7F;
pub const PI_NUM_AUX_SPI_CHANNEL : u32 = 3;
pub const PI_NUM_STD_SPI_CHANNEL : u32 = 2;
pub const PI_MAX_I2C_DEVICE_COUNT : u32 = (1<<16);
pub const PI_MAX_SPI_DEVICE_COUNT : u32 = (1<<16);
/* max pi_i2c_msg_t per transaction */
pub const PI_I2C_RDRW_IOCTL_MAX_MSGS : u32 = 42;
/* flags for i2cTransaction, pi_i2c_msg_t */
pub const PI_I2C_M_WR : u32 = 0x0000; /* write data */
pub const PI_I2C_M_RD : u32 = 0x0001; /* read data */
pub const PI_I2C_M_TEN : u32 = 0x0010; /* ten bit chip address */
pub const PI_I2C_M_RECV_LEN : u32 = 0x0400; /* length will be first received byte */
pub const PI_I2C_M_NO_RD_ACK : u32 = 0x0800; /* if I2C_FUNC_PROTOCOL_MANGLING */
pub const PI_I2C_M_IGNORE_NAK : u32 = 0x1000; /* if I2C_FUNC_PROTOCOL_MANGLING */
pub const PI_I2C_M_REV_DIR_ADDR : u32 = 0x2000; /* if I2C_FUNC_PROTOCOL_MANGLING */
pub const PI_I2C_M_NOSTART : u32 = 0x4000; /* if I2C_FUNC_PROTOCOL_MANGLING */
/* bbI2CZip and i2cZip commands */
pub const PI_I2C_END : u32 = 0;
pub const PI_I2C_ESC : u32 = 1;
pub const PI_I2C_START : u32 = 2;
pub const PI_I2C_COMBINED_ON : u32 = 2;
pub const PI_I2C_STOP : u32 = 3;
pub const PI_I2C_COMBINED_OFF : u32 = 3;
pub const PI_I2C_ADDR : u32 = 4;
pub const PI_I2C_FLAGS : u32 = 5;
pub const PI_I2C_READ : u32 = 6;
pub const PI_I2C_WRITE : u32 = 7;
/* SPI */
/* BSC registers */
/* BSC GPIO */
/* Longest busy delay */
pub const PI_MAX_BUSY_DELAY : u32 = 100;
/* timeout: 0-60000 */
pub const PI_MIN_WDOG_TIMEOUT : u32 = 0;
pub const PI_MAX_WDOG_TIMEOUT : u32 = 60000;
/* timer: 0-9 */
pub const PI_MIN_TIMER : u32 = 0;
pub const PI_MAX_TIMER : u32 = 9;
/* millis: 10-60000 */
pub const PI_MIN_MS : u32 = 10;
pub const PI_MAX_MS : u32 = 60000;
pub const PI_MAX_SCRIPTS : u32 = 32;
pub const PI_MAX_SCRIPT_TAGS : u32 = 50;
pub const PI_MAX_SCRIPT_VARS : u32 = 150;
pub const PI_MAX_SCRIPT_PARAMS : u32 = 10;
/* script status */
pub const PI_SCRIPT_INITING : u32 = 0;
pub const PI_SCRIPT_HALTED : u32 = 1;
pub const PI_SCRIPT_RUNNING : u32 = 2;
pub const PI_SCRIPT_WAITING : u32 = 3;
pub const PI_SCRIPT_FAILED : u32 = 4;
/* signum: 0-63 */
pub const PI_MIN_SIGNUM : u32 = 0;
pub const PI_MAX_SIGNUM : u32 = 63;
/* timetype: 0-1 */
pub const PI_TIME_RELATIVE : u32 = 0;
pub const PI_TIME_ABSOLUTE : u32 = 1;
pub const PI_MAX_MICS_DELAY : u32 = 1000000; /* 1 second */
pub const PI_MAX_MILS_DELAY : u32 = 60000  ; /* 60 seconds */
/* cfgMillis */
pub const PI_BUF_MILLIS_MIN : u32 = 100;
pub const PI_BUF_MILLIS_MAX : u32 = 10000;
/* cfgMicros: 1, 2, 4, 5, 8, or 10 */
/* cfgPeripheral: 0-1 */
pub const PI_CLOCK_PWM : u32 = 0;
pub const PI_CLOCK_PCM : u32 = 1;
/* DMA channel: 0-15, 15 is unset */
pub const PI_MIN_DMA_CHANNEL : u32 = 0;
pub const PI_MAX_DMA_CHANNEL : u32 = 15;
/* port */
pub const PI_MIN_SOCKET_PORT : u32 = 1024;
pub const PI_MAX_SOCKET_PORT : u32 = 32000;
/* ifFlags: */
pub const PI_DISABLE_FIFO_IF : u32 = 1;
pub const PI_DISABLE_SOCK_IF : u32 = 2;
pub const PI_LOCALHOST_SOCK_IF : u32 = 4;
pub const PI_DISABLE_ALERT : u32 = 8;
/* memAllocMode */
pub const PI_MEM_ALLOC_AUTO : u32 = 0;
pub const PI_MEM_ALLOC_PAGEMAP : u32 = 1;
pub const PI_MEM_ALLOC_MAILBOX : u32 = 2;
/* filters */
pub const PI_MAX_STEADY : u32 = 300000;
pub const PI_MAX_ACTIVE : u32 = 1000000;
/* gpioCfgInternals */
pub const PI_CFG_DBG_LEVEL : u32 = 0; /* bits 0-3 */
pub const PI_CFG_ALERT_FREQ : u32 = 4; /* bits 4-7 */
pub const PI_CFG_RT_PRIORITY : u32 = (1<<8);
pub const PI_CFG_STATS : u32 = (1<<9);
pub const PI_CFG_NOSIGHANDLER : u32 = (1<<10);
pub const PI_CFG_ILLEGAL_VAL : u32 = (1<<11);
/* gpioISR */
/* pads */
pub const PI_MAX_PAD : u32 = 2;
pub const PI_MIN_PAD_STRENGTH : u32 = 1;
pub const PI_MAX_PAD_STRENGTH : u32 = 16;
/* files */
pub const PI_FILE_NONE : u32 = 0;
pub const PI_FILE_MIN : u32 = 1;
pub const PI_FILE_READ : u32 = 1;
pub const PI_FILE_WRITE : u32 = 2;
pub const PI_FILE_RW : u32 = 3;
pub const PI_FILE_APPEND : u32 = 4;
pub const PI_FILE_CREATE : u32 = 8;
pub const PI_FILE_TRUNC : u32 = 16;
pub const PI_FILE_MAX : u32 = 31;
pub const PI_FROM_START : u32 = 0;
pub const PI_FROM_CURRENT : u32 = 1;
pub const PI_FROM_END : u32 = 2;
/* Allowed socket connect addresses */
/* events */
pub const PI_MAX_EVENT : u32 = 31;
/* Event auto generated on BSC slave activity */
pub const PI_EVENT_BSC : u32 = 31;
/*DEF_S Socket Command Codes*/
pub const PI_CMD_MODES : u32 = 0;
pub const PI_CMD_MODEG : u32 = 1;
pub const PI_CMD_PUD : u32 = 2;
pub const PI_CMD_READ : u32 = 3;
pub const PI_CMD_WRITE : u32 = 4;
pub const PI_CMD_PWM : u32 = 5;
pub const PI_CMD_PRS : u32 = 6;
pub const PI_CMD_PFS : u32 = 7;
pub const PI_CMD_SERVO : u32 = 8;
pub const PI_CMD_WDOG : u32 = 9;
pub const PI_CMD_BR1 : u32 = 10;
pub const PI_CMD_BR2 : u32 = 11;
pub const PI_CMD_BC1 : u32 = 12;
pub const PI_CMD_BC2 : u32 = 13;
pub const PI_CMD_BS1 : u32 = 14;
pub const PI_CMD_BS2 : u32 = 15;
pub const PI_CMD_TICK : u32 = 16;
pub const PI_CMD_HWVER : u32 = 17;
pub const PI_CMD_NO : u32 = 18;
pub const PI_CMD_NB : u32 = 19;
pub const PI_CMD_NP : u32 = 20;
pub const PI_CMD_NC : u32 = 21;
pub const PI_CMD_PRG : u32 = 22;
pub const PI_CMD_PFG : u32 = 23;
pub const PI_CMD_PRRG : u32 = 24;
pub const PI_CMD_HELP : u32 = 25;
pub const PI_CMD_PIGPV : u32 = 26;
pub const PI_CMD_WVCLR : u32 = 27;
pub const PI_CMD_WVAG : u32 = 28;
pub const PI_CMD_WVAS : u32 = 29;
pub const PI_CMD_WVGO : u32 = 30;
pub const PI_CMD_WVGOR : u32 = 31;
pub const PI_CMD_WVBSY : u32 = 32;
pub const PI_CMD_WVHLT : u32 = 33;
pub const PI_CMD_WVSM : u32 = 34;
pub const PI_CMD_WVSP : u32 = 35;
pub const PI_CMD_WVSC : u32 = 36;
pub const PI_CMD_TRIG : u32 = 37;
pub const PI_CMD_PROC : u32 = 38;
pub const PI_CMD_PROCD : u32 = 39;
pub const PI_CMD_PROCR : u32 = 40;
pub const PI_CMD_PROCS : u32 = 41;
pub const PI_CMD_SLRO : u32 = 42;
pub const PI_CMD_SLR : u32 = 43;
pub const PI_CMD_SLRC : u32 = 44;
pub const PI_CMD_PROCP : u32 = 45;
pub const PI_CMD_MICS : u32 = 46;
pub const PI_CMD_MILS : u32 = 47;
pub const PI_CMD_PARSE : u32 = 48;
pub const PI_CMD_WVCRE : u32 = 49;
pub const PI_CMD_WVDEL : u32 = 50;
pub const PI_CMD_WVTX : u32 = 51;
pub const PI_CMD_WVTXR : u32 = 52;
pub const PI_CMD_WVNEW : u32 = 53;
pub const PI_CMD_I2CO : u32 = 54;
pub const PI_CMD_I2CC : u32 = 55;
pub const PI_CMD_I2CRD : u32 = 56;
pub const PI_CMD_I2CWD : u32 = 57;
pub const PI_CMD_I2CWQ : u32 = 58;
pub const PI_CMD_I2CRS : u32 = 59;
pub const PI_CMD_I2CWS : u32 = 60;
pub const PI_CMD_I2CRB : u32 = 61;
pub const PI_CMD_I2CWB : u32 = 62;
pub const PI_CMD_I2CRW : u32 = 63;
pub const PI_CMD_I2CWW : u32 = 64;
pub const PI_CMD_I2CRK : u32 = 65;
pub const PI_CMD_I2CWK : u32 = 66;
pub const PI_CMD_I2CRI : u32 = 67;
pub const PI_CMD_I2CWI : u32 = 68;
pub const PI_CMD_I2CPC : u32 = 69;
pub const PI_CMD_I2CPK : u32 = 70;
pub const PI_CMD_SPIO : u32 = 71;
pub const PI_CMD_SPIC : u32 = 72;
pub const PI_CMD_SPIR : u32 = 73;
pub const PI_CMD_SPIW : u32 = 74;
pub const PI_CMD_SPIX : u32 = 75;
pub const PI_CMD_SERO : u32 = 76;
pub const PI_CMD_SERC : u32 = 77;
pub const PI_CMD_SERRB : u32 = 78;
pub const PI_CMD_SERWB : u32 = 79;
pub const PI_CMD_SERR : u32 = 80;
pub const PI_CMD_SERW : u32 = 81;
pub const PI_CMD_SERDA : u32 = 82;
pub const PI_CMD_GDC : u32 = 83;
pub const PI_CMD_GPW : u32 = 84;
pub const PI_CMD_HC : u32 = 85;
pub const PI_CMD_HP : u32 = 86;
pub const PI_CMD_CF1 : u32 = 87;
pub const PI_CMD_CF2 : u32 = 88;
pub const PI_CMD_BI2CC : u32 = 89;
pub const PI_CMD_BI2CO : u32 = 90;
pub const PI_CMD_BI2CZ : u32 = 91;
pub const PI_CMD_I2CZ : u32 = 92;
pub const PI_CMD_WVCHA : u32 = 93;
pub const PI_CMD_SLRI : u32 = 94;
pub const PI_CMD_CGI : u32 = 95;
pub const PI_CMD_CSI : u32 = 96;
pub const PI_CMD_FG : u32 = 97;
pub const PI_CMD_FN : u32 = 98;
pub const PI_CMD_NOIB : u32 = 99;
pub const PI_CMD_WVTXM : u32 = 100;
pub const PI_CMD_WVTAT : u32 = 101;
pub const PI_CMD_PADS : u32 = 102;
pub const PI_CMD_PADG : u32 = 103;
pub const PI_CMD_FO : u32 = 104;
pub const PI_CMD_FC : u32 = 105;
pub const PI_CMD_FR : u32 = 106;
pub const PI_CMD_FW : u32 = 107;
pub const PI_CMD_FS : u32 = 108;
pub const PI_CMD_FL : u32 = 109;
pub const PI_CMD_SHELL : u32 = 110;
pub const PI_CMD_BSPIC : u32 = 111;
pub const PI_CMD_BSPIO : u32 = 112;
pub const PI_CMD_BSPIX : u32 = 113;
pub const PI_CMD_BSCX : u32 = 114;
pub const PI_CMD_EVM : u32 = 115;
pub const PI_CMD_EVT : u32 = 116;
pub const PI_CMD_PROCU : u32 = 117;
/*DEF_E*/
/* pseudo commands */
pub const PI_CMD_SCRIPT : u32 = 800;
pub const PI_CMD_ADD : u32 = 800;
pub const PI_CMD_AND : u32 = 801;
pub const PI_CMD_CALL : u32 = 802;
pub const PI_CMD_CMDR : u32 = 803;
pub const PI_CMD_CMDW : u32 = 804;
pub const PI_CMD_CMP : u32 = 805;
pub const PI_CMD_DCR : u32 = 806;
pub const PI_CMD_DCRA : u32 = 807;
pub const PI_CMD_DIV : u32 = 808;
pub const PI_CMD_HALT : u32 = 809;
pub const PI_CMD_INR : u32 = 810;
pub const PI_CMD_INRA : u32 = 811;
pub const PI_CMD_JM : u32 = 812;
pub const PI_CMD_JMP : u32 = 813;
pub const PI_CMD_JNZ : u32 = 814;
pub const PI_CMD_JP : u32 = 815;
pub const PI_CMD_JZ : u32 = 816;
pub const PI_CMD_TAG : u32 = 817;
pub const PI_CMD_LD : u32 = 818;
pub const PI_CMD_LDA : u32 = 819;
pub const PI_CMD_LDAB : u32 = 820;
pub const PI_CMD_MLT : u32 = 821;
pub const PI_CMD_MOD : u32 = 822;
pub const PI_CMD_NOP : u32 = 823;
pub const PI_CMD_OR : u32 = 824;
pub const PI_CMD_POP : u32 = 825;
pub const PI_CMD_POPA : u32 = 826;
pub const PI_CMD_PUSH : u32 = 827;
pub const PI_CMD_PUSHA : u32 = 828;
pub const PI_CMD_RET : u32 = 829;
pub const PI_CMD_RL : u32 = 830;
pub const PI_CMD_RLA : u32 = 831;
pub const PI_CMD_RR : u32 = 832;
pub const PI_CMD_RRA : u32 = 833;
pub const PI_CMD_STA : u32 = 834;
pub const PI_CMD_STAB : u32 = 835;
pub const PI_CMD_SUB : u32 = 836;
pub const PI_CMD_SYS : u32 = 837;
pub const PI_CMD_WAIT : u32 = 838;
pub const PI_CMD_X : u32 = 839;
pub const PI_CMD_XA : u32 = 840;
pub const PI_CMD_XOR : u32 = 841;
pub const PI_CMD_EVTWT : u32 = 842;
/*DEF_S Error Codes*/
pub const PI_INIT_FAILED : i32 = -1; // gpioInitialise failed
pub const PI_BAD_USER_GPIO : i32 = -2; // GPIO not 0-31
pub const PI_BAD_GPIO : i32 = -3; // GPIO not 0-53
pub const PI_BAD_MODE : i32 = -4; // mode not 0-7
pub const PI_BAD_LEVEL : i32 = -5; // level not 0-1
pub const PI_BAD_PUD : i32 = -6; // pud not 0-2
pub const PI_BAD_PULSEWIDTH : i32 = -7; // pulsewidth not 0 or 500-2500
pub const PI_BAD_DUTYCYCLE : i32 = -8; // dutycycle outside set range
pub const PI_BAD_TIMER : i32 = -9; // timer not 0-9
pub const PI_BAD_MS : i32 = -10; // ms not 10-60000
pub const PI_BAD_TIMETYPE : i32 = -11; // timetype not 0-1
pub const PI_BAD_SECONDS : i32 = -12; // seconds < 0
pub const PI_BAD_MICROS : i32 = -13; // micros not 0-999999
pub const PI_TIMER_FAILED : i32 = -14; // gpioSetTimerFunc failed
pub const PI_BAD_WDOG_TIMEOUT : i32 = -15; // timeout not 0-60000
pub const PI_NO_ALERT_FUNC : i32 = -16; // DEPRECATED
pub const PI_BAD_CLK_PERIPH : i32 = -17; // clock peripheral not 0-1
pub const PI_BAD_CLK_SOURCE : i32 = -18; // DEPRECATED
pub const PI_BAD_CLK_MICROS : i32 = -19; // clock micros not 1, 2, 4, 5, 8, or 10
pub const PI_BAD_BUF_MILLIS : i32 = -20; // buf millis not 100-10000
pub const PI_BAD_DUTYRANGE : i32 = -21; // dutycycle range not 25-40000
pub const PI_BAD_DUTY_RANGE : i32 = -21; // DEPRECATED (use PI_BAD_DUTYRANGE)
pub const PI_BAD_SIGNUM : i32 = -22; // signum not 0-63
pub const PI_BAD_PATHNAME : i32 = -23; // can't open pathname
pub const PI_NO_HANDLE : i32 = -24; // no handle available
pub const PI_BAD_HANDLE : i32 = -25; // unknown handle
pub const PI_BAD_IF_FLAGS : i32 = -26; // ifFlags > 4
pub const PI_BAD_CHANNEL : i32 = -27; // DMA channel not 0-15
pub const PI_BAD_PRIM_CHANNEL : i32 = -27; // DMA primary channel not 0-15
pub const PI_BAD_SOCKET_PORT : i32 = -28; // socket port not 1024-32000
pub const PI_BAD_FIFO_COMMAND : i32 = -29; // unrecognized fifo command
pub const PI_BAD_SECO_CHANNEL : i32 = -30; // DMA secondary channel not 0-15
pub const PI_NOT_INITIALISED : i32 = -31; // function called before gpioInitialise
pub const PI_INITIALISED : i32 = -32; // function called after gpioInitialise
pub const PI_BAD_WAVE_MODE : i32 = -33; // waveform mode not 0-3
pub const PI_BAD_CFG_INTERNAL : i32 = -34; // bad parameter in gpioCfgInternals call
pub const PI_BAD_WAVE_BAUD : i32 = -35; // baud rate not 50-250K(RX)/50-1M(TX)
pub const PI_TOO_MANY_PULSES : i32 = -36; // waveform has too many pulses
pub const PI_TOO_MANY_CHARS : i32 = -37; // waveform has too many chars
pub const PI_NOT_SERIAL_GPIO : i32 = -38; // no bit bang serial read on GPIO
pub const PI_BAD_SERIAL_STRUC : i32 = -39; // bad (null) serial structure parameter
pub const PI_BAD_SERIAL_BUF : i32 = -40; // bad (null) serial buf parameter
pub const PI_NOT_PERMITTED : i32 = -41; // GPIO operation not permitted
pub const PI_SOME_PERMITTED : i32 = -42; // one or more GPIO not permitted
pub const PI_BAD_WVSC_COMMND : i32 = -43; // bad WVSC subcommand
pub const PI_BAD_WVSM_COMMND : i32 = -44; // bad WVSM subcommand
pub const PI_BAD_WVSP_COMMND : i32 = -45; // bad WVSP subcommand
pub const PI_BAD_PULSELEN : i32 = -46; // trigger pulse length not 1-100
pub const PI_BAD_SCRIPT : i32 = -47; // invalid script
pub const PI_BAD_SCRIPT_ID : i32 = -48; // unknown script id
pub const PI_BAD_SER_OFFSET : i32 = -49; // add serial data offset > 30 minutes
pub const PI_GPIO_IN_USE : i32 = -50; // GPIO already in use
pub const PI_BAD_SERIAL_COUNT : i32 = -51; // must read at least a byte at a time
pub const PI_BAD_PARAM_NUM : i32 = -52; // script parameter id not 0-9
pub const PI_DUP_TAG : i32 = -53; // script has duplicate tag
pub const PI_TOO_MANY_TAGS : i32 = -54; // script has too many tags
pub const PI_BAD_SCRIPT_CMD : i32 = -55; // illegal script command
pub const PI_BAD_VAR_NUM : i32 = -56; // script variable id not 0-149
pub const PI_NO_SCRIPT_ROOM : i32 = -57; // no more room for scripts
pub const PI_NO_MEMORY : i32 = -58; // can't allocate temporary memory
pub const PI_SOCK_READ_FAILED : i32 = -59; // socket read failed
pub const PI_SOCK_WRIT_FAILED : i32 = -60; // socket write failed
pub const PI_TOO_MANY_PARAM : i32 = -61; // too many script parameters (> 10)
pub const PI_NOT_HALTED : i32 = -62; // DEPRECATED
pub const PI_SCRIPT_NOT_READY : i32 = -62; // script initialising
pub const PI_BAD_TAG : i32 = -63; // script has unresolved tag
pub const PI_BAD_MICS_DELAY : i32 = -64; // bad MICS delay (too large)
pub const PI_BAD_MILS_DELAY : i32 = -65; // bad MILS delay (too large)
pub const PI_BAD_WAVE_ID : i32 = -66; // non existent wave id
pub const PI_TOO_MANY_CBS : i32 = -67; // No more CBs for waveform
pub const PI_TOO_MANY_OOL : i32 = -68; // No more OOL for waveform
pub const PI_EMPTY_WAVEFORM : i32 = -69; // attempt to create an empty waveform
pub const PI_NO_WAVEFORM_ID : i32 = -70; // no more waveforms
pub const PI_I2C_OPEN_FAILED : i32 = -71; // can't open I2C device
pub const PI_SER_OPEN_FAILED : i32 = -72; // can't open serial device
pub const PI_SPI_OPEN_FAILED : i32 = -73; // can't open SPI device
pub const PI_BAD_I2C_BUS : i32 = -74; // bad I2C bus
pub const PI_BAD_I2C_ADDR : i32 = -75; // bad I2C address
pub const PI_BAD_SPI_CHANNEL : i32 = -76; // bad SPI channel
pub const PI_BAD_FLAGS : i32 = -77; // bad i2c/spi/ser open flags
pub const PI_BAD_SPI_SPEED : i32 = -78; // bad SPI speed
pub const PI_BAD_SER_DEVICE : i32 = -79; // bad serial device name
pub const PI_BAD_SER_SPEED : i32 = -80; // bad serial baud rate
pub const PI_BAD_PARAM : i32 = -81; // bad i2c/spi/ser parameter
pub const PI_I2C_WRITE_FAILED : i32 = -82; // i2c write failed
pub const PI_I2C_READ_FAILED : i32 = -83; // i2c read failed
pub const PI_BAD_SPI_COUNT : i32 = -84; // bad SPI count
pub const PI_SER_WRITE_FAILED : i32 = -85; // ser write failed
pub const PI_SER_READ_FAILED : i32 = -86; // ser read failed
pub const PI_SER_READ_NO_DATA : i32 = -87; // ser read no data available
pub const PI_UNKNOWN_COMMAND : i32 = -88; // unknown command
pub const PI_SPI_XFER_FAILED : i32 = -89; // spi xfer/read/write failed
pub const PI_BAD_POINTER : i32 = -90; // bad (NULL) pointer
pub const PI_NO_AUX_SPI : i32 = -91; // no auxiliary SPI on Pi A or B
pub const PI_NOT_PWM_GPIO : i32 = -92; // GPIO is not in use for PWM
pub const PI_NOT_SERVO_GPIO : i32 = -93; // GPIO is not in use for servo pulses
pub const PI_NOT_HCLK_GPIO : i32 = -94; // GPIO has no hardware clock
pub const PI_NOT_HPWM_GPIO : i32 = -95; // GPIO has no hardware PWM
pub const PI_BAD_HPWM_FREQ : i32 = -96; // invalid hardware PWM frequency
pub const PI_BAD_HPWM_DUTY : i32 = -97; // hardware PWM dutycycle not 0-1M
pub const PI_BAD_HCLK_FREQ : i32 = -98; // invalid hardware clock frequency
pub const PI_BAD_HCLK_PASS : i32 = -99; // need password to use hardware clock 1
pub const PI_HPWM_ILLEGAL : i32 = -100; // illegal, PWM in use for main clock
pub const PI_BAD_DATABITS : i32 = -101; // serial data bits not 1-32
pub const PI_BAD_STOPBITS : i32 = -102; // serial (half) stop bits not 2-8
pub const PI_MSG_TOOBIG : i32 = -103; // socket/pipe message too big
pub const PI_BAD_MALLOC_MODE : i32 = -104; // bad memory allocation mode
pub const PI_TOO_MANY_SEGS : i32 = -105; // too many I2C transaction segments
pub const PI_BAD_I2C_SEG : i32 = -106; // an I2C transaction segment failed
pub const PI_BAD_SMBUS_CMD : i32 = -107; // SMBus command not supported by driver
pub const PI_NOT_I2C_GPIO : i32 = -108; // no bit bang I2C in progress on GPIO
pub const PI_BAD_I2C_WLEN : i32 = -109; // bad I2C write length
pub const PI_BAD_I2C_RLEN : i32 = -110; // bad I2C read length
pub const PI_BAD_I2C_CMD : i32 = -111; // bad I2C command
pub const PI_BAD_I2C_BAUD : i32 = -112; // bad I2C baud rate, not 50-500k
pub const PI_CHAIN_LOOP_CNT : i32 = -113; // bad chain loop count
pub const PI_BAD_CHAIN_LOOP : i32 = -114; // empty chain loop
pub const PI_CHAIN_COUNTER : i32 = -115; // too many chain counters
pub const PI_BAD_CHAIN_CMD : i32 = -116; // bad chain command
pub const PI_BAD_CHAIN_DELAY : i32 = -117; // bad chain delay micros
pub const PI_CHAIN_NESTING : i32 = -118; // chain counters nested too deeply
pub const PI_CHAIN_TOO_BIG : i32 = -119; // chain is too long
pub const PI_DEPRECATED : i32 = -120; // deprecated function removed
pub const PI_BAD_SER_INVERT : i32 = -121; // bit bang serial invert not 0 or 1
pub const PI_BAD_EDGE : i32 = -122; // bad ISR edge value, not 0-2
pub const PI_BAD_ISR_INIT : i32 = -123; // bad ISR initialisation
pub const PI_BAD_FOREVER : i32 = -124; // loop forever must be last command
pub const PI_BAD_FILTER : i32 = -125; // bad filter parameter
pub const PI_BAD_PAD : i32 = -126; // bad pad number
pub const PI_BAD_STRENGTH : i32 = -127; // bad pad drive strength
pub const PI_FIL_OPEN_FAILED : i32 = -128; // file open failed
pub const PI_BAD_FILE_MODE : i32 = -129; // bad file mode
pub const PI_BAD_FILE_FLAG : i32 = -130; // bad file flag
pub const PI_BAD_FILE_READ : i32 = -131; // bad file read
pub const PI_BAD_FILE_WRITE : i32 = -132; // bad file write
pub const PI_FILE_NOT_ROPEN : i32 = -133; // file not open for read
pub const PI_FILE_NOT_WOPEN : i32 = -134; // file not open for write
pub const PI_BAD_FILE_SEEK : i32 = -135; // bad file seek
pub const PI_NO_FILE_MATCH : i32 = -136; // no files match pattern
pub const PI_NO_FILE_ACCESS : i32 = -137; // no permission to access file
pub const PI_FILE_IS_A_DIR : i32 = -138; // file is a directory
pub const PI_BAD_SHELL_STATUS : i32 = -139; // bad shell return status
pub const PI_BAD_SCRIPT_NAME : i32 = -140; // bad script name
pub const PI_BAD_SPI_BAUD : i32 = -141; // bad SPI baud rate, not 50-500k
pub const PI_NOT_SPI_GPIO : i32 = -142; // no bit bang SPI in progress on GPIO
pub const PI_BAD_EVENT_ID : i32 = -143; // bad event id
pub const PI_CMD_INTERRUPTED : i32 = -144; // Used by Python
pub const PI_NOT_ON_BCM2711 : i32 = -145; // not available on BCM2711
pub const PI_ONLY_ON_BCM2711 : i32 = -146; // only available on BCM2711
pub const PI_PIGIF_ERR_0 : i32 = -2000;
pub const PI_PIGIF_ERR_99 : i32 = -2099;
pub const PI_CUSTOM_ERR_0 : i32 = -3000;
pub const PI_CUSTOM_ERR_999 : i32 = -3999;
/*DEF_E*/
/*DEF_S Defaults*/
/*DEF_E*/
#[allow(non_snake_case)]
pub fn PI_error_code_lookup(val : i32)
                            -> Option<(&'static str, &'static str)> {
  let r = match val {
    PI_INIT_FAILED => ("PI_INIT_FAILED", "gpioInitialise failed"),
    PI_BAD_USER_GPIO => ("PI_BAD_USER_GPIO", "GPIO not 0-31"),
    PI_BAD_GPIO => ("PI_BAD_GPIO", "GPIO not 0-53"),
    PI_BAD_MODE => ("PI_BAD_MODE", "mode not 0-7"),
    PI_BAD_LEVEL => ("PI_BAD_LEVEL", "level not 0-1"),
    PI_BAD_PUD => ("PI_BAD_PUD", "pud not 0-2"),
    PI_BAD_PULSEWIDTH => ("PI_BAD_PULSEWIDTH", "pulsewidth not 0 or 500-2500"),
    PI_BAD_DUTYCYCLE => ("PI_BAD_DUTYCYCLE", "dutycycle outside set range"),
    PI_BAD_TIMER => ("PI_BAD_TIMER", "timer not 0-9"),
    PI_BAD_MS => ("PI_BAD_MS", "ms not 10-60000"),
    PI_BAD_TIMETYPE => ("PI_BAD_TIMETYPE", "timetype not 0-1"),
    PI_BAD_SECONDS => ("PI_BAD_SECONDS", "seconds < 0"),
    PI_BAD_MICROS => ("PI_BAD_MICROS", "micros not 0-999999"),
    PI_TIMER_FAILED => ("PI_TIMER_FAILED", "gpioSetTimerFunc failed"),
    PI_BAD_WDOG_TIMEOUT => ("PI_BAD_WDOG_TIMEOUT", "timeout not 0-60000"),
    PI_BAD_CLK_PERIPH => ("PI_BAD_CLK_PERIPH", "clock peripheral not 0-1"),
    PI_BAD_CLK_MICROS => ("PI_BAD_CLK_MICROS", "clock micros not 1, 2, 4, 5, 8, or 10"),
    PI_BAD_BUF_MILLIS => ("PI_BAD_BUF_MILLIS", "buf millis not 100-10000"),
    PI_BAD_DUTYRANGE => ("PI_BAD_DUTYRANGE", "dutycycle range not 25-40000"),
    PI_BAD_SIGNUM => ("PI_BAD_SIGNUM", "signum not 0-63"),
    PI_BAD_PATHNAME => ("PI_BAD_PATHNAME", "can't open pathname"),
    PI_NO_HANDLE => ("PI_NO_HANDLE", "no handle available"),
    PI_BAD_HANDLE => ("PI_BAD_HANDLE", "unknown handle"),
    PI_BAD_IF_FLAGS => ("PI_BAD_IF_FLAGS", "ifFlags > 4"),
    PI_BAD_CHANNEL => ("PI_BAD_CHANNEL", "DMA channel not 0-15"),
    PI_BAD_SOCKET_PORT => ("PI_BAD_SOCKET_PORT", "socket port not 1024-32000"),
    PI_BAD_FIFO_COMMAND => ("PI_BAD_FIFO_COMMAND", "unrecognized fifo command"),
    PI_BAD_SECO_CHANNEL => ("PI_BAD_SECO_CHANNEL", "DMA secondary channel not 0-15"),
    PI_NOT_INITIALISED => ("PI_NOT_INITIALISED", "function called before gpioInitialise"),
    PI_INITIALISED => ("PI_INITIALISED", "function called after gpioInitialise"),
    PI_BAD_WAVE_MODE => ("PI_BAD_WAVE_MODE", "waveform mode not 0-3"),
    PI_BAD_CFG_INTERNAL => ("PI_BAD_CFG_INTERNAL", "bad parameter in gpioCfgInternals call"),
    PI_BAD_WAVE_BAUD => ("PI_BAD_WAVE_BAUD", "baud rate not 50-250K(RX)/50-1M(TX)"),
    PI_TOO_MANY_PULSES => ("PI_TOO_MANY_PULSES", "waveform has too many pulses"),
    PI_TOO_MANY_CHARS => ("PI_TOO_MANY_CHARS", "waveform has too many chars"),
    PI_NOT_SERIAL_GPIO => ("PI_NOT_SERIAL_GPIO", "no bit bang serial read on GPIO"),
    PI_BAD_SERIAL_STRUC => ("PI_BAD_SERIAL_STRUC", "bad (null) serial structure parameter"),
    PI_BAD_SERIAL_BUF => ("PI_BAD_SERIAL_BUF", "bad (null) serial buf parameter"),
    PI_NOT_PERMITTED => ("PI_NOT_PERMITTED", "GPIO operation not permitted"),
    PI_SOME_PERMITTED => ("PI_SOME_PERMITTED", "one or more GPIO not permitted"),
    PI_BAD_WVSC_COMMND => ("PI_BAD_WVSC_COMMND", "bad WVSC subcommand"),
    PI_BAD_WVSM_COMMND => ("PI_BAD_WVSM_COMMND", "bad WVSM subcommand"),
    PI_BAD_WVSP_COMMND => ("PI_BAD_WVSP_COMMND", "bad WVSP subcommand"),
    PI_BAD_PULSELEN => ("PI_BAD_PULSELEN", "trigger pulse length not 1-100"),
    PI_BAD_SCRIPT => ("PI_BAD_SCRIPT", "invalid script"),
    PI_BAD_SCRIPT_ID => ("PI_BAD_SCRIPT_ID", "unknown script id"),
    PI_BAD_SER_OFFSET => ("PI_BAD_SER_OFFSET", "add serial data offset > 30 minutes"),
    PI_GPIO_IN_USE => ("PI_GPIO_IN_USE", "GPIO already in use"),
    PI_BAD_SERIAL_COUNT => ("PI_BAD_SERIAL_COUNT", "must read at least a byte at a time"),
    PI_BAD_PARAM_NUM => ("PI_BAD_PARAM_NUM", "script parameter id not 0-9"),
    PI_DUP_TAG => ("PI_DUP_TAG", "script has duplicate tag"),
    PI_TOO_MANY_TAGS => ("PI_TOO_MANY_TAGS", "script has too many tags"),
    PI_BAD_SCRIPT_CMD => ("PI_BAD_SCRIPT_CMD", "illegal script command"),
    PI_BAD_VAR_NUM => ("PI_BAD_VAR_NUM", "script variable id not 0-149"),
    PI_NO_SCRIPT_ROOM => ("PI_NO_SCRIPT_ROOM", "no more room for scripts"),
    PI_NO_MEMORY => ("PI_NO_MEMORY", "can't allocate temporary memory"),
    PI_SOCK_READ_FAILED => ("PI_SOCK_READ_FAILED", "socket read failed"),
    PI_SOCK_WRIT_FAILED => ("PI_SOCK_WRIT_FAILED", "socket write failed"),
    PI_TOO_MANY_PARAM => ("PI_TOO_MANY_PARAM", "too many script parameters (> 10)"),
    PI_SCRIPT_NOT_READY => ("PI_SCRIPT_NOT_READY", "script initialising"),
    PI_BAD_TAG => ("PI_BAD_TAG", "script has unresolved tag"),
    PI_BAD_MICS_DELAY => ("PI_BAD_MICS_DELAY", "bad MICS delay (too large)"),
    PI_BAD_MILS_DELAY => ("PI_BAD_MILS_DELAY", "bad MILS delay (too large)"),
    PI_BAD_WAVE_ID => ("PI_BAD_WAVE_ID", "non existent wave id"),
    PI_TOO_MANY_CBS => ("PI_TOO_MANY_CBS", "No more CBs for waveform"),
    PI_TOO_MANY_OOL => ("PI_TOO_MANY_OOL", "No more OOL for waveform"),
    PI_EMPTY_WAVEFORM => ("PI_EMPTY_WAVEFORM", "attempt to create an empty waveform"),
    PI_NO_WAVEFORM_ID => ("PI_NO_WAVEFORM_ID", "no more waveforms"),
    PI_I2C_OPEN_FAILED => ("PI_I2C_OPEN_FAILED", "can't open I2C device"),
    PI_SER_OPEN_FAILED => ("PI_SER_OPEN_FAILED", "can't open serial device"),
    PI_SPI_OPEN_FAILED => ("PI_SPI_OPEN_FAILED", "can't open SPI device"),
    PI_BAD_I2C_BUS => ("PI_BAD_I2C_BUS", "bad I2C bus"),
    PI_BAD_I2C_ADDR => ("PI_BAD_I2C_ADDR", "bad I2C address"),
    PI_BAD_SPI_CHANNEL => ("PI_BAD_SPI_CHANNEL", "bad SPI channel"),
    PI_BAD_FLAGS => ("PI_BAD_FLAGS", "bad i2c/spi/ser open flags"),
    PI_BAD_SPI_SPEED => ("PI_BAD_SPI_SPEED", "bad SPI speed"),
    PI_BAD_SER_DEVICE => ("PI_BAD_SER_DEVICE", "bad serial device name"),
    PI_BAD_SER_SPEED => ("PI_BAD_SER_SPEED", "bad serial baud rate"),
    PI_BAD_PARAM => ("PI_BAD_PARAM", "bad i2c/spi/ser parameter"),
    PI_I2C_WRITE_FAILED => ("PI_I2C_WRITE_FAILED", "i2c write failed"),
    PI_I2C_READ_FAILED => ("PI_I2C_READ_FAILED", "i2c read failed"),
    PI_BAD_SPI_COUNT => ("PI_BAD_SPI_COUNT", "bad SPI count"),
    PI_SER_WRITE_FAILED => ("PI_SER_WRITE_FAILED", "ser write failed"),
    PI_SER_READ_FAILED => ("PI_SER_READ_FAILED", "ser read failed"),
    PI_SER_READ_NO_DATA => ("PI_SER_READ_NO_DATA", "ser read no data available"),
    PI_UNKNOWN_COMMAND => ("PI_UNKNOWN_COMMAND", "unknown command"),
    PI_SPI_XFER_FAILED => ("PI_SPI_XFER_FAILED", "spi xfer/read/write failed"),
    PI_BAD_POINTER => ("PI_BAD_POINTER", "bad (NULL) pointer"),
    PI_NO_AUX_SPI => ("PI_NO_AUX_SPI", "no auxiliary SPI on Pi A or B"),
    PI_NOT_PWM_GPIO => ("PI_NOT_PWM_GPIO", "GPIO is not in use for PWM"),
    PI_NOT_SERVO_GPIO => ("PI_NOT_SERVO_GPIO", "GPIO is not in use for servo pulses"),
    PI_NOT_HCLK_GPIO => ("PI_NOT_HCLK_GPIO", "GPIO has no hardware clock"),
    PI_NOT_HPWM_GPIO => ("PI_NOT_HPWM_GPIO", "GPIO has no hardware PWM"),
    PI_BAD_HPWM_FREQ => ("PI_BAD_HPWM_FREQ", "invalid hardware PWM frequency"),
    PI_BAD_HPWM_DUTY => ("PI_BAD_HPWM_DUTY", "hardware PWM dutycycle not 0-1M"),
    PI_BAD_HCLK_FREQ => ("PI_BAD_HCLK_FREQ", "invalid hardware clock frequency"),
    PI_BAD_HCLK_PASS => ("PI_BAD_HCLK_PASS", "need password to use hardware clock 1"),
    PI_HPWM_ILLEGAL => ("PI_HPWM_ILLEGAL", "illegal, PWM in use for main clock"),
    PI_BAD_DATABITS => ("PI_BAD_DATABITS", "serial data bits not 1-32"),
    PI_BAD_STOPBITS => ("PI_BAD_STOPBITS", "serial (half) stop bits not 2-8"),
    PI_MSG_TOOBIG => ("PI_MSG_TOOBIG", "socket/pipe message too big"),
    PI_BAD_MALLOC_MODE => ("PI_BAD_MALLOC_MODE", "bad memory allocation mode"),
    PI_TOO_MANY_SEGS => ("PI_TOO_MANY_SEGS", "too many I2C transaction segments"),
    PI_BAD_I2C_SEG => ("PI_BAD_I2C_SEG", "an I2C transaction segment failed"),
    PI_BAD_SMBUS_CMD => ("PI_BAD_SMBUS_CMD", "SMBus command not supported by driver"),
    PI_NOT_I2C_GPIO => ("PI_NOT_I2C_GPIO", "no bit bang I2C in progress on GPIO"),
    PI_BAD_I2C_WLEN => ("PI_BAD_I2C_WLEN", "bad I2C write length"),
    PI_BAD_I2C_RLEN => ("PI_BAD_I2C_RLEN", "bad I2C read length"),
    PI_BAD_I2C_CMD => ("PI_BAD_I2C_CMD", "bad I2C command"),
    PI_BAD_I2C_BAUD => ("PI_BAD_I2C_BAUD", "bad I2C baud rate, not 50-500k"),
    PI_CHAIN_LOOP_CNT => ("PI_CHAIN_LOOP_CNT", "bad chain loop count"),
    PI_BAD_CHAIN_LOOP => ("PI_BAD_CHAIN_LOOP", "empty chain loop"),
    PI_CHAIN_COUNTER => ("PI_CHAIN_COUNTER", "too many chain counters"),
    PI_BAD_CHAIN_CMD => ("PI_BAD_CHAIN_CMD", "bad chain command"),
    PI_BAD_CHAIN_DELAY => ("PI_BAD_CHAIN_DELAY", "bad chain delay micros"),
    PI_CHAIN_NESTING => ("PI_CHAIN_NESTING", "chain counters nested too deeply"),
    PI_CHAIN_TOO_BIG => ("PI_CHAIN_TOO_BIG", "chain is too long"),
    PI_DEPRECATED => ("PI_DEPRECATED", "deprecated function removed"),
    PI_BAD_SER_INVERT => ("PI_BAD_SER_INVERT", "bit bang serial invert not 0 or 1"),
    PI_BAD_EDGE => ("PI_BAD_EDGE", "bad ISR edge value, not 0-2"),
    PI_BAD_ISR_INIT => ("PI_BAD_ISR_INIT", "bad ISR initialisation"),
    PI_BAD_FOREVER => ("PI_BAD_FOREVER", "loop forever must be last command"),
    PI_BAD_FILTER => ("PI_BAD_FILTER", "bad filter parameter"),
    PI_BAD_PAD => ("PI_BAD_PAD", "bad pad number"),
    PI_BAD_STRENGTH => ("PI_BAD_STRENGTH", "bad pad drive strength"),
    PI_FIL_OPEN_FAILED => ("PI_FIL_OPEN_FAILED", "file open failed"),
    PI_BAD_FILE_MODE => ("PI_BAD_FILE_MODE", "bad file mode"),
    PI_BAD_FILE_FLAG => ("PI_BAD_FILE_FLAG", "bad file flag"),
    PI_BAD_FILE_READ => ("PI_BAD_FILE_READ", "bad file read"),
    PI_BAD_FILE_WRITE => ("PI_BAD_FILE_WRITE", "bad file write"),
    PI_FILE_NOT_ROPEN => ("PI_FILE_NOT_ROPEN", "file not open for read"),
    PI_FILE_NOT_WOPEN => ("PI_FILE_NOT_WOPEN", "file not open for write"),
    PI_BAD_FILE_SEEK => ("PI_BAD_FILE_SEEK", "bad file seek"),
    PI_NO_FILE_MATCH => ("PI_NO_FILE_MATCH", "no files match pattern"),
    PI_NO_FILE_ACCESS => ("PI_NO_FILE_ACCESS", "no permission to access file"),
    PI_FILE_IS_A_DIR => ("PI_FILE_IS_A_DIR", "file is a directory"),
    PI_BAD_SHELL_STATUS => ("PI_BAD_SHELL_STATUS", "bad shell return status"),
    PI_BAD_SCRIPT_NAME => ("PI_BAD_SCRIPT_NAME", "bad script name"),
    PI_BAD_SPI_BAUD => ("PI_BAD_SPI_BAUD", "bad SPI baud rate, not 50-500k"),
    PI_NOT_SPI_GPIO => ("PI_NOT_SPI_GPIO", "no bit bang SPI in progress on GPIO"),
    PI_BAD_EVENT_ID => ("PI_BAD_EVENT_ID", "bad event id"),
    PI_CMD_INTERRUPTED => ("PI_CMD_INTERRUPTED", "Used by Python"),
    PI_NOT_ON_BCM2711 => ("PI_NOT_ON_BCM2711", "not available on BCM2711"),
    PI_ONLY_ON_BCM2711 => ("PI_ONLY_ON_BCM2711", "only available on BCM2711"),
    PI_PIGIF_ERR_0 => ("PI_PIGIF_ERR_0", ""),
    PI_PIGIF_ERR_99 => ("PI_PIGIF_ERR_99", ""),
    PI_CUSTOM_ERR_0 => ("PI_CUSTOM_ERR_0", ""),
    PI_CUSTOM_ERR_999 => ("PI_CUSTOM_ERR_999", ""),
    _ => { return None },
  };
  Some(r)
}
