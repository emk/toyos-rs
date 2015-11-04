//! Interface to our PCI devices.
//!
//! As usual, this is heavily inspired by http://wiki.osdev.org/Pci

use core::intrinsics::transmute;
use spin::Mutex;
use arch::x86_64::io;

struct Pci {
    address: io::Port<u32>,
    data: io::Port<u32>,
}

impl Pci {
    /// Read a 32-bit aligned word from PCI Configuration Address Space.
    /// This is marked as `unsafe` because passing in out-of-range
    /// parameters probably does excitingly horrible things to the
    /// hardware.
    unsafe fn read_config(&mut self, bus: u8, slot: u8, function: u8, offset: u8)
        -> u32
    {
        let address: u32 =
            0x80000000
            | (bus as u32) << 16
            | (slot as u32) << 11
            | (function as u32) << 8
            | (offset & 0b1111_1100) as u32;
        self.address.write(address);
        self.data.read()
    }

    /// Check for a PCI device, and return information about it if present.
    unsafe fn probe(
        &mut self, bus: u8, slot: u8, function: u8)
        -> Option<DeviceInfo>
    {
        let config_0 = self.read_config(bus, slot, function, 0);
        // We'll receive all 1's if no device is present.
        if config_0 == 0xFFFFFFFF { return None }

        let config_4 = self.read_config(bus, slot, function, 0x8);
        let config_c = self.read_config(bus, slot, function, 0xC);

        Some(DeviceInfo {
            vendor_id: config_0 as u16,
            device_id: (config_0 >> 16) as u16,
            revision_id: config_4 as u8,
            subclass: (config_4 >> 16) as u8,
            class_code: DeviceClass::from_u8((config_4 >> 24) as u8),
            multifunction: config_c & 0x800000 != 0,
        })
    }
}

#[derive(Debug)]
#[repr(u8)]
enum DeviceClass {
    Legacy = 0x00,
    MassStorage = 0x01,
    Network = 0x02,
    Display = 0x03,
    Multimedia = 0x04,
    Memory = 0x05,
    BridgeDevice = 0x06,
    SimpleCommunication = 0x07,
    BaseSystemPeripheral = 0x08,
    InputDevice = 0x09,
    DockingStation = 0x0A,
    Processor = 0x0B,
    SerialBus = 0x0C,
    Wireless = 0x0D,
    IntelligentIO = 0x0E,
    SatelliteCommunication = 0x0F,
    EncryptionDecryption = 0x10,
    DataAndSignalProcessing = 0x11,
    Unknown,
}

impl DeviceClass {
    fn from_u8(c: u8) -> DeviceClass {
        if c <= DeviceClass::DataAndSignalProcessing as u8 {
            unsafe { transmute(c) }
        } else {
            DeviceClass::Unknown
        }
    }
}

#[derive(Debug)]
struct DeviceInfo {
    vendor_id: u16,
    device_id: u16,
    revision_id: u8,
    subclass: u8,
    class_code: DeviceClass,
    multifunction: bool,
}

static PCI: Mutex<Pci> = Mutex::new(Pci {
    address: unsafe { io::Port::new(0xCF8) },
    data: unsafe { io::Port::new(0xCFC) },
});

/// Brute-force PCI bus probing.
pub fn dump_devices() {
    let mut pci = PCI.lock();
    println!("Scanning PCI bus");
    unsafe {
        for bus in 0..256 {
            for device in 0..32 {
                if let Some(info) = pci.probe(bus as u8, device, 0) {
                    println!("PCI device {}.{}: {:04x} {:04x} {:?} {:02x}",
                             bus, device,
                             info.vendor_id, info.device_id,
                             info.class_code, info.subclass);
                    if info.multifunction {
                        for function in 1..8 {
                            if let Some(info) =
                                pci.probe(bus as u8, device, function)
                            {
                                println!("  Function {}: {:?} {:02x}",
                                         function,
                                         info.class_code, info.subclass);
                            }
                        }
                    }
                }
            }
        }
    }
}

// Running under QEMU, and checking against http://pcidatabase.com/ , we have:
//
// 0.0: 8086 1237 Intel 82440LX/EX PCI & Memory
// 0.1: 8086 7000 Intel 82371SB PIIX3 PCI-to-ISA Bridge (Triton II)
// 0.2: 1013 00b8 Cirrus Logic CL-GD5446 64-bit VisualMedia Accelerator
// 0.3: 8086 100e Intel 02000 Intel Pro 1000/MT
