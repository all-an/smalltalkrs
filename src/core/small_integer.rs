//! SmallInteger implementation for Smalltalk
//! 
//! SmallInteger represents integer values in the Smalltalk system.
//! In traditional Smalltalk, SmallIntegers are immediate values (not heap objects)
//! for performance, but this implementation treats them as regular objects.

use super::object::{ObjectId, SmalltalkObject};

/// SmallInteger represents integer values in Smalltalk
/// 
/// SmallIntegers support basic arithmetic operations and comparisons.
/// They are immutable objects that represent integer values.
#[derive(Debug, Clone)]
pub struct SmallInteger {
    id: ObjectId,
    value: i64,
}

impl SmallInteger {
    /// Creates a new SmallInteger with the given value
    /// 
    /// # Arguments
    /// * `value` - The integer value to wrap
    /// 
    /// # Returns
    /// A new SmallInteger object
    /// 
    /// # Examples
    /// ```
    /// use smalltalkrs::core::SmallInteger;
    /// let num = SmallInteger::new(42);
    /// assert_eq!(num.value(), 42);
    /// ```
    pub fn new(value: i64) -> Self {
        Self {
            id: ObjectId::new(),
            value,
        }
    }
    
    /// Returns the integer value
    /// 
    /// # Returns
    /// The wrapped i64 value
    pub fn value(&self) -> i64 {
        self.value
    }
    
    /// Adds another SmallInteger to this one
    /// 
    /// Equivalent to Smalltalk's `+` message.
    /// 
    /// # Arguments
    /// * `other` - The SmallInteger to add
    /// 
    /// # Returns
    /// A new SmallInteger containing the sum
    /// 
    /// # Examples
    /// ```
    /// use smalltalkrs::core::SmallInteger;
    /// let a = SmallInteger::new(3);
    /// let b = SmallInteger::new(4);
    /// let result = a.add(&b);
    /// assert_eq!(result.value(), 7);
    /// ```
    pub fn add(&self, other: &SmallInteger) -> SmallInteger {
        SmallInteger::new(self.value + other.value)
    }
    
    /// Subtracts another SmallInteger from this one
    /// 
    /// Equivalent to Smalltalk's `-` message.
    /// 
    /// # Arguments
    /// * `other` - The SmallInteger to subtract
    /// 
    /// # Returns
    /// A new SmallInteger containing the difference
    /// 
    /// # Examples
    /// ```
    /// use smalltalkrs::core::SmallInteger;
    /// let a = SmallInteger::new(10);
    /// let b = SmallInteger::new(3);
    /// let result = a.subtract(&b);
    /// assert_eq!(result.value(), 7);
    /// ```
    pub fn subtract(&self, other: &SmallInteger) -> SmallInteger {
        SmallInteger::new(self.value - other.value)
    }
    
    /// Tests if this integer is less than another
    /// 
    /// Equivalent to Smalltalk's `<` message.
    /// 
    /// # Arguments
    /// * `other` - The SmallInteger to compare with
    /// 
    /// # Returns
    /// True if this integer is less than the other
    /// 
    /// # Examples
    /// ```
    /// use smalltalkrs::core::SmallInteger;
    /// let a = SmallInteger::new(3);
    /// let b = SmallInteger::new(5);
    /// assert!(a.less_than(&b));
    /// assert!(!b.less_than(&a));
    /// ```
    pub fn less_than(&self, other: &SmallInteger) -> bool {
        self.value < other.value
    }
}

impl SmalltalkObject for SmallInteger {
    fn object_id(&self) -> ObjectId {
        self.id
    }
    
    fn equals(&self, other: &dyn SmalltalkObject) -> bool {
        if let Some(other_int) = other.as_any().downcast_ref::<SmallInteger>() {
            self.value == other_int.value
        } else {
            false
        }
    }
    
    fn to_smalltalk_string(&self) -> String {
        self.value.to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_small_integer_creation() {
        let num = SmallInteger::new(42);
        assert_eq!(num.value(), 42);
    }
    
    #[test]
    fn test_small_integer_unique_ids() {
        let num1 = SmallInteger::new(42);
        let num2 = SmallInteger::new(42);
        
        // Same values but different object identities
        assert_ne!(num1.object_id(), num2.object_id());
    }
    
    #[test]
    fn test_small_integer_addition() {
        let a = SmallInteger::new(3);
        let b = SmallInteger::new(4);
        let result = a.add(&b);
        
        assert_eq!(result.value(), 7);
    }
    
    #[test]
    fn test_small_integer_subtraction() {
        let a = SmallInteger::new(10);
        let b = SmallInteger::new(3);
        let result = a.subtract(&b);
        
        assert_eq!(result.value(), 7);
    }
    
    #[test]
    fn test_small_integer_subtraction_negative() {
        let a = SmallInteger::new(3);
        let b = SmallInteger::new(10);
        let result = a.subtract(&b);
        
        assert_eq!(result.value(), -7);
    }
    
    #[test]
    fn test_small_integer_less_than() {
        let a = SmallInteger::new(3);
        let b = SmallInteger::new(5);
        let c = SmallInteger::new(3);
        
        assert!(a.less_than(&b));
        assert!(!b.less_than(&a));
        assert!(!a.less_than(&c)); // Equal values
    }
    
    #[test]
    fn test_small_integer_equality() {
        let a = SmallInteger::new(42);
        let b = SmallInteger::new(42);
        let c = SmallInteger::new(24);
        
        // Same values are equal
        assert!(a.equals(&b));
        assert!(b.equals(&a));
        
        // Different values are not equal
        assert!(!a.equals(&c));
        
        // Self equality
        assert!(a.equals(&a));
    }
    
    #[test]
    fn test_small_integer_identity_vs_equality() {
        let a = SmallInteger::new(42);
        let b = SmallInteger::new(42);
        
        // Equal but not identical (different objects)
        assert!(a.equals(&b));
        assert!(!a.is_identical(&b));
        
        // Identical to self
        assert!(a.is_identical(&a));
    }
    
    #[test]
    fn test_small_integer_to_string() {
        let num = SmallInteger::new(42);
        assert_eq!(num.to_smalltalk_string(), "42");
        
        let negative = SmallInteger::new(-17);
        assert_eq!(negative.to_smalltalk_string(), "-17");
        
        let zero = SmallInteger::new(0);
        assert_eq!(zero.to_smalltalk_string(), "0");
    }
    
    #[test]
    fn test_operations_dont_mutate_original() {
        let original = SmallInteger::new(5);
        let other = SmallInteger::new(3);
        
        let sum = original.add(&other);
        let diff = original.subtract(&other);
        
        // Original value unchanged
        assert_eq!(original.value(), 5);
        assert_eq!(other.value(), 3);
        
        // New objects created
        assert_eq!(sum.value(), 8);
        assert_eq!(diff.value(), 2);
    }
    
    #[test]
    fn test_equals_with_non_smallinteger() {
        // Test line 125 - equality check with non-SmallInteger object
        use super::super::object::{ObjectId, SmalltalkObject};
        
        #[derive(Debug)]
        struct NotAnInteger {
            id: ObjectId,
        }
        
        impl NotAnInteger {
            fn new() -> Self {
                Self {
                    id: ObjectId::new(),
                }
            }
        }
        
        impl SmalltalkObject for NotAnInteger {
            fn object_id(&self) -> ObjectId {
                self.id
            }
        }
        
        let num = SmallInteger::new(42);
        let not_num = NotAnInteger::new();
        
        // SmallInteger should not equal non-SmallInteger objects
        // This tests the "else false" branch on line 125
        assert!(!num.equals(&not_num));
    }
}