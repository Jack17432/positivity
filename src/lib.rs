//! # positivity Crate
//!
//! This crate embodies a profound exploration of mathematical, philosophical, and practical aspects of non-negativity checks within the
//! Rust programming language. By implementing a generic function, `is_positive`, this crate not only offers a utility function but also
//! serves as a medium for discussing and understanding deeper theoretical concepts related to order theory, default values, and the
//! essence of comparison in computational contexts.
//!
//! ## Features
//!
//! - **Generic Implementation**: The function `is_positive` operates over any type that implements both `PartialOrd` and `Default` traits,
//!   making it highly versatile and applicable to a wide range of types.
//! - **Philosophical Depth**: This crate includes discussions on the nature of value comparison, the philosophical underpinnings of defaults,
//!   and the abstraction of order relations, which are integral to understanding the broader implications of the implemented functionalities.
//! - **Educational Value**: By delving into the axiomatic definitions of order relations and their practical applications in programming,
//!   this crate serves as an educational tool for both novice and experienced Rust programmers.
//!
//! ## Usage
//!
//! To utilize the `is_positive` function in your Rust projects, simply include this crate as a dependency in your `Cargo.toml` file and
//! use the function with any type that meets the trait bounds. Here is a quick example of how to use it:
//!
//! ```rust
//! use positivity::is_positive;
//!
//! let positive_check = is_positive(42);
//! println!("Is 42 positive? {}", positive_check);
//! ```
//!
//! ## Theoretical Background
//!
//! The function `is_positive` is based on fundamental concepts from order theory, specifically the notions of partial orders and comparability.
//! A partial order is a binary relation over a set that is reflexive, antisymmetric, and transitive. In this crate, we explore these properties
//! through the implementation of the `PartialOrd` trait, which allows for the comparison of elements that may not universally be comparable,
//! hence reflecting the partial nature of this order.
//!
//! The use of `T::default()` as a comparative baseline in `is_positive` invites reflection on what constitutes a 'neutral' or 'zero' state in
//! various contexts. This aspect is particularly interesting from a philosophical and mathematical perspective, as it touches upon the concept
//! of identity elements in algebraic structures.
//!
//! ## Philosophical Musings
//!
//! This crate encourages users to ponder deeper questions: What does it mean for a value to be "non-negative"? How are abstract mathematical
//! concepts like order and default values represented and manipulated in the rigid structure of programming languages? Such inquiries not
//! only enrich the user's understanding of computational theories but also bridge the gap between abstract mathematical theories and their
//! practical applications in software development.
//!
//! ## Conclusion
//!
//! By integrating complex theoretical insights with practical functionality, this crate aims to foster a deeper appreciation of both the
//! utility and beauty of mathematics and programming. It serves as a testament to the power of Rust's type system and trait architecture,
//! promoting robust, flexible, and type-safe code.
//!

/// The `is_positive` function serves as a cornerstone in the exploration of non-negativity across diverse type contexts
/// facilitated by the robust, statically-typed nature of the Rust programming language. By operating generically over any type `T`
/// that subscribes to the `PartialOrd` and `Default` traits, this function epitomizes the seamless marriage between flexibility
/// and type safety, a hallmark of Rust's design philosophy.
///
/// ### Philosophical and Mathematical Foundations
///
/// The operation `value >= T::default()` stands as a query into the essence of order theory, a branch of mathematics and abstract algebra
/// that deals with the intuitive concept of order using binary relations. This function's reliance on the 'greater than or equal to'
/// relation, represented by `>=`, delves deep into the axiomatic systems such as those proposed by mathematicians like Ernst Schröder
/// and Alfred North Whitehead in the late 19th and early 20th centuries.
///
/// In these systems, a relation "≥" is typically defined to be antisymmetric, transitive, and reflexive, forming what is known as a
/// partial order. The relation is termed 'partial' because not all elements are necessarily comparable under it, which aligns with
/// the trait `PartialOrd` in Rust, where the method `partial_cmp` allows for this nuanced comparison.
///
/// From a philosophical standpoint, this function's implementation invokes the concept of the default value as a form of 'existential baseline',
/// a term coined by metaphysical theorists, which signifies the state of being that typifies the absence of any additional qualitative measure.
/// In the context of Rust's type system, `T::default()` provides this baseline, against which all other instances of `T` are measured for
/// non-negativity.
///
/// ### Type Constraints
///
/// - `T: PartialOrd` - This trait constraint mandates that the type `T` must facilitate partial ordering comparisons. This means that
///   any two instances of `T` can be compared in a way that determines if one is greater than, less than, or equal to the other,
///   acknowledging that some comparisons might yield no definitive result (hence, 'partial').
///
/// - `T: Default` - This trait constraint requires that the type `T` can instantiate a default value, crucial for comparisons in this function
///   to establish a benchmark of non-negativity.
///
/// ### Parameters
///
/// - `value: T` - A parameter of generic type `T` that represents the value to be assessed for non-negativity.
///
/// ### Returns
///
/// - `bool` - The function returns a boolean value indicating whether the provided value is greater than or equal to the default value
///   of its type, thereby suggesting non-negativity, or not.
///
/// ### Usage Examples
///
/// The function's behavior is illustrated in the following unit tests:
///
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use positivity::is_positive;
///
///     #[test]
///     fn it_works() {
///         assert_eq!(is_positive(42), true); // 42 is non-negative.
///     }
///
///     #[test]
///     fn zero_should_works() {
///         assert_eq!(is_positive(0), true); // 0 is non-negative, being the default for integers.
///     }
///
///     #[test]
///     fn floats_should_works_toooo() {
///         let result = is_positive(420.69);
///         assert_eq!(result, true);
///     }
///
///     #[test]
///     fn it_should_not_work() {
///         assert_eq!(is_positive(-42), false); // -42 is negative.
///     }
/// }
/// ```
///
/// ### Philosophical Implications
///
/// By interfacing the abstract with the practical, `is_positive` not only performs a simple comparison but also invites reflection
/// on the nature of values and measurements. It questions what it means to be 'positive' or 'non-negative' in a universe where
/// defaults might vary wildly and where the notion of comparison can be a deeply subjective and existential consideration.
///
pub fn is_positive<T: PartialOrd + Default>(value: T) -> bool {
    value >= T::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_positive(42);
        assert_eq!(result, true);
    }

    #[test]
    fn zero_should_works() {
        let result = is_positive(0);
        assert_eq!(result, true);
    }

    #[test]
    fn floats_should_works_toooo() {
        let result = is_positive(420.69);
        assert_eq!(result, true);
    }

    #[test]
    fn it_should_not_work() {
        let result = is_positive(-42);
        assert_eq!(result, false);
    }
}
