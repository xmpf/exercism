#![feature(type_ascription)]

/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    create_buffer(0)
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    // return a vector of size `count` initialized with `0`
    vec![0u8; count]: Vec<u8>
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut fibs: Vec<u8> = create_buffer(5);
    
    fibs[0] = 1;
    fibs[1] = 1;
    
    for i in 2..5 {
        fibs[i] = fibs[i-1] + fibs[i-2];
    }
    
    fibs
}
