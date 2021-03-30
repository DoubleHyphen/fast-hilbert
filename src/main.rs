// The LUT, copied from your code:
const LUT_3: [u8; 256] = [
    64, 1, 206, 79, 16, 211, 84, 21, 131, 2, 205, 140, 81, 82, 151, 22, 4, 199, 8, 203, 158, 157,
    88, 25, 69, 70, 73, 74, 31, 220, 155, 26, 186, 185, 182, 181, 32, 227, 100, 37, 59, 248, 55,
    244, 97, 98, 167, 38, 124, 61, 242, 115, 174, 173, 104, 41, 191, 62, 241, 176, 47, 236, 171,
    42, 0, 195, 68, 5, 250, 123, 60, 255, 65, 66, 135, 6, 249, 184, 125, 126, 142, 141, 72, 9, 246,
    119, 178, 177, 15, 204, 139, 10, 245, 180, 51, 240, 80, 17, 222, 95, 96, 33, 238, 111, 147, 18,
    221, 156, 163, 34, 237, 172, 20, 215, 24, 219, 36, 231, 40, 235, 85, 86, 89, 90, 101, 102, 105,
    106, 170, 169, 166, 165, 154, 153, 150, 149, 43, 232, 39, 228, 27, 216, 23, 212, 108, 45, 226,
    99, 92, 29, 210, 83, 175, 46, 225, 160, 159, 30, 209, 144, 48, 243, 116, 53, 202, 75, 12, 207,
    113, 114, 183, 54, 201, 136, 77, 78, 190, 189, 120, 57, 198, 71, 130, 129, 63, 252, 187, 58,
    197, 132, 3, 192, 234, 107, 44, 239, 112, 49, 254, 127, 233, 168, 109, 110, 179, 50, 253, 188,
    230, 103, 162, 161, 52, 247, 56, 251, 229, 164, 35, 224, 117, 118, 121, 122, 218, 91, 28, 223,
    138, 137, 134, 133, 217, 152, 93, 94, 11, 200, 7, 196, 214, 87, 146, 145, 76, 13, 194, 67, 213,
    148, 19, 208, 143, 14, 193, 128,
];

fn get_6_key_bits(x: u8) -> u8 {
    LUT_3[x as usize] & 63
}

fn get_state(x: u8) -> u8 {
    LUT_3[x as usize]>>6
}

// This is DEEP MAGIC. Took me a full day to figure out.
// This works for your own code, but not for mine.
// Mine is easier to explain and reason about; please find it
// at the end of this file.
// 
// Here is the explanation, pretty much:
// 1. Split the input into groups of 2 bits
// 2. For each one of those, if it is a 10, turn it into a 01
// 3. XOR all those couples of bits together
// 4. Flip the last bit.
fn state_after_the_first_byte_of_a_larger_2d_hilbert_key(x: u8) -> u8 {
    let a = x & 0x55;
    let b = x & 0xAA;
    let mut c = a | (b >> 1);
    c |= (a << 1) & b;
    c ^= c >> 4;
    c ^= c >> 2;
    (c & 3) ^ 1
}

fn new_state (x: u8) -> u8 {
    state_after_the_first_byte_of_a_larger_2d_hilbert_key(x)
    // A long name for clarity, and a short name for usability.
}

fn main() {
    for i in 0u8..64 {
        // To get the next 6 bits of the coordinates without
        // using the state as part of the index,
        // we need to do the following:
        
        // If the state is 0, the LUT obviously can be used as-is.
        assert_eq!(get_6_key_bits(i), get_6_key_bits(i));
        
        // If the state is 3, we index the LUT using the negation
        // of the index:
        assert_eq!(get_6_key_bits(i + 192), get_6_key_bits(i ^ 63));
        
        // If the state is 1, we swap the x and y coordinates:
        let other_i = (i >> 3) | (i << 3) & 63;
        assert_eq!(get_6_key_bits(i + 64), get_6_key_bits(other_i));
        
        // If the state is 2, we do both:
        assert_eq!(get_6_key_bits(i + 128), get_6_key_bits(other_i ^ 63));
        
        
        // And now, let us exhibit the calculation of the new states!
        // Step 1: Calculate the old state.
        // Step 2: Calculate the new bits for the Hilbert key.
        // Step 3: Calculate the state that would appear if the new bits
        // were in the beginning of the key.
        // Step 4: XOR those 2 states together.
        // Step 5: Done!
        
        let assert_state_function_is_correct_for_this_index = |j| {
            let result = get_6_key_bits(j);
            let old_state = j>>6;
            assert_eq!(get_state(j), new_state(result) ^ old_state);
        };
        
        assert_state_function_is_correct_for_this_index(i + 000);
        assert_state_function_is_correct_for_this_index(i + 064);
        assert_state_function_is_correct_for_this_index(i + 128);
        assert_state_function_is_correct_for_this_index(i + 192);
    }
    
    println!("All done!");
} 


// This is DEEP MAGIC. Took me a full day to figure out.
// Basically works as follows:
// 1. Split the new Hilbert Key byte you calculated into groups of two bits
// 2. For each group, add the two bits to one another
// 3. XOR all the results together
// This is implemented a bit more efficiently than it is described:
// If the two bits are XY, then X+Y == XY - X.
fn _the_new_state_that_works_for_my_code(x: u8) -> u8 {
        let b = x & 0xAA; // Extract the odd-significance bits…
        let mut c = x - (b>>1); // Shift them and subtract…
        c ^= c >> 4; //
        c ^= c >> 2; //
        c & 3        // And, as previously, XOR and mask.
}
