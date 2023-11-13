use super::encoding::{Encode, AMI};

pub trait Scramble {
    fn scramble(&self, data: &str, scheme: Scrambling) -> Vec<i8>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scrambling {
    B8ZS,
    HDB3,
    None,
}

impl Scramble for AMI {
    fn scramble(&self, data: &str, scheme: Scrambling) -> Vec<i8> {
        match scheme {
            Scrambling::None => {
                return self.encode(data);
            }
            Scrambling::B8ZS => {
                return b8zs(data);
            }
            Scrambling::HDB3 => {
                return hdb3(data);
            }
        }
    }
}

pub fn b8zs(data: &str) -> Vec<i8> {
    let mut encoded_data = Vec::new();
    let mut count = 0;
    let mut toggle = -1;

    for bit in data.chars().into_iter() {
        match bit {
            '0' => {
                encoded_data.push(0);
                count += 1;
                if count == 8 {
                    count = 0;
                    encoded_data.splice(
                        (encoded_data.len() - 8)..encoded_data.len(),
                        [0, 0, 0, toggle, -toggle, 0, -toggle, toggle],
                    );
                }
            }
            '1' => {
                toggle *= -1;
                encoded_data.push(toggle);
                count = 0;
            }
            _ => {}
        }
    }

    encoded_data
}

pub fn hdb3(data: &str) -> Vec<i8> {
    let mut encoded_data = Vec::new();
    let mut count = 0;
    let mut toggle = -1;
    let mut non_zero_voltage_bool = true;
    
    for bit in data.chars().into_iter() {
        match bit {
            '0' => {
                encoded_data.push(0);
                count += 1;
                if count == 4 {
                    count = 0;
                    let seq;
                    if non_zero_voltage_bool {
                        seq = vec![-toggle, 0, 0, -toggle];
                        toggle *= -1;
                    } else {
                        seq = vec![0, 0, 0, -toggle];
                        non_zero_voltage_bool = !non_zero_voltage_bool;
                    }
                    encoded_data.splice(
                        (encoded_data.len() - 4)..encoded_data.len(),
                        seq,
                    );
                }
            }
            '1' => {
                non_zero_voltage_bool = !non_zero_voltage_bool;
                toggle *= -1;
                encoded_data.push(toggle);
                count = 0;
            }
            _ => {}
        }
    }

    encoded_data
}
