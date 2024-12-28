/// Represents a pair of attribute name and value.
///
/// This struct is used to hold a pair of attribute name and value. The
/// attribute name is a reference to a string slice, and the attribute value is
/// a `Cow` (Copy-on-Write) reference to a string. This allows for efficient
/// handling of both borrowed and owned strings.
///
/// # Fields
///
/// * `0`: The attribute name as a reference to a string slice.
/// * `1`: The attribute value as a `Cow` reference to a string.
pub struct AttrPair<'a>(pub(crate) &'a str, pub(crate) std::borrow::Cow<'a, str>);

/// A collection of attribute pairs.
///
/// This type alias represents a vector of `AttrPair` instances, which can be
/// used to collect and manage multiple attribute pairs.
pub type AttrCollection<'a> = Vec<AttrPair<'a>>;

impl<'a> From<(&'a str, &'a str)> for AttrPair<'a> {
    /// Converts a tuple of two string slices into an `AttrPair`.
    ///
    /// This method takes a tuple of two string slices and returns an `AttrPair`
    /// instance. The first element of the tuple becomes the attribute name, and
    /// the second element becomes the attribute value as a borrowed string.
    ///
    /// # Example
    ///
    /// ```
    /// let attr_pair = AttrPair::from(("name", "value"));
    /// ```
    fn from(tuple: (&'a str, &'a str)) -> Self {
        AttrPair(tuple.0, std::borrow::Cow::Borrowed(tuple.1))
    }
}

impl<'a> From<(&'a str, String)> for AttrPair<'a> {
    /// Converts a tuple of a string slice and a `String` into an `AttrPair`.
    ///
    /// This method takes a tuple of a string slice and a `String` and returns
    /// an `AttrPair` instance. The string slice becomes the attribute name, and
    /// the `String` becomes the attribute value as an owned string.
    ///
    /// # Example
    ///
    /// ```
    /// let attr_pair = AttrPair::from(("name", String::from("value")));
    /// ```
    fn from(tuple: (&'a str, String)) -> Self {
        AttrPair(tuple.0, std::borrow::Cow::Owned(tuple.1))
    }
}

impl<'a> From<(&'a str, &String)> for AttrPair<'a> {
    /// Converts a tuple of a string slice and a reference to a `String` into an
    /// `AttrPair`.
    ///
    /// This method takes a tuple of a string slice and a reference to a
    /// `String` and returns an `AttrPair` instance. The string slice becomes
    /// the attribute name, and the `String` becomes the attribute value as an
    /// owned string.
    ///
    /// # Example
    ///
    /// ```
    /// let string = String::from("value");
    /// let attr_pair = AttrPair::from(("name", &string));
    /// ```
    fn from(tuple: (&'a str, &String)) -> Self {
        AttrPair(tuple.0, std::borrow::Cow::Owned(tuple.1.to_owned()))
    }
}

impl<'a> From<(&'a str, Box<str>)> for AttrPair<'a> {
    /// Converts a tuple of a string slice and a `Box` of a string slice into an
    /// `AttrPair`.
    ///
    /// This method takes a tuple of a string slice and a `Box` of a string
    /// slice and returns an `AttrPair` instance. The string slice becomes the
    /// attribute name, and the `Box` of a string slice becomes the attribute
    /// value as an owned string.
    ///
    /// # Example
    ///
    /// ```
    /// let box_str = Box::new("value");
    /// let attr_pair = AttrPair::from(("name", box_str));
    /// ```
    fn from(tuple: (&'a str, Box<str>)) -> Self {
        AttrPair(tuple.0, std::borrow::Cow::Owned(tuple.1.into_string()))
    }
}

impl<'a> From<(&'a str, &Box<str>)> for AttrPair<'a> {
    /// Converts a tuple of a string slice and a reference to a `Box` of a
    /// string slice into an `AttrPair`.
    ///
    /// This method takes a tuple of a string slice and a reference to a `Box`
    /// of a string slice and returns an `AttrPair` instance. The string slice
    /// becomes the attribute name, and the `Box` of a string slice becomes the
    /// attribute value as an owned string.
    ///
    /// # Example
    ///
    /// ```
    /// let box_str = Box::new("value");
    /// let attr_pair = AttrPair::from(("name", &box_str));
    /// ```
    fn from(tuple: (&'a str, &Box<str>)) -> Self {
        AttrPair(
            tuple.0,
            std::borrow::Cow::Owned(tuple.1.clone().into_string()),
        )
    }
}

impl<'a> From<AttrPair<'a>> for (&'a str, std::borrow::Cow<'a, str>) {
    /// Converts an `AttrPair` into a tuple of a string slice and a `Cow` of a
    /// string slice.
    ///
    /// This method takes an `AttrPair` and returns a tuple of a string slice
    /// and a `Cow` of a string slice. The attribute name becomes the string
    /// slice, and the attribute value becomes the `Cow` of a string slice.
    ///
    /// # Example
    ///
    /// ```
    /// let attr_pair = AttrPair::from(("name", "value"));
    /// let tuple = <(&str, Cow<str>)>::from(attr_pair);
    /// ```
    fn from(attr_pair: AttrPair<'a>) -> Self {
        (attr_pair.0, attr_pair.1)
    }
}
