#![allow(dead_code)]
// Adapted from https://github.com/BurntSushi/memchr/blob/master/src/arch/generic/memchr.rs

use crate::{
    Escapes, Vector,
    ext::Pointer,
    vector::MoveMask,
    writer::{Writer, write, write_slice},
};

/// A generic structure for handling escape sequences in a vectorized manner.
///
/// # Type Parameters
/// - `E`: The escape type implementing the `Escapes` trait.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Generic<E> {
    escapes: E,
}

impl<E> Generic<E>
where
    E: Escapes,
{
    /// The number of bytes processed per iteration in the search loop.
    const LOOP_SIZE: usize = 4 * E::Vector::BYTES;

    /// Creates a new `Generic` instance with the given escape handler.
    ///
    /// # Parameters
    /// - `escapes`: The escape handler to be used.
    #[inline(always)]
    pub(crate) fn new(escapes: E) -> Generic<E> {
        Generic { escapes }
    }

    /// Escapes the input string by applying the escape sequences defined in the `Escapes` trait.
    ///
    /// # Parameters
    /// - `haystack`: The input string to be processed for escape sequences.
    /// - `writer`: The function to write the escaped output.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the escape operation.
    #[inline(always)]
    pub(crate) fn escape<R>(
        &mut self,
        haystack: &str,
        mut writer: impl Writer<R>,
    ) -> Result<(), R> {
        let len = haystack.len();
        let cur = haystack.as_ptr();
        unsafe { self.escape_raw(cur, cur.add(len), &mut writer) }
    }

    /// Escapes the input data between the `start` and `end` pointers.
    ///
    /// # Parameters
    /// - `start`: The starting pointer of the data to be escaped.
    /// - `end`: The ending pointer of the data to be escaped.
    /// - `writer`: The function to write the escaped output.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the escape operation.
    ///
    /// # Safety
    /// This function is unsafe because it operates on raw pointers and assumes
    /// that the memory between `start` and `end` is valid and properly aligned.
    #[inline(always)]
    pub(crate) unsafe fn escape_raw<R>(
        &mut self,
        start: *const u8,
        end: *const u8,
        writer: &mut impl Writer<R>,
    ) -> Result<(), R> {
        unsafe {
            let len = end.distance(start);
            let mut written = start;

            debug_assert!(
                len >= E::Vector::BYTES,
                "haystack has length {}, but must be at least {}",
                len,
                E::Vector::BYTES
            );

            let align = E::Vector::BYTES - (start.to_usize() & E::Vector::ALIGN);
            if align > 0 {
                let x = E::Vector::load_unaligned(start);
                let mask = self.escapes.masking(x).movemask();
                self.write_mask_unaligned(mask, start, align, &mut written, writer)?;
            }

            // Set `cur` to the first V-aligned pointer greater than `start`.
            let mut cur = start.add(align);
            debug_assert!(cur > start && end.sub(E::Vector::BYTES) >= start);

            if len >= Self::LOOP_SIZE {
                while cur <= end.sub(Self::LOOP_SIZE) {
                    debug_assert_eq!(0, cur.to_usize() % E::Vector::BYTES);

                    let a = E::Vector::load_aligned(cur);
                    let b = E::Vector::load_aligned(cur.add(E::Vector::BYTES));
                    let c = E::Vector::load_aligned(cur.add(2 * E::Vector::BYTES));
                    let d = E::Vector::load_aligned(cur.add(3 * E::Vector::BYTES));
                    let eqa = self.escapes.masking(a);
                    let eqb = self.escapes.masking(b);
                    let eqc = self.escapes.masking(c);
                    let eqd = self.escapes.masking(d);
                    let or1 = eqa.or(eqb);
                    let or2 = eqc.or(eqd);
                    let or3 = or1.or(or2);
                    if or3.movemask_will_have_non_zero() {
                        self.write_mask(eqa.movemask(), cur, &mut written, writer)?;
                        self.write_mask(
                            eqb.movemask(),
                            cur.add(E::Vector::BYTES),
                            &mut written,
                            writer,
                        )?;
                        self.write_mask(
                            eqc.movemask(),
                            cur.add(E::Vector::BYTES * 2),
                            &mut written,
                            writer,
                        )?;
                        self.write_mask(
                            eqd.movemask(),
                            cur.add(E::Vector::BYTES * 3),
                            &mut written,
                            writer,
                        )?;
                    }
                    cur = cur.add(Self::LOOP_SIZE);
                }
            }
            // Handle any leftovers after the aligned loop above.
            while cur <= end.sub(E::Vector::BYTES) {
                debug_assert!(end.distance(cur) >= E::Vector::BYTES);
                let v = E::Vector::load_aligned(cur);
                let mask = self.escapes.masking(v).movemask();

                self.write_mask(mask, cur, &mut written, writer)?;
                cur = cur.add(E::Vector::BYTES);
            }

            // Handle any remaining bytes that are less than a full vector's worth.
            if cur < end {
                debug_assert!(end.distance(cur) < E::Vector::BYTES);
                let rest = (E::Vector::BYTES - end.distance(cur)) as u32;
                let start = cur.sub(E::Vector::BYTES - end.distance(cur));
                debug_assert_eq!(end.distance(start), E::Vector::BYTES);
                let x = E::Vector::load_unaligned(start);
                let mask = self.escapes.masking(x).movemask().shr(rest);

                self.write_mask(mask, cur, &mut written, writer)?;
            }

            if written < end {
                write_slice(written, end, writer)?;
            }

            Ok(())
        }
    }

    /// Writes a single step of the escape process, handling any necessary escapes.
    ///
    /// # Parameters
    /// - `mask`: The mask indicating which bytes need to be escaped.
    /// - `cur`: The current pointer in the data.
    /// - `offset`: The offset from the current pointer.
    /// - `written`: A mutable reference to the pointer indicating the last written position.
    /// - `writer`: The function to write the escaped output.
    ///
    /// # Returns
    /// A `Result` containing the updated mask after clearing the least significant bit.
    ///
    /// # Safety
    /// This function is unsafe because it operates on raw pointers and assumes
    /// that the memory is valid.
    #[inline(always)]
    unsafe fn write_step<R>(
        mask: <<E as Escapes>::Vector as Vector>::Mask,
        cur: *const u8,
        offset: usize,
        written: &mut *const u8,
        writer: &mut impl Writer<R>,
    ) -> Result<<<E as Escapes>::Vector as Vector>::Mask, R> {
        unsafe {
            let c = E::position(*cur.add(offset));
            if !E::FALSE_POSITIVE || c < E::ESCAPE_LEN {
                let at = cur.add(offset);
                if *written < at {
                    write_slice(*written, at, writer)?;
                }
                write(E::escape(c), writer)?;
                *written = at.add(1);
            }

            Ok(mask.clear_least_significant_bit())
        }
    }

    /// A helper function to write the escape mask, handling both aligned and unaligned data.
    ///
    /// # Parameters
    /// - `mask`: The mask indicating which bytes need to be escaped.
    /// - `cur`: The current pointer in the data.
    /// - `limit`: The limit up to which the mask should be processed.
    /// - `written`: A mutable reference to the pointer indicating the last written position.
    /// - `writer`: The function to write the escaped output.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the write operation.
    ///
    /// # Safety
    /// This function is unsafe because it operates on raw pointers and assumes
    /// that the memory is valid.
    #[inline(always)]
    unsafe fn write_mask_helper<R>(
        &mut self,
        mut mask: <<E as Escapes>::Vector as Vector>::Mask,
        cur: *const u8,
        limit: usize,
        written: &mut *const u8,
        writer: &mut impl Writer<R>,
    ) -> Result<(), R> {
        unsafe {
            if mask.has_non_zero() {
                let mut offset = mask.first_offset();
                while offset < limit {
                    mask = Self::write_step(mask, cur, offset, written, writer)?;
                    if !mask.has_non_zero() {
                        break;
                    }
                    offset = mask.first_offset();
                }
            }
            Ok(())
        }
    }

    /// Writes the escape mask for unaligned data.
    ///
    /// # Parameters
    /// - `mask`: The mask indicating which bytes need to be escaped.
    /// - `cur`: The current pointer in the data.
    /// - `align`: The alignment offset.
    /// - `written`: A mutable reference to the pointer indicating the last written position.
    /// - `writer`: The function to write the escaped output.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the write operation.
    ///
    /// # Safety
    /// This function is unsafe because it operates on raw pointers and assumes
    /// that the memory is valid.
    #[inline(always)]
    unsafe fn write_mask_unaligned<R>(
        &mut self,
        mask: <<E as Escapes>::Vector as Vector>::Mask,
        cur: *const u8,
        align: usize,
        written: &mut *const u8,
        writer: &mut impl Writer<R>,
    ) -> Result<(), R> {
        unsafe { self.write_mask_helper(mask, cur, align, written, writer) }
    }

    /// Writes the escape mask for aligned data.
    ///
    /// # Parameters
    /// - `mask`: The mask indicating which bytes need to be escaped.
    /// - `cur`: The current pointer in the data.
    /// - `written`: A mutable reference to the pointer indicating the last written position.
    /// - `writer`: The function to write the escaped output.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the write operation.
    ///
    /// # Safety
    /// This function is unsafe because it operates on raw pointers and assumes
    /// that the memory is valid.
    #[inline(always)]
    unsafe fn write_mask<R>(
        &mut self,
        mask: <<E as Escapes>::Vector as Vector>::Mask,
        cur: *const u8,
        written: &mut *const u8,
        writer: &mut impl Writer<R>,
    ) -> Result<(), R> {
        unsafe { self.write_mask_helper(mask, cur, usize::MAX, written, writer) }
    }
}
