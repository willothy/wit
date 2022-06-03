pub fn hex(vec: &Vec<u8>) -> String {
    let mut sha = 0;
    let mut buf: [u8; 4] = [0, 0, 0, 0];
    vec.iter().enumerate().for_each(|(i, x)| {
        if i % 4 == 0 {
            // https://stackoverflow.com/questions/36669427/does-rust-have-a-way-to-convert-several-bytes-to-a-number
            sha +=
            ((buf[0] as u32) << 24) +
            ((buf[1] as u32) << 16) +
            ((buf[2] as u32) <<  8) +
            ((buf[3] as u32) <<  0);
        }
        buf[i % 4] = *x;
    });
    format!("{:x}", &sha)
}