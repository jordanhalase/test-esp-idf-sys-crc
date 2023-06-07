use esp_idf_sys::*;

const CRC_INITIAL: u32 = 0xffffffff;

#[inline(always)]
pub fn crc32_be(crc: u32, buf: &[u8]) -> u32 {
    unsafe { esp_rom_crc32_be(crc, buf.as_ptr(), buf.len() as u32) }
}

fn main() -> ! {
    esp_idf_sys::link_patches();

    let crc_check = "123456789";

    loop {
        // Perform CRC-32/MPEG-2
        // width=32 poly=0x04c11db7 init=0xffffffff refin=false refout=false xorout=0x00000000 check=0x0376e6e7 residue=0x00000000 name="CRC-32/MPEG-2"
        let crc = !crc32_be(!CRC_INITIAL, crc_check.as_bytes());

        println!("CRC is {:08x}", crc);
        assert_eq!(crc, 0x0376e6e7);

        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
