use hidapi::HidDevice;
use std::{ thread, time::Duration };

pub fn get(hid_device: &HidDevice, wired: bool) {
    let mut bfr_w = [0u8; 65];

    if wired {
        bfr_w[3] = 0x02;
    }
    
    bfr_w[4] = 0x03;
    bfr_w[6] = 0x81;

    hid_device
        .send_feature_report(&bfr_w)
        .unwrap(); // TODO: error handling

    thread::sleep(Duration::from_millis(50));

    let mut bfr_r = [0u8; 65];

    hid_device
        .get_feature_report(&mut bfr_r)
        .unwrap(); // TODO: error handling

    println!(
        "{}.{}.{}.{}",
        bfr_r[7],
        bfr_r[8],
        bfr_r[9],
        bfr_r[10],
    );
}