// SPDX-License-Identifier: (GPL-2.0-or-later OR Apache-2.0)
// Copyright (c) Viacheslav Bocharov <v@baodeep.com> and JetHome (r)

//! ATM90E32 register address map.
//!
//! The ATM90E32 uses 10-bit register addresses, so all constants are `u16`.
//! Addresses are grouped by function: configuration, calibration, and
//! measurement.

// ── Configuration & control ─────────────────────────────────────────

pub const REG_METEREN: u16 = 0x00;
pub const REG_SYSSTATUS0: u16 = 0x01;
pub const REG_SAGPEAKDETCFG: u16 = 0x05;
pub const REG_ZXCONFIG: u16 = 0x07;
pub const REG_FREQLOTH: u16 = 0x0C;
pub const REG_FREQHITH: u16 = 0x0D;
pub const REG_PLCONSTH: u16 = 0x31;
pub const REG_PLCONSTL: u16 = 0x32;
pub const REG_MMODE0: u16 = 0x33;
pub const REG_MMODE1: u16 = 0x34;
pub const REG_PSTARTTH: u16 = 0x35;
pub const REG_QSTARTTH: u16 = 0x36;
pub const REG_SSTARTTH: u16 = 0x37;
pub const REG_PPHASETH: u16 = 0x38;
pub const REG_QPHASETH: u16 = 0x39;
pub const REG_SOFTRESET: u16 = 0x70;
pub const REG_CFGREGACCEN: u16 = 0x7F;

// ── Calibration: voltage gains ──────────────────────────────────────

pub const REG_UGAIN_A: u16 = 0x61;
pub const REG_UGAIN_B: u16 = 0x65;
pub const REG_UGAIN_C: u16 = 0x69;

// ── Calibration: current gains ──────────────────────────────────────

pub const REG_IGAIN_A: u16 = 0x62;
pub const REG_IGAIN_B: u16 = 0x66;
pub const REG_IGAIN_C: u16 = 0x6A;

// ── Measurement: voltage RMS ────────────────────────────────────────

pub const REG_URMS_A: u16 = 0xD9;
pub const REG_URMS_B: u16 = 0xDA;
pub const REG_URMS_C: u16 = 0xDB;

// ── Measurement: current RMS ────────────────────────────────────────

pub const REG_IRMS_A: u16 = 0xDD;
pub const REG_IRMS_B: u16 = 0xDE;
pub const REG_IRMS_C: u16 = 0xDF;

// ── Measurement: active power mean (high word) ──────────────────────

pub const REG_PMEAN_A: u16 = 0xB1;
pub const REG_PMEAN_B: u16 = 0xB2;
pub const REG_PMEAN_C: u16 = 0xB3;

// ── Measurement: reactive power mean (high word) ────────────────────

pub const REG_QMEAN_A: u16 = 0xB5;
pub const REG_QMEAN_B: u16 = 0xB6;
pub const REG_QMEAN_C: u16 = 0xB7;

// ── Measurement: active power mean (low word) ───────────────────────

pub const REG_PMEAN_A_LSB: u16 = 0xC1;
pub const REG_PMEAN_B_LSB: u16 = 0xC2;
pub const REG_PMEAN_C_LSB: u16 = 0xC3;

// ── Measurement: reactive power mean (low word) ─────────────────────

pub const REG_QMEAN_A_LSB: u16 = 0xC5;
pub const REG_QMEAN_B_LSB: u16 = 0xC6;
pub const REG_QMEAN_C_LSB: u16 = 0xC7;

// ── Measurement: power factor ───────────────────────────────────────

pub const REG_PFMEAN_A: u16 = 0xBD;
pub const REG_PFMEAN_B: u16 = 0xBE;
pub const REG_PFMEAN_C: u16 = 0xBF;

// ── Measurement: line frequency ─────────────────────────────────────

pub const REG_FREQ: u16 = 0xF8;
