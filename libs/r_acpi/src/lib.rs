#![no_std]

use {core::mem::size_of, static_assertions::const_assert_eq};

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rsdp {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oemid: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    pub length: u32,
    pub xsdt_address: u64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}
const_assert_eq!(size_of::<Rsdp>(), 36);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DescriptionHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oemid: [u8; 6],
    pub oem_table_id: u64,
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}
const_assert_eq!(size_of::<DescriptionHeader>(), 36);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Xsdt {
    pub header: DescriptionHeader,
    pub entry: [u64; 0],
}
const_assert_eq!(size_of::<Xsdt>(), 36);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fadt {
    pub header: DescriptionHeader,
    pub firmware_ctrl: u32,
    pub dsdt: u32,
    pub reserved_1: u8,
    pub preferred_pm_profile: u8,
    pub sci_int: u16,
    pub smi_cmd: u32,
    pub acpi_enable: u8,
    pub acpi_disable: u8,
    pub s4bios_req: u8,
    pub pstate_cnt: u8,
    pub pm1a_evt_blk: u32,
    pub pm1b_evt_blk: u32,
    pub pm1a_cnt_blk: u32,
    pub pm1b_cnt_blk: u32,
    pub pm2_cnt_blk: u32,
    pub pm_tmr_blk: u32,
    pub gpe0_blk: u32,
    pub gpe1_blk: u32,
    pub pm1_evt_len: u8,
    pub pm1_cnt_len: u8,
    pub pm2_cnt_len: u8,
    pub pm_tmr_len: u8,
    pub gpe0_blk_len: u8,
    pub gpe1_blk_len: u8,
    pub gpe1_base: u8,
    pub cst_cnt: u8,
    pub p_lvl2_lat: u16,
    pub p_lvl3_lat: u16,
    pub flush_size: u16,
    pub flush_stride: u16,
    pub duty_offset: u8,
    pub duty_width: u8,
    pub day_alrm: u8,
    pub mon_alrm: u8,
    pub century: u8,
    pub iapc_boot_arch: u16,
    pub reserved_2: u8,
    pub flags: u32,
    pub reset_reg: GenericAddressStructure,
    pub reset_value: u8,
    pub arm_boot_arch: u16,
    pub fadt_minor_version: u8,
    pub x_firmware_ctrl: u64,
    pub x_dsdt: u64,
    pub x_pm1a_evt_blk: GenericAddressStructure,
    pub x_pm1b_evt_blk: GenericAddressStructure,
    pub x_pm1a_cnt_blk: GenericAddressStructure,
    pub x_pm1b_cnt_blk: GenericAddressStructure,
    pub x_pm2_cnt_blk: GenericAddressStructure,
    pub x_pm_tmr_blk: GenericAddressStructure,
    pub x_gpe0_blk: GenericAddressStructure,
    pub x_gpe1_blk: GenericAddressStructure,
    pub sleep_control_reg: GenericAddressStructure,
    pub sleep_statue_reg: GenericAddressStructure,
    pub hypervisor_vendor_identity: u64,
}
const_assert_eq!(size_of::<Fadt>(), 276);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenericAddressStructure {
    pub address_space_id: u8,
    pub register_bit_width: u8,
    pub register_bit_offset: u8,
    pub access_size: u8,
    pub address: u64,
}
const_assert_eq!(size_of::<GenericAddressStructure>(), 12);

pub const SYSTEM_MEMORY_SPACE: u8 = 0x00;
pub const SYSTEM_IO_SPACE: u8 = 0x01;
pub const PCI_CONFIGURATION_SPACE: u8 = 0x02;
pub const EMBEDDED_CONTROLLER: u8 = 0x03;
pub const SM_BUS: u8 = 0x04;
pub const SYSTEM_CMOS: u8 = 0x05;
pub const PCI_BAR_TARGET: u8 = 0x06;
pub const IPMI: u8 = 0x07;
pub const GENERAL_PURPOSE_IO: u8 = 0x08;
pub const GENERIC_SERIAL_BUS: u8 = 0x09;
pub const PLATFORM_COMMUNICATIONS_CHANNEL: u8 = 0x0a;
pub const FUNCTIONAL_FIXED_HARDWARE: u8 = 0x7f;

pub const UNDEFINED: u8 = 0;
pub const BYTE_ACCESS: u8 = 1;
pub const WORD_ACCESS: u8 = 2;
pub const DWORD_ACCESS: u8 = 3;
pub const QWORD_ACCESS: u8 = 4;

pub const PM_TIMER_FREQUENCY_HZ: u32 = 3_579_545;
