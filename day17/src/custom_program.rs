#[allow(unused)]
const EXPECTED_OUTPUT: [usize; 16] = [2, 4, 1, 1, 7, 5, 4, 4, 1, 4, 0, 3, 5, 5, 3, 0];

#[allow(unused)]
pub(crate) fn run(initial_a: usize) -> bool {
    let mut a = initial_a;
    let mut b = 0;
    let mut c;
    let mut output_index = 0;
    let mut output = vec![];

    loop {
        // only keep the last 3 bits
        b %= 8;
        b ^= 0b001;
        // drop 0-7 of the last bits
        c = a >> b;
        b ^= c;
        b ^= 0b010;
        // drop the last 3 bits
        a >>= 3;

        if EXPECTED_OUTPUT[output_index] != b % 8 {
            if output_index > 4 {
                println!("{initial_a:b} got to: {output:?}");
            }
            return false;
        }

        output.push(b % 8);
        output_index += 1;

        if a == 0 {
            return output_index == EXPECTED_OUTPUT.len();
        }
    }
}

#[allow(unused)]
pub(crate) fn run_for_output(a: usize) -> Vec<usize> {
    let mut a = a;
    let mut b = 0;
    let mut c;
    let mut output = vec![];

    loop {
        // only keep the last 3 bits
        b %= 8;
        b ^= 0b001;
        // drop 0-7 of the last bits
        c = a >> b;
        b ^= c;
        b ^= 0b010;
        // drop the last 3 bits
        a >>= 3;

        output.push(b % 8);

        if a == 0 {
            return output;
        }
    }
}

#[allow(unused)]
pub(crate) fn find_target_input(i: usize) -> Option<usize> {
    let output = run_for_output(i);
    if output == EXPECTED_OUTPUT {
        return Some(i);
    }
    if !EXPECTED_OUTPUT.ends_with(&output) {
        return None;
    }

    (0b000..=0b111).find_map(|suffix| {
        let next = (i << 3) | suffix; // concats the suffix to i
        find_target_input(next)
    })
}
