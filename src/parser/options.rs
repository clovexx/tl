mod flags {
    pub const TRACK_IDS: u8 = 1;
    pub const TRACK_CLASSES: u8 = 2;
}

/// Options for the HTML Parser
///
/// This allows users of this library to configure the parser.
/// The default options (`ParserOptions::default()`) are optimized for raw parsing.
/// If you need to do HTML tag lookups by ID or class names, you can enable tracking.
/// This will cache HTML nodes as they appear in the source code on the fly.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ParserOptions(u8);

impl Default for ParserOptions {
    fn default() -> Self {
        Self(0)
    }
}

impl ParserOptions {
    /// Creates a new ParserOptions with no flags set
    pub fn new() -> Self {
        Self::default()
    }

    fn set_flag(&mut self, flag: u8) {
        self.0 |= flag;
    }

    #[inline]
    fn has_flag(&self, flag: u8) -> bool {
        self.0 & flag != 0
    }

    /// Enables tracking of HTML Tag IDs and stores them in a lookup table.
    ///
    /// This makes `get_element_by_id()` lookups ~O(1)
    pub fn track_ids(mut self) -> Self {
        self.set_flag(flags::TRACK_IDS);
        self
    }

    /// Enables tracking of HTML Tag classes and stores them in a lookup table.
    ///
    /// This makes `get_elements_by_class_name()` lookups ~O(1)
    pub fn track_classes(mut self) -> Self {
        self.set_flag(flags::TRACK_CLASSES);
        self
    }

    /// Returns whether the parser is tracking HTML Tag IDs.
    #[inline]
    pub fn is_tracking_ids(&self) -> bool {
        self.has_flag(flags::TRACK_IDS)
    }

    /// Returns whether the parser is tracking HTML Tag classes.
    #[inline]
    pub fn is_tracking_classes(&self) -> bool {
        self.has_flag(flags::TRACK_CLASSES)
    }

    /// Returns whether the parser is tracking HTML Tag IDs or classes (previously enabled by a call to `track_ids()` or `track_classes()`).
    #[inline]
    pub fn is_tracking(&self) -> bool {
        // for now we can just check if any bit is set, may or may not lead to better codegen than two cmps
        // this must be changed in some way if we ever add more flags
        // self.is_tracking_ids() || self.is_tracking_classes()
        self.0 > 0
    }
}
