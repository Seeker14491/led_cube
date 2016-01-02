extern crate serial;

use std::ffi::OsStr;
use std::io;
use std::io::Write;

/// A type representing a position on the cube, of the form [`x`, `y`, `z`].
pub type CubePosition = [usize; 3];

/// A connection to an LED cube.
///
/// The methods on this struct operate on an internal buffer, and won't have an effect on the cube
/// until the `flush` method is called.
///
/// # Examples
///
/// Turn each light on in sequence:
///
/// ```no_run
/// use std::thread;
/// use std::time::Duration;
/// use led_cube::*;
///
/// let mut cube = Cube::new("COM5").unwrap();
///
/// for z in 0..4 {
///     for y in 0..4 {
///         for x in 0..4 {
///             cube.clear();
///             cube.set([x, y, z], true);
///             cube.flush();
///             thread::sleep(Duration::from_millis(500));
///         }
///     }
/// }
/// ```
pub struct Cube {
    state: [u8; 16],
    port: serial::SystemPort,
}

impl Cube {
    /// Connect to an LED cube through a serial port.
    pub fn new<T: AsRef<OsStr> + ?Sized>(port: &T) -> io::Result<Self> {
        let mut cube = Cube {
            state: [0b0000, 0b0000, 0b0000, 0b0000,
                    0b0000, 0b0000, 0b0000, 0b0000,
                    0b0000, 0b0000, 0b0000, 0b0000,
                    0b0000, 0b0000, 0b0000, 0b0000],
            port: serial::open(port).unwrap(),
        };
        try!(cube.flush());
        Result::Ok(cube)
    }

    /// Update the LED cube to match the internal buffer.
    pub fn flush(&mut self) -> io::Result<()>{
        self.port.write(&self.state).map(|_| {})
    }

    /// Turn the LED at position `pos` on or off based on `state`.
    ///
    /// # Panics
    /// Panics if `pos` is out of range.
    pub fn set(&mut self, pos: CubePosition, state: bool) {
        check_bounds(pos);
        let pattern_idx = 4 * invert4(pos[1]) + invert4(pos[2]);
        let mask = 1 << invert4(pos[0]);
        if state {
            self.state[pattern_idx] |= mask;
        } else {
            self.state[pattern_idx] &= !mask;
        }
    }

    /// Returns `true` if the LED at position `pos` is on.
    ///
    /// # Panics
    /// Panics if `pos` is out of range.
    pub fn get(&self, pos: CubePosition) -> bool {
        check_bounds(pos);
        self.state[4 * invert4(pos[1]) + invert4(pos[2])] & (1 << invert4(pos[0])) != 0
    }

    /// Turns off all LEDs.
    pub fn clear(&mut self) {
        self.state = [0b0000, 0b0000, 0b0000, 0b0000,
                      0b0000, 0b0000, 0b0000, 0b0000,
                      0b0000, 0b0000, 0b0000, 0b0000,
                      0b0000, 0b0000, 0b0000, 0b0000];
    }
}

fn check_bounds(pos: CubePosition) {
    assert!(pos[0] < 4 && pos[1] < 4 && pos[2] < 4);
}

fn invert4(n: usize) -> usize {
    [3, 2, 1, 0][n]
}
