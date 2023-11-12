use std::vec::Vec;

pub fn b8zs_encoder(bin_data: &Vec<usize>) -> Vec<i32> {
    let mut encoded_data:Vec<i32> = Vec::new();
    let mut cnt = 0;
    let mut curr_polarity = 1;

    for &bit in bin_data.iter() {
        if bit == 0 {
            cnt += 1;
            if cnt == 8 {
                encoded_data.push(-curr_polarity);
                encoded_data.push(0);
                encoded_data.push(0);
                encoded_data.push(0);
                encoded_data.push(curr_polarity);
                encoded_data.push(0);
                encoded_data.push(0);
                encoded_data.push(0);
                cnt = 0;
            } else {
                encoded_data.push(bit as i32);
            }
        } else {
            cnt = 0;
            curr_polarity = -curr_polarity;
            encoded_data.push(bit as i32);
        }
    }

    encoded_data
}

pub fn hdb3_encoder(bin_data: &Vec<usize>) -> Vec<i32> {
    let mut encoded_data:Vec<i32> = Vec::new();
    let mut cnt = 0;
    let mut curr_polarity = 1;
    let mut even_parity = true;

    for &bit in bin_data.iter() {
        if bit == 0 {
            cnt += 1;
            if cnt == 4 {
                if even_parity {
                    encoded_data.push(-curr_polarity);
                    encoded_data.push(0);
                    encoded_data.push(0);
                    encoded_data.push(curr_polarity);
                } else {
                    encoded_data.push(curr_polarity);
                    even_parity = true;
                }
                cnt = 0;
            } else {
                encoded_data.push(bit as i32);
            }
        } else {
            cnt = 0;
            even_parity = !even_parity;
            curr_polarity = -curr_polarity;
            encoded_data.push(bit as i32);
        }
    }

    encoded_data
}
