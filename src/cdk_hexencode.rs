
static ASCII_CONTROL_CHARS: [&str; 32] = [
    "NULL", "SOH", "STX", "ETX",  "EOT", "ENQ", "ACK", "BEL", 
    "BS",   "HT",  "LF",   "VT",  "FF",  "CR",  "SO",  "SI",   
    "DLE",  "DC1", "DC2",  "DC3", "DC4", "NAK", "SYN", "ETB",    
    "CAN",  "EM",  "SUB",  "ESC", "FS",  "GS",  "RS",  "US"
];

pub fn char_to_printable(value : u8) -> String 
{
    match value
    {
        0..=31 =>  String::from(ASCII_CONTROL_CHARS[value as usize]),
        127    => String::from("DEL"),
        _ => String::from(value as char),
    }
}

pub fn byte_array_to_hex_str(ba : &[u8]) -> String
{
    let mut ret_str = "".to_owned();
    for b in ba {
        let str = format!("{:02X?}", *b);
        // Create &str  
        let x = &str[..];
        ret_str.push_str(x);
    }

    ret_str
}

pub fn print_ascii_table() {

    for row in 0..=15 {
        for col in 0..8 {
            let value = row + (col * 16);
            let print_str = char_to_printable(value).to_string();
            print!("{:<5} {:<5} | ", value, print_str);
        }
        println!();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        print_ascii_table();
    }

    #[test]
    fn test_ba_to_str()
    {
        let data = [0x0, 0x1, 0xe, 0xf, 0xff];
        let s = byte_array_to_hex_str(&data);
        print!("{}", s);
    }
}