sed -i 's/buffer: &mut \[u8\]/_buffer: \&mut \[u8\]/g' src/driver/windows_compat.rs
sed -i 's/buffer: &\[u8\]/_buffer: \&\[u8\]/g' src/driver/windows_compat.rs
sed -i 's/device: &mut DEVICE_OBJECT/_device: \&mut DEVICE_OBJECT/g' src/driver/windows_compat.rs
sed -i 's/offset: u32/_offset: u32/g' src/drivers/e1000_nic.rs
sed -i 's/value: u32/_value: u32/g' src/drivers/e1000_nic.rs
sed -i 's/offset: usize/_offset: usize/g' src/drivers/nvme_storage.rs
sed -i 's/value: u32/_value: u32/g' src/drivers/nvme_storage.rs
sed -i 's/value: u64/_value: u64/g' src/drivers/nvme_storage.rs
sed -i 's/offset: u16/_offset: u16/g' src/drivers/intel_hda.rs
sed -i 's/value: u32/_value: u32/g' src/drivers/intel_hda.rs
sed -i 's/value: u16/_value: u16/g' src/drivers/intel_hda.rs
sed -i 's/value: u8/_value: u8/g' src/drivers/intel_hda.rs
sed -i 's/file_path: &str/_file_path: \&str/g' src/compatibility/historic_linux.rs
sed -i 's/data_len: usize/_data_len: usize/g' src/compatibility/historic_linux.rs
sed -i 's/username: &str/_username: \&str/g' src/security/root_improvement.rs
sed -i 's/password_hash: &str/_password_hash: \&str/g' src/security/root_improvement.rs
sed -i 's/args: &\[u64\]/_args: \&\[u64\]/g' src/kernel/gap_closing.rs
sed -i 's/coproc: u8, opcode1: u8, value: u32, cr_n: u8, cr_m: u8, opcode2: u8/_coproc: u8, _opcode1: u8, _value: u32, _cr_n: u8, _cr_m: u8, _opcode2: u8/g' src/kernel/cpu_features.rs
sed -i 's/let key = vec!\[1, 2, 3, 4, 5\];/let key: Vec<u8> = (1u8..=5u8).collect();/g' src/security/password.rs
sed -i 's/let seed: u64 = 0x5BD1E9955C3A7B2D; \/\/ Test seed constant/let seed: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64;/g' src/driver/distro_drivers.rs
