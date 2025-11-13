const MAGIC: u64 = 8318619103235151522;
const MAGIC_BITS: usize = 22;
const MAGIC_SHIFT: usize = 64 - MAGIC_BITS;

fn main() {
    let mut results = vec![0; 2_usize.pow(MAGIC_BITS as u32)];
    for x in 1..=30 {
        for y in 1..=30 {
            for z in 1..=30 {
                let line = format!("{x}x{y}x{z}");

                let mut dims = line
                    .split_terminator('x')
                    .map(|d| d.parse::<u32>().unwrap());
                let l = dims.next().unwrap();
                let w = dims.next().unwrap();
                let h = dims.next().unwrap();
                let smallest = (l * w).min(w * h).min(l * h);
                let result = 2 * l * w + 2 * w * h + 2 * h * l + smallest;

                let mut bytes = [0; 8];
                bytes[..line.len()].copy_from_slice(line.as_bytes());
                let hash = (u64::from_ne_bytes(bytes).wrapping_mul(MAGIC)) >> MAGIC_SHIFT;

                results[hash as usize] = result;
            }
        }
    }

    let bytes: Vec<u8> = results.iter().flat_map(|v| v.to_ne_bytes()).collect();
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("LUT_2015d2p1.bin");
    std::fs::write(&dest_path, bytes).unwrap();
}
